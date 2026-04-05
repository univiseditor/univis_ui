//! Layout and root systems for Univis UI.
//!
//! This module contains:
//!
//! - [`crate::layout::layout_system`] for [`crate::layout::layout_system::URootUi`]
//!   and root resolution.
//! - [`crate::layout::univis_node`] for node, layout, and local positioning components.
//! - [`crate::layout::geometry`] for logical UI units and spacing helpers.
//! - [`crate::layout::render`] for mesh/material synchronization.
//!
//! Most users reach this module through [`crate::prelude`] or
//! [`crate::layout::prelude`].

#[doc(hidden)]
pub mod algorithms;
#[doc(hidden)]
pub mod components;
#[doc(hidden)]
pub mod core;
/// Logical UI units, spacing helpers, and box-side utilities.
pub mod geometry;
/// Image-backed node helpers.
pub mod image;
/// The public root model and root-resolution systems.
pub mod layout_system;
/// Physically based material overrides for `World3d` content.
pub mod pbr;
#[doc(hidden)]
pub mod pipeline;
/// Optional profiling and layout diagnostics helpers.
pub mod profiling;
/// Mesh/material synchronization and render-facing types.
pub mod render;
#[doc(hidden)]
pub mod solver_types;
/// Core node, layout, and local positioning components.
pub mod univis_node;

/// Common imports for authoring layout trees and roots directly from the engine crate.
pub mod prelude {
    pub use crate::layout::UnivisLayoutPlugin;
    pub use crate::layout::geometry::{UCornerRadius, USides, UVal};
    pub use crate::layout::image::UImage;
    #[allow(deprecated)]
    pub use crate::layout::layout_system::{
        URootUi, UScreenRoot, UWorldRoot, UiCameraRef, UiCanvasSize, UiSpace,
    };
    pub use crate::layout::pbr::UPbr;
    pub use crate::layout::univis_node::*;
}

use crate::internal_prelude::*;
use bevy::asset::AssetEventSystems;
use bevy::prelude::*;
use bevy::sprite::update_text2d_layout;

/// Installs the layout pipeline for Univis UI roots and nodes.
///
/// This plugin resolves roots, builds the layout hierarchy, measures content,
/// solves final geometry, and prepares render sync data in `PostUpdate`.
pub struct UnivisLayoutPlugin;

