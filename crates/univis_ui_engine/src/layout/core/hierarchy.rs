use crate::internal_prelude::*;
use bevy::{ecs::relationship::Relationship, platform::collections::HashSet, prelude::*};

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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::SystemState;

    fn sample_root(root_entity: Entity) -> (ResolvedRootUi, ResolvedRootStack) {
        let resolved = ResolvedRootUi {
            root_entity,
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            canvas_size: Vec2::new(800.0, 600.0),
            camera_entity: None,
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        };
        let band_width = 0.04;
        let stack = ResolvedRootStack {
            authored_root_z: 3.0,
            spawn_rank: 9,
            capsule_sort_key: 5.0,
            capsule_band_base: 5.0,
            capsule_band_width: band_width,
            capsule_band_step: band_width / 2048.0,
            applied_root_z: 42.0,
            initialized: true,
        };

        (resolved, stack)
    }

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

    #[test]
    fn root_stack_write_noise_does_not_require_subtree_sync() {
        let mut world = World::new();

        let root = world.spawn(UNode::default()).id();
        let (resolved, stack) = sample_root(root);
        let cached = CachedUiContext {
            root_entity: Some(root),
            camera_entity: resolved.camera_entity,
            space: resolved.space,
            ui_to_world_scale: resolved.ui_units_to_world_scale(),
            root_stack: stack.descendant_context_snapshot(),
            clip_ancestor: None,
        };

        world.entity_mut(root).insert((resolved, stack, cached));

        let mut system_state: SystemState<(
            Query<Option<&CachedUiContext>, With<UNode>>,
            Query<&ChildOf>,
            Query<(&ResolvedRootUi, Option<&ResolvedRootStack>), With<ResolvedRootUi>>,
            Query<&UClip>,
        )> = SystemState::new(&mut world);

        {
            let (node_contexts, parents_query, root_query, clipper_query) =
                system_state.get(&world);
            assert!(!root_context_requires_subtree_sync(
                root,
                &node_contexts,
                &parents_query,
                &root_query,
                &clipper_query,
            ));
        }

        world
            .entity_mut(root)
            .get_mut::<ResolvedRootStack>()
            .expect("root should have stack")
            .applied_root_z = 128.0;

        {
            let (node_contexts, parents_query, root_query, clipper_query) =
                system_state.get(&world);
            assert!(!root_context_requires_subtree_sync(
                root,
                &node_contexts,
                &parents_query,
                &root_query,
                &clipper_query,
            ));
        }

        {
            let mut entity = world.entity_mut(root);
            let mut stack = entity
                .get_mut::<ResolvedRootStack>()
                .expect("root should have stack");
            stack.capsule_band_width *= 2.0;
            stack.capsule_band_step *= 2.0;
        }

        let (node_contexts, parents_query, root_query, clipper_query) = system_state.get(&world);
        assert!(root_context_requires_subtree_sync(
            root,
            &node_contexts,
            &parents_query,
            &root_query,
            &clipper_query,
        ));
    }

    #[test]
    fn removing_child_updates_only_the_detached_subtree_context() {
        let mut app = App::new();
        app.add_systems(Update, update_cached_ui_contexts);

        let root = app.world_mut().spawn(UNode::default()).id();
        let (resolved, stack) = sample_root(root);
        app.world_mut().entity_mut(root).insert((resolved, stack));

        let detached = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();
        let grandchild = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(detached)))
            .id();
        let sibling = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        assert!(
            app.world().entity(detached).contains::<CachedUiContext>(),
            "detached branch should start with cached context",
        );
        assert!(
            app.world().entity(sibling).contains::<CachedUiContext>(),
            "sibling should start with cached context",
        );

        app.world_mut().entity_mut(detached).remove::<ChildOf>();
        app.update();

        assert!(
            !app.world().entity(detached).contains::<CachedUiContext>(),
            "detached node should lose root context after unparenting",
        );
        assert!(
            !app.world().entity(grandchild).contains::<CachedUiContext>(),
            "detached descendants should lose root context too",
        );

        let sibling_cached = app
            .world()
            .entity(sibling)
            .get::<CachedUiContext>()
            .copied()
            .expect("sibling should keep cached context");
        assert_eq!(sibling_cached.root_entity, Some(root));
    }

    #[test]
    fn removing_clip_updates_descendant_clip_context() {
        let mut app = App::new();
        app.add_systems(Update, update_cached_ui_contexts);

        let root = app.world_mut().spawn(UNode::default()).id();
        let (resolved, stack) = sample_root(root);
        app.world_mut().entity_mut(root).insert((resolved, stack));

        let clipper = app
            .world_mut()
            .spawn((UNode::default(), UClip::enabled(true), ChildOf(root)))
            .id();
        let leaf = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(clipper)))
            .id();

        app.update();

        let initial = app
            .world()
            .entity(leaf)
            .get::<CachedUiContext>()
            .copied()
            .expect("leaf should start with cached context");
        assert_eq!(initial.clip_ancestor, Some(clipper));

        app.world_mut().entity_mut(clipper).remove::<UClip>();
        app.update();

        let updated = app
            .world()
            .entity(leaf)
            .get::<CachedUiContext>()
            .copied()
            .expect("leaf should keep cached context after clip removal");
        assert_eq!(updated.root_entity, Some(root));
        assert_eq!(updated.clip_ancestor, None);
    }
}
