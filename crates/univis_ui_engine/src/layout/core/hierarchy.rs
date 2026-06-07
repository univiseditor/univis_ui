#[cfg(test)]
mod tests;

use bevy::{ecs::relationship::Relationship, platform::collections::HashSet, prelude::*};

use crate::layout::components::{CachedUiContext, LayoutDepth, LayoutTreeDepth, UiLocalStacking};
use crate::layout::layout_system::{ResolvedRootStack, ResolvedRootUi};
use crate::layout::query::UiPickingContext;
use crate::layout::univis_node::{UClip, UNode, USelf, UZIndex};
use crate::schedule::{UiRolloutConfig, UiValidationMode, UiValidationState, UiWorkState};

/// System to update the `LayoutDepth` component for all UI nodes.
///
/// This runs periodically (or on change) to ensure every node knows its depth level.
/// It also calculates the `LayoutTreeDepth` resource (maximum depth).
pub fn update_layout_hierarchy(
    root_query: Query<Entity, With<ResolvedRootUi>>,
    children_query: Query<&Children>,
    uself_query: Query<&USelf>,
    zindex_query: Query<&UZIndex>,
    mut commands: Commands,
    mut tree_depth: ResMut<LayoutTreeDepth>,
) {
    let mut max_depth = 0;

    for root_entity in root_query.iter() {
        let mut paint_order = Vec::new();
        max_depth = max_depth.max(traverse_and_mark(
            root_entity,
            0,
            &children_query,
            &uself_query,
            &zindex_query,
            &mut commands,
            &mut paint_order,
        ));

        let total = paint_order.len().max(1) as f32;
        for (index, entity) in paint_order.into_iter().enumerate() {
            commands.entity(entity).insert(UiLocalStacking {
                normalized: index as f32 / total,
            });
        }
    }

    tree_depth.max_depth = max_depth;
}

fn traverse_and_mark(
    entity: Entity,
    depth: usize,
    children_q: &Query<&Children>,
    uself_q: &Query<&USelf>,
    zindex_q: &Query<&UZIndex>,
    commands: &mut Commands,
    paint_order: &mut Vec<Entity>,
) -> usize {
    commands.entity(entity).insert(LayoutDepth(depth));
    paint_order.push(entity);

    let mut current_max = depth;

    if let Ok(children) = children_q.get(entity) {
        let mut ordered_children: Vec<(usize, i32, Entity)> = children
            .iter()
            .enumerate()
            .map(|(original_index, child)| {
                #[allow(deprecated)]
                let order = zindex_q.get(child).map_or_else(
                    |_| uself_q.get(child).map_or(0, |uself| uself.order),
                    |zindex| match zindex {
                        UZIndex::Auto => uself_q.get(child).map_or(0, |uself| uself.order),
                        UZIndex::Local(z) => *z,
                        UZIndex::Global(z) => *z, // Temporary fallback for Global until full separate pass is built
                    },
                );
                (original_index, order, child)
            })
            .collect();
        ordered_children.sort_unstable_by(
            |(left_index, left_order, _), (right_index, right_order, _)| {
                left_order
                    .cmp(right_order)
                    .then_with(|| left_index.cmp(right_index))
            },
        );

        for (_, _, child) in ordered_children {
            let child_depth = traverse_and_mark(
                child,
                depth + 1,
                children_q,
                uself_q,
                zindex_q,
                commands,
                paint_order,
            );
            current_max = current_max.max(child_depth);
        }
    }
    current_max
}