impl Plugin for UnivisLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<USelf>()
            .register_type::<URootUi>()
            .register_type::<UiSpace>()
            .register_type::<UiCanvasSize>()
            .register_type::<UiCameraRef>()
            .register_type::<UAlignSelf>()
            .register_type::<UPosition>()
            .register_type::<ULayoutContainerExt>()
            .register_type::<ULayoutBoxAlignContainer>()
            .register_type::<ULayoutFlexContainer>()
            .register_type::<ULayoutGridContainer>()
            .register_type::<ULayoutItemExt>()
            .register_type::<ULayoutBoxAlignSelf>()
            .register_type::<ULayoutFlexItem>()
            .register_type::<ULayoutGridItem>()
            .register_type::<UAlignSelfExt>()
            .register_type::<UAlignItemsExt>()
            .register_type::<UContentAlignExt>()
            .register_type::<UOverflowPosition>()
            .register_type::<UFlexWrap>()
            .register_type::<UTrackSize>()
            .register_type::<UGridAutoFlow>()
            .init_resource::<LayoutTreeDepth>()
            .init_resource::<UiRolloutConfig>()
            .init_resource::<UiValidationState>()
            .init_resource::<UiSettlementConfig>()
            .init_resource::<UiWorkState>()
            .init_resource::<RootSpawnRankCounter>()
            .init_schedule(UiSettlementSchedule)
            .add_plugins(LayoutCachePlugin)
            .configure_sets(
                UiSettlementSchedule,
                (
                    UnivisPostUpdateSet::WidgetSync,
                    UnivisPostUpdateSet::RootResolve,
                    UnivisPostUpdateSet::LayoutHierarchy,
                    UnivisPostUpdateSet::LayoutMeasure,
                    UnivisPostUpdateSet::LayoutSolve,
                    UnivisPostUpdateSet::RenderSync,
                    UnivisPostUpdateSet::PickSync,
                    UnivisPostUpdateSet::UiSettled,
                )
                    .chain(),
            )
            .add_systems(
                UiSettlementSchedule,
                (
                    begin_ui_settlement_work,
                    resolve_root_ui,
                    assign_root_spawn_ranks,
                    resolve_root_stacking,
                    sync_root_capsule_transforms,
                    mark_root_resolve_complete,
                )
                    .chain()
                    .in_set(UnivisPostUpdateSet::RootResolve),
            )
            .add_systems(
                UiSettlementSchedule,
                (
                    update_layout_hierarchy,
                    update_cached_ui_contexts,
                    update_depth_cache,
                    track_layout_changes,
                    mark_hierarchy_complete,
                )
                    .chain()
                    .in_set(UnivisPostUpdateSet::LayoutHierarchy),
            )
            .add_systems(
                UiSettlementSchedule,
                (
                    upward_measure_pass_cached,
                    sync_fit_content_root_canvas_sizes,
                    mark_measure_complete,
                )
                    .chain()
                    .in_set(UnivisPostUpdateSet::LayoutMeasure),
            )
            .add_systems(
                UiSettlementSchedule,
                (downward_solve_pass_safe, mark_solve_complete)
                    .chain()
                    .in_set(UnivisPostUpdateSet::LayoutSolve),
            )
            .add_systems(
                PostUpdate,
                run_ui_settlement_loop
                    .after(update_text2d_layout)
                    .before(AssetEventSystems),
            )
            .add_systems(
                UiSettlementSchedule,
                (mark_render_complete, validate_cached_ui_contexts)
                    .chain()
                    .in_set(UnivisPostUpdateSet::UiSettled),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::core::layout_cache::LayoutCache;

    #[derive(Component)]
    struct DeferredResizeTarget;

    #[derive(Resource, Default)]
    struct DeferredResizeOnce(bool);

    fn resize_target_in_render_sync(
        mut resize_once: ResMut<DeferredResizeOnce>,
        mut query: Query<&mut UNode, With<DeferredResizeTarget>>,
    ) {
        if resize_once.0 {
            return;
        }

        let mut node = query
            .single_mut()
            .expect("test should only spawn one deferred resize target");
        node.width = UVal::Px(120.0);
        resize_once.0 = true;
    }

    #[test]
    fn fresh_ui_tree_settles_on_the_first_frame() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));

        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(400.0, 200.0)),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    ..default()
                },
                ULayout::default(),
            ))
            .id();

        let child = app
            .world_mut()
            .spawn((
                ChildOf(root),
                UNode {
                    width: UVal::Px(100.0),
                    height: UVal::Px(50.0),
                    ..default()
                },
            ))
            .id();

        app.update();

        let cache = app.world().resource::<LayoutCache>();
        let root_layer = cache
            .get_entities_at_depth(0)
            .expect("root depth bucket should be built on the first frame");
        let child_layer = cache
            .get_entities_at_depth(1)
            .expect("child depth bucket should be built on the first frame");

        assert!(root_layer.contains(&root));
        assert!(child_layer.contains(&child));
        assert_eq!(
            app.world()
                .entity(child)
                .get::<LayoutDepth>()
                .copied()
                .expect("child should have a depth assigned on the first frame"),
            LayoutDepth(1)
        );

        let child_size = app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .copied()
            .expect("child should have a computed size on the first frame");
        assert_eq!(child_size.width, 100.0);
        assert_eq!(child_size.height, 50.0);

        let work_state = app.world().resource::<UiWorkState>();
        assert_eq!(work_state.current_generation(), 1);
        assert!(work_state.is_settled());
        assert_eq!(work_state.last_frame_iterations(), 2);
        assert!(!work_state.budget_exhausted());
        assert_eq!(work_state.completed_generation(UiWorkStage::RootResolve), 1);
        assert_eq!(work_state.completed_generation(UiWorkStage::Hierarchy), 1);
        assert_eq!(work_state.completed_generation(UiWorkStage::Measure), 1);
        assert_eq!(work_state.completed_generation(UiWorkStage::Solve), 1);
        assert_eq!(work_state.completed_generation(UiWorkStage::Render), 1);
    }

    #[test]
    fn settlement_loop_drains_follow_up_layout_work_in_the_same_frame() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
        app.init_resource::<DeferredResizeOnce>();
        app.add_systems(
            UiSettlementSchedule,
            resize_target_in_render_sync.in_set(UnivisPostUpdateSet::RenderSync),
        );

        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(400.0, 200.0)),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    ..default()
                },
                ULayout::default(),
            ))
            .id();

        let child = app
            .world_mut()
            .spawn((
                ChildOf(root),
                DeferredResizeTarget,
                UNode {
                    width: UVal::Px(100.0),
                    height: UVal::Px(50.0),
                    ..default()
                },
            ))
            .id();

        app.update();

        let child_size = app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .copied()
            .expect("child should have a computed size after settlement drains");
        assert_eq!(child_size.width, 120.0);
        assert_eq!(child_size.height, 50.0);

        let work_state = app.world().resource::<UiWorkState>();
        assert!(work_state.is_settled());
        assert_eq!(work_state.current_generation(), 2);
        assert_eq!(work_state.last_frame_iterations(), 3);
        assert!(!work_state.budget_exhausted());
    }

    #[test]
    fn settlement_budget_exhaustion_prevents_the_frame_from_reporting_idle() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
        app.insert_resource(UiSettlementConfig { max_iterations: 1 });
        app.init_resource::<DeferredResizeOnce>();
        app.add_systems(
            UiSettlementSchedule,
            resize_target_in_render_sync.in_set(UnivisPostUpdateSet::RenderSync),
        );

        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(400.0, 200.0)),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    ..default()
                },
                ULayout::default(),
            ))
            .id();

        let child = app
            .world_mut()
            .spawn((
                ChildOf(root),
                DeferredResizeTarget,
                UNode {
                    width: UVal::Px(100.0),
                    height: UVal::Px(50.0),
                    ..default()
                },
            ))
            .id();

        app.update();

        let child_size = app
            .world()
            .entity(child)
            .get::<ComputedSize>()
            .copied()
            .expect("child should still have a computed size after the first iteration");
        assert_eq!(child_size.width, 100.0);

        let work_state = app.world().resource::<UiWorkState>();
        assert_eq!(work_state.current_generation(), 1);
        assert_eq!(work_state.last_frame_iterations(), 1);
        assert!(work_state.budget_exhausted());
        assert!(!work_state.is_settled());
    }

    #[test]
    fn downward_solve_stays_scoped_to_dirty_subtrees() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, UnivisLayoutPlugin));
        app.init_resource::<LayoutProfiler>();

        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(400.0, 200.0)),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    ..default()
                },
                ULayout::default(),
            ))
            .id();

        let branch_a = app
            .world_mut()
            .spawn((
                ChildOf(root),
                UNode {
                    width: UVal::Content,
                    height: UVal::Content,
                    ..default()
                },
                ULayout::default(),
            ))
            .id();

        let leaf_a = app
            .world_mut()
            .spawn((
                ChildOf(branch_a),
                UNode {
                    width: UVal::Px(80.0),
                    height: UVal::Px(40.0),
                    ..default()
                },
            ))
            .id();

        let branch_b = app
            .world_mut()
            .spawn((
                ChildOf(root),
                UNode {
                    width: UVal::Content,
                    height: UVal::Content,
                    ..default()
                },
                ULayout::default(),
            ))
            .id();

        let leaf_b = app
            .world_mut()
            .spawn((
                ChildOf(branch_b),
                UNode {
                    width: UVal::Px(50.0),
                    height: UVal::Px(30.0),
                    ..default()
                },
            ))
            .id();

        app.update();

        app.world_mut()
            .entity_mut(leaf_a)
            .get_mut::<UNode>()
            .unwrap()
            .width = UVal::Px(120.0);
        app.update();

        let profiler = app.world().resource::<LayoutProfiler>();
        assert_eq!(profiler.solved_nodes, 2);

        let branch_b_size = app
            .world()
            .entity(branch_b)
            .get::<ComputedSize>()
            .copied()
            .expect("branch_b should keep its computed size");
        let leaf_b_size = app
            .world()
            .entity(leaf_b)
            .get::<ComputedSize>()
            .copied()
            .expect("leaf_b should keep its computed size");
        assert_eq!(branch_b_size.width, 50.0);
        assert_eq!(leaf_b_size.width, 50.0);
    }
}

