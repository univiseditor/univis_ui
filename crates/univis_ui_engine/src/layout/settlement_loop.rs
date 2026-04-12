use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::layout::components::CachedUiContext;
use crate::layout::core::layout_cache::{
    LayoutCache, collect_layout_children, layout_children_changed,
};
use crate::layout::image::UImage;
#[allow(deprecated)]
use crate::layout::layout_system::{
    ResolvedRootUi, URootUi, UScreenRoot, UWorldRoot, UiRootSettlementState,
};
use crate::layout::pbr::UPbr;
use crate::layout::profiling::LayoutProfiler;
use crate::layout::univis_node::{UBorder, UClip, ULayout, UNode, USelf};
use crate::schedule::{
    UiPendingStages, UiSettlementConfig, UiSettlementSchedule, UiWorkStage, UiWorkState,
};

pub(super) fn run_ui_settlement_loop(world: &mut World) {
    let max_iterations = world.resource::<UiSettlementConfig>().max_iterations.max(1);

    world.resource_mut::<UiWorkState>().begin_frame();
    if let Some(mut profiler) = world.get_resource_mut::<LayoutProfiler>() {
        profiler.begin_frame();
    }

    let mut exhausted = true;
    for _ in 0..max_iterations {
        let had_pending_before = world.resource::<UiWorkState>().pending().any();
        let generation_before = world.resource::<UiWorkState>().current_generation();

        world.resource_mut::<UiWorkState>().record_iteration();
        world.run_schedule(UiSettlementSchedule);

        let work_state = world.resource::<UiWorkState>();
        let started_generation = work_state.current_generation() != generation_before;
        let settled = work_state.is_settled();

        if !had_pending_before && !started_generation && settled {
            exhausted = false;
            break;
        }
    }

    if exhausted {
        let (generation, pending, iterations) = {
            let work_state = world.resource::<UiWorkState>();
            (
                work_state.current_generation(),
                work_state.pending(),
                work_state.last_frame_iterations(),
            )
        };

        world.resource_mut::<UiWorkState>().mark_budget_exhausted();
        bevy::log::warn!(
            "UI settlement exhausted its iteration budget after {iterations} passes (generation={generation}, pending={pending:?})"
        );
    }
}

#[allow(deprecated)]
pub(super) fn begin_ui_settlement_work(
    mut work_state: ResMut<UiWorkState>,
    cache: Res<LayoutCache>,
    root_mutations: Query<
        (),
        Or<(
            Added<URootUi>,
            Changed<URootUi>,
            Added<UScreenRoot>,
            Changed<UScreenRoot>,
            Added<UWorldRoot>,
            Changed<UWorldRoot>,
        )>,
    >,
    layout_nodes: Query<(), With<UNode>>,
    layout_child_mutations: Query<(Entity, Ref<Children>), (With<UNode>, Changed<Children>)>,
    layout_mutations: Query<
        (),
        Or<(
            Added<UNode>,
            Changed<UNode>,
            Changed<ULayout>,
            Changed<USelf>,
        )>,
    >,
    render_mutations: Query<
        (),
        Or<(
            Added<UBorder>,
            Changed<UBorder>,
            Added<UImage>,
            Changed<UImage>,
            Added<UPbr>,
            Changed<UPbr>,
            Added<UClip>,
            Changed<UClip>,
        )>,
    >,
    mut removed_nodes: RemovedComponents<UNode>,
    mut removed_children: RemovedComponents<ChildOf>,
) {
    let roots_changed = !root_mutations.is_empty();
    let layout_children_changed = layout_child_mutations.iter().any(|(entity, children)| {
        let current_layout_children = collect_layout_children(Some(&children), &layout_nodes);
        layout_children_changed(
            cache.layout_children_snapshot(entity),
            &current_layout_children,
        )
    });
    let removed_layout_children = removed_children
        .read()
        .any(|entity| layout_nodes.get(entity).is_ok() || cache.has_node_snapshot(entity));
    let structure_changed =
        removed_nodes.read().next().is_some() || removed_layout_children || layout_children_changed;
    let layout_changed = !layout_mutations.is_empty() || structure_changed;
    let render_changed = !render_mutations.is_empty();

    let pending = UiPendingStages {
        root_resolve: roots_changed,
        hierarchy: roots_changed || layout_changed || cache.dirty_count() > 0,
        measure: roots_changed || layout_changed || cache.measure_dirty_count() > 0,
        solve: roots_changed || layout_changed || cache.solve_dirty_count() > 0,
        render: roots_changed || layout_changed || render_changed || cache.render_dirty_count() > 0,
    };

    work_state.begin_generation(pending);
}

pub(super) fn mark_root_resolve_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::RootResolve);
}

pub(super) fn mark_hierarchy_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::Hierarchy);
}

pub(super) fn refresh_root_settlement_state(
    cache: Res<LayoutCache>,
    nodes: Query<(Entity, Option<&CachedUiContext>), With<UNode>>,
    mut roots: Query<(Entity, &mut UiRootSettlementState), With<ResolvedRootUi>>,
) {
    let root_entities: Vec<Entity> = roots.iter_mut().map(|(entity, _)| entity).collect();
    let mut next_states = HashMap::<Entity, UiRootSettlementState>::default();

    for entity in root_entities {
        next_states.insert(entity, UiRootSettlementState::default());
    }

    for (entity, cached) in nodes.iter() {
        let root_entity = cached
            .and_then(|context| context.root_entity)
            .unwrap_or(entity);
        let Some(root_state) = next_states.get_mut(&root_entity) else {
            continue;
        };

        root_state.observe_node(cache.stage_versions(entity));
    }

    for (entity, mut state) in roots.iter_mut() {
        let next = next_states.get(&entity).copied().unwrap_or_default();
        if *state != next {
            *state = next;
        }
    }
}

fn root_stage_has_pending_work(
    roots: &Query<&UiRootSettlementState, With<ResolvedRootUi>>,
    generation: u64,
    pending_count: fn(&UiRootSettlementState) -> u32,
) -> bool {
    generation != 0
        && roots
            .iter()
            .any(|state| state.current_generation == generation && pending_count(state) > 0)
}

pub(super) fn mark_measure_complete(
    mut work_state: ResMut<UiWorkState>,
    roots: Query<&UiRootSettlementState, With<ResolvedRootUi>>,
) {
    let generation = work_state.current_generation();
    if !root_stage_has_pending_work(&roots, generation, |state| state.pending_measure) {
        work_state.complete_stage(UiWorkStage::Measure);
    }
}

pub(super) fn mark_solve_complete(
    mut work_state: ResMut<UiWorkState>,
    roots: Query<&UiRootSettlementState, With<ResolvedRootUi>>,
) {
    let generation = work_state.current_generation();
    if !root_stage_has_pending_work(&roots, generation, |state| state.pending_solve) {
        work_state.complete_stage(UiWorkStage::Solve);
    }
}

pub(super) fn mark_render_complete(
    mut work_state: ResMut<UiWorkState>,
    roots: Query<&UiRootSettlementState, With<ResolvedRootUi>>,
) {
    let generation = work_state.current_generation();
    if !root_stage_has_pending_work(&roots, generation, |state| state.pending_render) {
        work_state.complete_stage(UiWorkStage::Render);
    }
}