/// Refreshes cached root and clip ancestry for UI nodes.
///
/// Root, clip, and hierarchy changes stay scoped to the affected subtree so
/// common structural churn does not force a whole-tree refresh.
pub fn update_cached_ui_contexts(
    mut commands: Commands,
    node_contexts: Query<Option<&CachedUiContext>, With<UNode>>,
    incremental_nodes: Query<
        (Entity, Option<&CachedUiContext>),
        Or<(Added<UNode>, Changed<ChildOf>)>,
    >,
    changed_roots: Query<Entity, Or<(Changed<ResolvedRootUi>, Changed<ResolvedRootStack>)>>,
    changed_clips: Query<Entity, Or<(Added<UClip>, Changed<UClip>)>>,
    mut removed_children: RemovedComponents<ChildOf>,
    mut removed_clips: RemovedComponents<UClip>,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
    root_query: Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: Query<&UClip>,
) {
    let mut visited = HashSet::default();

    for entity in removed_children.read() {
        sync_cached_context_subtree(
            entity,
            &node_contexts,
            &children_query,
            &parents_query,
            &root_query,
            &clipper_query,
            &mut commands,
            &mut visited,
        );
    }

    for entity in removed_clips.read() {
        sync_cached_context_subtree(
            entity,
            &node_contexts,
            &children_query,
            &parents_query,
            &root_query,
            &clipper_query,
            &mut commands,
            &mut visited,
        );
    }

    for (entity, cached) in incremental_nodes.iter() {
        if !visited.insert(entity) {
            continue;
        }

        sync_cached_context_for_entity(
            entity,
            cached.copied(),
            &parents_query,
            &root_query,
            &clipper_query,
            &mut commands,
        );
    }

    for entity in changed_roots.iter() {
        if !root_context_requires_subtree_sync(
            entity,
            &node_contexts,
            &parents_query,
            &root_query,
            &clipper_query,
        ) {
            continue;
        }

        sync_cached_context_subtree(
            entity,
            &node_contexts,
            &children_query,
            &parents_query,
            &root_query,
            &clipper_query,
            &mut commands,
            &mut visited,
        );
    }

    for entity in changed_clips.iter() {
        sync_cached_context_subtree(
            entity,
            &node_contexts,
            &children_query,
            &parents_query,
            &root_query,
            &clipper_query,
            &mut commands,
            &mut visited,
        );
    }
}

/// Refreshes read-only picking snapshots for UI nodes.
///
/// This keeps interaction crates off the engine's hierarchy/cache internals by
/// projecting the minimum picking data they need onto each resolved node.
pub fn update_picking_contexts(
    mut commands: Commands,
    node_contexts: Query<
        (
            Option<&UiPickingContext>,
            Option<&CachedUiContext>,
            Option<&LayoutDepth>,
            Option<&USelf>,
            Option<&UiLocalStacking>,
        ),
        With<UNode>,
    >,
    incremental_nodes: Query<
        (
            Entity,
            Option<&UiPickingContext>,
            Option<&CachedUiContext>,
            Option<&LayoutDepth>,
            Option<&USelf>,
            Option<&UiLocalStacking>,
        ),
        Or<(
            Added<UNode>,
            Changed<ChildOf>,
            Changed<CachedUiContext>,
            Changed<LayoutDepth>,
            Changed<USelf>,
            Changed<UiLocalStacking>,
        )>,
    >,
    changed_roots: Query<Entity, Or<(Changed<ResolvedRootUi>, Changed<ResolvedRootStack>)>>,
    mut removed_children: RemovedComponents<ChildOf>,
    children_query: Query<&Children>,
    root_query: Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
) {
    let mut visited = HashSet::default();

    for entity in removed_children.read() {
        sync_picking_context_subtree(
            entity,
            &node_contexts,
            &children_query,
            &root_query,
            &mut commands,
            &mut visited,
        );
    }

    for (entity, cached, spatial, depth, uself, local_stacking) in incremental_nodes.iter() {
        if !visited.insert(entity) {
            continue;
        }

        sync_picking_context_for_entity(
            entity,
            cached.copied(),
            spatial.copied(),
            depth.copied(),
            uself.copied(),
            local_stacking.copied(),
            &root_query,
            &mut commands,
        );
    }

    for entity in changed_roots.iter() {
        sync_picking_context_subtree(
            entity,
            &node_contexts,
            &children_query,
            &root_query,
            &mut commands,
            &mut visited,
        );
    }
}