fn run_ui_settlement_loop(world: &mut World) {
    let max_iterations = world.resource::<UiSettlementConfig>().max_iterations.max(1);

    world.resource_mut::<UiWorkState>().begin_frame();
    if let Some(mut profiler) = world.get_resource_mut::<LayoutProfiler>() {
        profiler.solved_nodes = 0;
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
fn begin_ui_settlement_work(
    mut work_state: ResMut<UiWorkState>,
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
    layout_mutations: Query<
        (),
        Or<(
            Added<UNode>,
            Changed<UNode>,
            Changed<ULayout>,
            Changed<USelf>,
            Changed<Children>,
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
    let structure_changed =
        removed_nodes.read().next().is_some() || removed_children.read().next().is_some();
    let layout_changed = !layout_mutations.is_empty() || structure_changed;
    let render_changed = !render_mutations.is_empty();

    work_state.begin_generation(UiPendingStages {
        root_resolve: roots_changed,
        hierarchy: roots_changed || layout_changed,
        measure: roots_changed || layout_changed,
        solve: roots_changed || layout_changed,
        render: roots_changed || layout_changed || render_changed,
    });
}

fn mark_root_resolve_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::RootResolve);
}

fn mark_hierarchy_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::Hierarchy);
}

fn mark_measure_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::Measure);
}

fn mark_solve_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::Solve);
}

fn mark_render_complete(mut work_state: ResMut<UiWorkState>) {
    work_state.complete_stage(UiWorkStage::Render);
}
