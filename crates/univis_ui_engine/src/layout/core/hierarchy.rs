use crate::internal_prelude::*;
use bevy::{ecs::relationship::Relationship, prelude::*};

/// System to update the `LayoutDepth` component for all UI nodes.
///
/// This runs periodically (or on change) to ensure every node knows its depth level.
/// It also calculates the `LayoutTreeDepth` resource (maximum depth).
pub fn update_layout_hierarchy(
    root_query: Query<Entity, With<ResolvedRootUi>>,
    children_query: Query<&Children>,
    mut commands: Commands,
    mut tree_depth: ResMut<LayoutTreeDepth>,
) {
    let mut max_depth = 0;

    for root_entity in root_query.iter() {
        max_depth = max_depth.max(traverse_and_mark(
            root_entity,
            0,
            &children_query,
            &mut commands,
        ));
    }

    tree_depth.max_depth = max_depth;
}

/// Recursive helper function to traverse the tree and mark depth.
fn traverse_and_mark(
    entity: Entity,
    depth: usize,
    children_q: &Query<&Children>,
    commands: &mut Commands,
) -> usize {
    // Insert or update depth component
    commands.entity(entity).insert(LayoutDepth(depth));

    let mut current_max = depth;

    if let Ok(children) = children_q.get(entity) {
        for &child in children {
            // Traverse children
            let child_depth = traverse_and_mark(child, depth + 1, children_q, commands);
            current_max = current_max.max(child_depth);
        }
    }
    current_max
}

/// Refreshes cached root and clip ancestry for UI nodes.
///
/// Full refreshes only happen when resolved root state or clip ancestry changes.
/// Common structural additions still stay incremental.
pub fn update_cached_ui_contexts(
    mut commands: Commands,
    all_nodes: Query<(Entity, Option<&CachedUiContext>), With<UNode>>,
    incremental_nodes: Query<
        (Entity, Option<&CachedUiContext>),
        Or<(Added<UNode>, Changed<ChildOf>)>,
    >,
    roots_changed: Query<(), Or<(Changed<ResolvedRootUi>, Changed<ResolvedRootStack>)>>,
    clips_changed: Query<(), Or<(Added<UClip>, Changed<UClip>)>>,
    mut removed_children: RemovedComponents<ChildOf>,
    mut removed_clips: RemovedComponents<UClip>,
    parents_query: Query<&ChildOf>,
    root_query: Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
    clipper_query: Query<&UClip>,
) {
    let requires_full_refresh = !roots_changed.is_empty()
        || !clips_changed.is_empty()
        || removed_children.read().next().is_some()
        || removed_clips.read().next().is_some();

    if requires_full_refresh {
        for (entity, cached) in all_nodes.iter() {
            sync_cached_context_for_entity(
                entity,
                cached.copied(),
                &parents_query,
                &root_query,
                &clipper_query,
                &mut commands,
            );
        }
    } else {
        for (entity, cached) in incremental_nodes.iter() {
            sync_cached_context_for_entity(
                entity,
                cached.copied(),
                &parents_query,
                &root_query,
                &clipper_query,
                &mut commands,
            );
        }
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
                root_stack: stack.copied().unwrap_or_default(),
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
            root_stack: stack.copied().unwrap_or_default(),
            clip_ancestor: None,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_context_validation_reports_shadow_mismatches() {
        let mut app = App::new();
        app.init_resource::<UiValidationState>();
        app.insert_resource(UiRolloutConfig {
            validation: UiValidationMode::LogWarnings,
            ..default()
        });

        let mut work_state = UiWorkState::default();
        assert!(work_state.begin_generation(UiPendingStages {
            hierarchy: true,
            ..default()
        }));
        work_state.complete_stage(UiWorkStage::Hierarchy);
        app.insert_resource(work_state);

        app.add_systems(Update, validate_cached_ui_contexts);

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::Screen,
                    canvas: UiCanvasSize::Viewport,
                    canvas_size: Vec2::new(800.0, 600.0),
                    camera_entity: None,
                    meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                    resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
                },
                ResolvedRootStack::default(),
            ))
            .id();
        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved root state")
            .root_entity = root;
        app.world_mut().entity_mut(root).insert(CachedUiContext {
            root_entity: Some(root),
            space: UiSpace::Screen,
            ui_to_world_scale: 1.0,
            root_stack: ResolvedRootStack::default(),
            ..default()
        });

        app.world_mut().spawn((
            UNode::default(),
            ChildOf(root),
            CachedUiContext {
                root_entity: None,
                ..default()
            },
        ));

        app.update();

        let validation_state = app.world().resource::<UiValidationState>();
        assert_eq!(validation_state.cached_context_generation, 1);
        assert_eq!(validation_state.cached_context_mismatches, 1);
    }
}