/// Shadow-validates the cached UI ancestry against the legacy parent walk.
///
/// This stays behind `UiValidationMode` so rollout harnesses can compare the
/// new cached context against the old path without changing runtime behavior.
pub fn validate_cached_ui_contexts(
    rollout: Option<Res<UiRolloutConfig>>,
    work_state: Option<Res<UiWorkState>>,
    mut validation_state: ResMut<UiValidationState>,
    all_nodes: Query<(Entity, Option<&CachedUiContext>), With<UNode>>,
    parents_query: Query<&ChildOf>,
    root_query: Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: Query<&UClip>,
) {
    let validation_mode = rollout
        .as_ref()
        .map_or(UiValidationMode::Disabled, |config| config.validation);
    if validation_mode == UiValidationMode::Disabled {
        return;
    }

    let current_generation = work_state
        .as_ref()
        .map_or(0, |state| state.current_generation());
    if current_generation == 0 || validation_state.cached_context_generation == current_generation {
        return;
    }

    let mut mismatches = 0usize;
    let mut first_mismatch = None;

    for (entity, cached) in all_nodes.iter() {
        let expected =
            resolve_cached_ui_context(entity, &parents_query, &root_query, &clipper_query);
        let actual = cached.copied();

        if actual != expected {
            mismatches += 1;
            if first_mismatch.is_none() {
                first_mismatch = Some((entity, actual, expected));
            }
        }
    }

    validation_state.record_cached_context_check(current_generation, mismatches);
    if mismatches > 0
        && validation_state.cached_context_warning_generation != Some(current_generation)
    {
        if let Some((entity, actual, expected)) = first_mismatch {
            bevy::log::warn!(
                "Univis cached-context validation mismatch detected (generation={}, mismatches={}, first_entity={entity:?}, actual={actual:?}, expected={expected:?})",
                current_generation,
                mismatches
            );
        }
        validation_state.cached_context_warning_generation = Some(current_generation);
    }
}

fn sync_cached_context_for_entity(
    entity: Entity,
    cached: Option<CachedUiContext>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: &Query<&UClip>,
    commands: &mut Commands,
) {
    let Some(updated) = resolve_cached_ui_context(entity, parents_query, root_query, clipper_query)
    else {
        if cached.is_some() {
            commands.entity(entity).remove::<CachedUiContext>();
        }
        return;
    };

    if cached != Some(updated) {
        commands.entity(entity).insert(updated);
    }
}

fn sync_picking_context_for_entity(
    entity: Entity,
    cached: Option<UiPickingContext>,
    spatial: Option<CachedUiContext>,
    depth: Option<LayoutDepth>,
    uself: Option<USelf>,
    local_stacking: Option<UiLocalStacking>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    commands: &mut Commands,
) {
    let updated = resolve_picking_context(spatial, depth, uself, local_stacking, root_query);

    match updated {
        Some(updated) if cached != Some(updated) => {
            commands.entity(entity).insert(updated);
        }
        None if cached.is_some() => {
            commands.entity(entity).remove::<UiPickingContext>();
        }
        _ => {}
    }
}

fn sync_cached_context_subtree(
    entity: Entity,
    node_contexts: &Query<Option<&CachedUiContext>, With<UNode>>,
    children_query: &Query<&Children>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: &Query<&UClip>,
    commands: &mut Commands,
    visited: &mut HashSet<Entity>,
) {
    if !visited.insert(entity) {
        return;
    }

    if let Ok(cached) = node_contexts.get(entity) {
        sync_cached_context_for_entity(
            entity,
            cached.copied(),
            parents_query,
            root_query,
            clipper_query,
            commands,
        );
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            sync_cached_context_subtree(
                child,
                node_contexts,
                children_query,
                parents_query,
                root_query,
                clipper_query,
                commands,
                visited,
            );
        }
    }
}

