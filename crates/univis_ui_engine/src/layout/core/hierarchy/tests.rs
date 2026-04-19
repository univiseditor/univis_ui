use super::*;

use bevy::ecs::system::SystemState;

use crate::layout::layout_system::{URootUi, UiCanvasSize, UiSpace};
use crate::schedule::{UiPendingStages, UiWorkStage};

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
        let (node_contexts, parents_query, root_query, clipper_query) = system_state.get(&world);
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
        let (node_contexts, parents_query, root_query, clipper_query) = system_state.get(&world);
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

#[test]
fn picking_context_uses_hierarchical_subtree_stacking_order() {
    let mut app = App::new();
    app.init_resource::<LayoutTreeDepth>();
    app.add_systems(
        Update,
        (
            update_layout_hierarchy,
            update_cached_ui_contexts,
            update_picking_contexts,
        )
            .chain(),
    );

    let root = app.world_mut().spawn(UNode::default()).id();
    let (resolved, stack) = sample_root(root);
    app.world_mut().entity_mut(root).insert((resolved, stack));

    let earlier_branch = app
        .world_mut()
        .spawn((UNode::default(), ChildOf(root)))
        .id();
    let earlier_child = app
        .world_mut()
        .spawn((UNode::default(), ChildOf(earlier_branch)))
        .id();
    let later_branch = app
        .world_mut()
        .spawn((UNode::default(), ChildOf(root)))
        .id();

    app.update();

    let root_key = app
        .world()
        .entity(root)
        .get::<UiPickingContext>()
        .expect("root should have picking context")
        .local_depth_key;
    let earlier_key = app
        .world()
        .entity(earlier_branch)
        .get::<UiPickingContext>()
        .expect("earlier branch should have picking context")
        .local_depth_key;
    let earlier_child_key = app
        .world()
        .entity(earlier_child)
        .get::<UiPickingContext>()
        .expect("earlier child should have picking context")
        .local_depth_key;
    let later_key = app
        .world()
        .entity(later_branch)
        .get::<UiPickingContext>()
        .expect("later branch should have picking context")
        .local_depth_key;

    assert!(root_key < earlier_key);
    assert!(earlier_key < earlier_child_key);
    assert!(earlier_child_key < later_key);
}