fn sync_picking_context_subtree(
    entity: Entity,
    node_contexts: &Query<
        (
            Option<&UiPickingContext>,
            Option<&CachedUiContext>,
            Option<&LayoutDepth>,
            Option<&USelf>,
            Option<&UiLocalStacking>,
        ),
        With<UNode>,
    >,
    children_query: &Query<&Children>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    commands: &mut Commands,
    visited: &mut HashSet<Entity>,
) {
    if !visited.insert(entity) {
        return;
    }

    if let Ok((cached, spatial, depth, uself, local_stacking)) = node_contexts.get(entity) {
        sync_picking_context_for_entity(
            entity,
            cached.copied(),
            spatial.copied(),
            depth.copied(),
            uself.copied(),
            local_stacking.copied(),
            root_query,
            commands,
        );
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            sync_picking_context_subtree(
                child,
                node_contexts,
                children_query,
                root_query,
                commands,
                visited,
            );
        }
    }
}

fn root_context_requires_subtree_sync(
    entity: Entity,
    node_contexts: &Query<Option<&CachedUiContext>, With<UNode>>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: &Query<&UClip>,
) -> bool {
    let Ok(current_cached) = node_contexts.get(entity) else {
        return true;
    };

    let resolved = resolve_cached_ui_context(entity, parents_query, root_query, clipper_query);
    current_cached.copied() != resolved
}

fn resolve_cached_ui_context(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: &Query<&UClip>,
) -> Option<CachedUiContext> {
    let mut current = entity;
    let mut clip_ancestor = None;

    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();

        if clip_ancestor.is_none()
            && let Ok(clip) = clipper_query.get(current)
            && clip.enabled
        {
            clip_ancestor = Some(current);
        }

        if let Ok((root, stack)) = root_query.get(current) {
            return Some(CachedUiContext {
                root_entity: Some(root.root_entity),
                camera_entity: root.camera_entity,
                space: root.space,
                ui_to_world_scale: root.ui_units_to_world_scale(),
                root_stack: stack
                    .copied()
                    .map(ResolvedRootStack::descendant_context_snapshot)
                    .unwrap_or_default(),
                clip_ancestor,
            });
        }
    }

    if let Ok((root, stack)) = root_query.get(entity) {
        return Some(CachedUiContext {
            root_entity: Some(root.root_entity),
            camera_entity: root.camera_entity,
            space: root.space,
            ui_to_world_scale: root.ui_units_to_world_scale(),
            root_stack: stack
                .copied()
                .map(ResolvedRootStack::descendant_context_snapshot)
                .unwrap_or_default(),
            clip_ancestor: None,
        });
    }

    None
}

fn resolve_picking_context(
    spatial: Option<CachedUiContext>,
    depth: Option<LayoutDepth>,
    uself: Option<USelf>,
    local_stacking: Option<UiLocalStacking>,
    root_query: &Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
) -> Option<UiPickingContext> {
    let spatial = spatial?;
    let layout_depth = depth.map_or(0, |value| value.0);
    #[allow(deprecated)]
    let order = uself.map_or(0, |value| value.order);

    let mut camera_entity = spatial.camera_entity;
    let mut space = spatial.space;
    let mut root_sort_key = 0.0;
    let mut local_depth_key = local_stacking.map_or_else(
        || spatial.root_stack.local_depth_key(layout_depth, order),
        |stack| stack.normalized,
    );

    let mut world_scale = 1.0;

    if let Some(root_entity) = spatial.root_entity
        && let Ok((root, stack)) = root_query.get(root_entity)
    {
        camera_entity = root.camera_entity;
        space = root.space;
        world_scale = root.ui_units_to_world_scale();

        if let Some(stack) = stack.copied() {
            root_sort_key = stack.capsule_sort_key;
            if local_stacking.is_none() {
                local_depth_key = stack.local_depth_key(layout_depth, order);
            }
        }
    }

    Some(UiPickingContext {
        root_entity: spatial.root_entity,
        camera_entity,
        space,
        clip_ancestor: spatial.clip_ancestor,
        root_sort_key,
        local_depth_key,
        world_scale,
    })
}
