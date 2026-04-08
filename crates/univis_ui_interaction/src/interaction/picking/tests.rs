use super::*;
use bevy::camera::{
    CameraProjection, ComputedCameraValues, NormalizedRenderTarget, RenderTargetInfo,
};
use bevy::ecs::message::Messages;
use bevy::ecs::system::SystemState;
use bevy::picking::pointer::Location;
use univis_ui_engine::layout::core::hierarchy::update_cached_ui_contexts;
use univis_ui_engine::schedule::UiPendingStages;

fn sample_root(
    root_entity: Entity,
    space: univis_ui_engine::layout::layout_system::UiSpace,
) -> (ResolvedRootUi, ResolvedRootStack) {
    let root = ResolvedRootUi {
        root_entity,
        space,
        canvas: match space {
            univis_ui_engine::layout::layout_system::UiSpace::Screen => {
                univis_ui_engine::layout::layout_system::UiCanvasSize::Viewport
            }
            univis_ui_engine::layout::layout_system::UiSpace::World2d
            | univis_ui_engine::layout::layout_system::UiSpace::World3d => {
                univis_ui_engine::layout::layout_system::UiCanvasSize::Fixed(Vec2::new(
                    800.0, 600.0,
                ))
            }
        },
        canvas_size: Vec2::new(800.0, 600.0),
        camera_entity: Some(Entity::PLACEHOLDER),
        meters_per_unit: univis_ui_engine::layout::layout_system::URootUi::DEFAULT_METERS_PER_UNIT,
        resolution_scale:
            univis_ui_engine::layout::layout_system::URootUi::DEFAULT_RESOLUTION_SCALE,
    };
    let band_width = if matches!(
        space,
        univis_ui_engine::layout::layout_system::UiSpace::Screen
    ) {
        0.004
    } else {
        root.ui_units_to_world_scale() * 0.04
    };
    (root, ResolvedRootStack::with_capsule(0.0, band_width))
}

fn run_picking_for_space(
    space: univis_ui_engine::layout::layout_system::UiSpace,
    perspective: bool,
) -> Vec<PointerHits> {
    let mut app = App::new();
    app.add_message::<PointerHits>();
    app.add_systems(Update, univis_picking_backend);

    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: UVec2::new(800, 600),
            scale_factor: 1.0,
        }),
        clip_from_view: if perspective {
            PerspectiveProjection {
                fov: core::f32::consts::FRAC_PI_2,
                aspect_ratio: 800.0 / 600.0,
                near: 0.1,
                ..default()
            }
            .get_clip_from_view()
        } else {
            OrthographicProjection {
                area: Rect::new(-400.0, -300.0, 400.0, 300.0),
                ..OrthographicProjection::default_2d()
            }
            .get_clip_from_view()
        },
        ..default()
    };

    let camera_entity = app
        .world_mut()
        .spawn((
            camera,
            GlobalTransform::from(Transform::from_xyz(
                0.0,
                0.0,
                if perspective { 5.0 } else { 1000.0 },
            )),
        ))
        .id();

    let root = app.world_mut().spawn_empty().id();
    let (resolved_root, resolved_stack) = sample_root(root, space);
    app.world_mut()
        .entity_mut(root)
        .insert((resolved_root, resolved_stack))
        .insert(GlobalTransform::default());

    let node = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UInteraction::default(),
            LayoutDepth(1),
            UNode {
                width: univis_ui_engine::layout::geometry::UVal::Px(200.0),
                height: univis_ui_engine::layout::geometry::UVal::Px(120.0),
                ..default()
            },
            ComputedSize {
                width: 200.0,
                height: 120.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::default(),
        ))
        .id();

    app.world_mut()
        .entity_mut(root)
        .get_mut::<ResolvedRootUi>()
        .unwrap()
        .camera_entity = Some(camera_entity);

    app.world_mut().spawn((
        PointerId::Mouse,
        PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: 800,
                height: 600,
            },
            position: Vec2::new(400.0, 300.0),
        }),
    ));

    app.update();

    let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
    let collected = hits.drain().collect::<Vec<_>>();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0].picks.len(), 1);
    assert_eq!(collected[0].picks[0].0, node);
    collected
}

#[test]
fn post_settle_picking_hits_first_frame_geometry_with_cached_context() {
    let mut app = App::new();
    app.add_message::<PointerHits>();
    app.init_resource::<PickingSyncState>();
    app.init_resource::<UiValidationState>();
    app.add_systems(Update, post_settle_picking_backend);

    let mut work_state = UiWorkState::default();
    work_state.begin_generation(UiPendingStages {
        render: true,
        ..Default::default()
    });
    app.insert_resource(work_state);

    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: UVec2::new(800, 600),
            scale_factor: 1.0,
        }),
        clip_from_view: OrthographicProjection {
            area: Rect::new(-400.0, -300.0, 400.0, 300.0),
            ..OrthographicProjection::default_2d()
        }
        .get_clip_from_view(),
        ..default()
    };

    let camera_entity = app
        .world_mut()
        .spawn((
            camera,
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
        ))
        .id();
    let root = app.world_mut().spawn_empty().id();
    let (_, root_stack) = sample_root(
        root,
        univis_ui_engine::layout::layout_system::UiSpace::Screen,
    );

    let node = app
        .world_mut()
        .spawn((
            UInteraction::default(),
            LayoutDepth(1),
            UNode {
                width: univis_ui_engine::layout::geometry::UVal::Px(200.0),
                height: univis_ui_engine::layout::geometry::UVal::Px(120.0),
                ..default()
            },
            ComputedSize {
                width: 200.0,
                height: 120.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::default(),
            CachedUiContext {
                root_entity: Some(root),
                camera_entity: Some(camera_entity),
                space: univis_ui_engine::layout::layout_system::UiSpace::Screen,
                ui_to_world_scale: 1.0,
                root_stack,
                clip_ancestor: None,
            },
        ))
        .id();

    app.world_mut().spawn((
        PointerId::Mouse,
        PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: 800,
                height: 600,
            },
            position: Vec2::new(400.0, 300.0),
        }),
    ));
    app.world_mut()
        .resource_mut::<PickingSyncState>()
        .pointer_generation = 1;

    app.update();

    let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
    let collected = hits.drain().collect::<Vec<_>>();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0].picks.len(), 1);
    assert_eq!(collected[0].picks[0].0, node);
}

#[test]
fn post_settle_picking_validation_records_shadow_mismatches_without_emitting_hits() {
    let mut app = App::new();
    app.add_message::<PointerHits>();
    app.init_resource::<PickingSyncState>();
    app.init_resource::<UiValidationState>();
    app.insert_resource(UiRolloutConfig {
        use_post_settle_picking: false,
        validation: UiValidationMode::LogWarnings,
        ..default()
    });
    app.add_systems(Update, post_settle_picking_backend);

    let mut work_state = UiWorkState::default();
    work_state.begin_generation(UiPendingStages {
        render: true,
        ..Default::default()
    });
    app.insert_resource(work_state);

    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: UVec2::new(800, 600),
            scale_factor: 1.0,
        }),
        clip_from_view: OrthographicProjection {
            area: Rect::new(-400.0, -300.0, 400.0, 300.0),
            ..OrthographicProjection::default_2d()
        }
        .get_clip_from_view(),
        ..default()
    };

    let camera_entity = app
        .world_mut()
        .spawn((
            camera,
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
        ))
        .id();
    let root = app.world_mut().spawn_empty().id();
    let (_, root_stack) = sample_root(
        root,
        univis_ui_engine::layout::layout_system::UiSpace::Screen,
    );
    let clipper = app
        .world_mut()
        .spawn((
            GlobalTransform::from(Transform::from_xyz(500.0, 500.0, 0.0)),
            ComputedSize {
                width: 20.0,
                height: 20.0,
                ..default()
            },
            UNode::default(),
            UClip { enabled: true },
        ))
        .id();

    app.world_mut().spawn((
        ChildOf(root),
        UInteraction::default(),
        LayoutDepth(1),
        UNode {
            width: univis_ui_engine::layout::geometry::UVal::Px(200.0),
            height: univis_ui_engine::layout::geometry::UVal::Px(120.0),
            ..default()
        },
        ComputedSize {
            width: 200.0,
            height: 120.0,
            local_pos: Vec2::ZERO,
        },
        GlobalTransform::default(),
        CachedUiContext {
            root_entity: Some(root),
            camera_entity: Some(camera_entity),
            space: univis_ui_engine::layout::layout_system::UiSpace::Screen,
            ui_to_world_scale: 1.0,
            root_stack,
            clip_ancestor: Some(clipper),
        },
    ));

    let (mut resolved_root, resolved_stack) = sample_root(
        root,
        univis_ui_engine::layout::layout_system::UiSpace::Screen,
    );
    resolved_root.camera_entity = Some(camera_entity);
    app.world_mut()
        .entity_mut(root)
        .insert((resolved_root, resolved_stack));

    app.world_mut().spawn((
        PointerId::Mouse,
        PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: 800,
                height: 600,
            },
            position: Vec2::new(400.0, 300.0),
        }),
    ));
    app.world_mut()
        .resource_mut::<PickingSyncState>()
        .pointer_generation = 1;

    app.update();

    let validation_state = app.world().resource::<UiValidationState>();
    assert_eq!(validation_state.picking_shadow_ui_generation, 1);
    assert_eq!(validation_state.picking_shadow_pointer_generation, 1);
    assert_eq!(validation_state.picking_shadow_mismatches, 1);

    let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
    let collected = hits.drain().collect::<Vec<_>>();
    assert!(collected.is_empty());
}

#[test]
fn post_settle_picking_refreshes_cached_camera_after_root_state_change() {
    let mut app = App::new();
    app.add_message::<PointerHits>();
    app.init_resource::<PickingSyncState>();
    app.init_resource::<UiValidationState>();
    app.add_systems(
        Update,
        (update_cached_ui_contexts, post_settle_picking_backend).chain(),
    );

    let mut camera_a = Camera::default();
    camera_a.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: UVec2::new(800, 600),
            scale_factor: 1.0,
        }),
        clip_from_view: OrthographicProjection {
            area: Rect::new(-400.0, -300.0, 400.0, 300.0),
            ..OrthographicProjection::default_2d()
        }
        .get_clip_from_view(),
        ..default()
    };
    let camera_a = app
        .world_mut()
        .spawn((
            camera_a,
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
        ))
        .id();

    let mut camera_b = Camera::default();
    camera_b.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: UVec2::new(800, 600),
            scale_factor: 1.0,
        }),
        clip_from_view: OrthographicProjection {
            area: Rect::new(-400.0, -300.0, 400.0, 300.0),
            ..OrthographicProjection::default_2d()
        }
        .get_clip_from_view(),
        ..default()
    };
    let camera_b = app
        .world_mut()
        .spawn((
            camera_b,
            GlobalTransform::from(Transform::from_xyz(250.0, 0.0, 1000.0)),
        ))
        .id();

    let root = app.world_mut().spawn(UNode::default()).id();
    let (mut resolved_root, resolved_stack) = sample_root(
        root,
        univis_ui_engine::layout::layout_system::UiSpace::Screen,
    );
    resolved_root.camera_entity = Some(camera_a);
    app.world_mut()
        .entity_mut(root)
        .insert((resolved_root, resolved_stack));

    let node = app
        .world_mut()
        .spawn((
            ChildOf(root),
            UInteraction::default(),
            LayoutDepth(1),
            UNode {
                width: univis_ui_engine::layout::geometry::UVal::Px(120.0),
                height: univis_ui_engine::layout::geometry::UVal::Px(80.0),
                ..default()
            },
            ComputedSize {
                width: 120.0,
                height: 80.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::default(),
        ))
        .id();

    app.world_mut().spawn((
        PointerId::Mouse,
        PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: 800,
                height: 600,
            },
            position: Vec2::new(400.0, 300.0),
        }),
    ));
    app.world_mut()
        .resource_mut::<PickingSyncState>()
        .pointer_generation = 1;

    app.update();

    {
        let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
        let collected = hits.drain().collect::<Vec<_>>();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].picks.len(), 1);
        assert_eq!(collected[0].picks[0].0, node);
    }

    let cached_before = app
        .world()
        .entity(node)
        .get::<CachedUiContext>()
        .copied()
        .expect("node should have cached context after the first update");
    assert_eq!(cached_before.camera_entity, Some(camera_a));

    app.world_mut()
        .entity_mut(root)
        .get_mut::<ResolvedRootUi>()
        .expect("root should keep resolved root state")
        .camera_entity = Some(camera_b);
    app.world_mut()
        .resource_mut::<PickingSyncState>()
        .pointer_generation = 2;

    app.update();

    let cached_after = app
        .world()
        .entity(node)
        .get::<CachedUiContext>()
        .copied()
        .expect("node should keep cached context after camera reassignment");
    assert_eq!(cached_after.camera_entity, Some(camera_b));

    let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
    let collected = hits.drain().collect::<Vec<_>>();
    assert!(collected.is_empty());
}

#[test]
fn resolve_root_for_entity_walks_up_the_parent_chain() {
    let mut app = App::new();

    let root = app.world_mut().spawn_empty().id();
    app.world_mut().entity_mut(root).insert(sample_root(
        root,
        univis_ui_engine::layout::layout_system::UiSpace::Screen,
    ));

    let child = app.world_mut().spawn(ChildOf(root)).id();
    let grandchild = app.world_mut().spawn(ChildOf(child)).id();

    let mut state = SystemState::<(
        Query<&ChildOf>,
        Query<(&ResolvedRootUi, &ResolvedRootStack)>,
    )>::new(app.world_mut());
    let (parents, roots) = state.get(app.world());

    let resolved = resolve_root_for_entity(None, grandchild, &parents, &roots)
        .expect("descendants should resolve to the ancestor root");
    assert_eq!(resolved.root_entity, root);
    assert_eq!(
        resolved.space,
        univis_ui_engine::layout::layout_system::UiSpace::Screen
    );
}

#[test]
fn intersect_ray_with_node_plane_returns_world_and_local_hit() {
    let ray = CachedPointerRay {
        order: 0.0,
        origin: Vec3::new(0.0, 0.0, 5.0),
        direction: Vec3::NEG_Z,
    };
    let transform = GlobalTransform::from(Transform::default());

    let (world_hit, local_hit, normal, distance) = intersect_ray_with_node_plane(&ray, &transform)
        .expect("ray should intersect the default UI plane");

    assert!(world_hit.abs_diff_eq(Vec3::ZERO, 1e-5));
    assert!(local_hit.abs_diff_eq(Vec2::ZERO, 1e-5));
    assert!(normal.abs_diff_eq(Vec3::Z, 1e-5));
    assert!((distance - 5.0).abs() <= 1e-5);
}

#[test]
fn is_clipped_by_ancestors_detects_points_outside_enabled_clipper() {
    let mut app = App::new();

    let clipper = app
        .world_mut()
        .spawn((
            GlobalTransform::default(),
            ComputedSize {
                width: 100.0,
                height: 100.0,
                ..default()
            },
            UNode::default(),
            UClip { enabled: true },
        ))
        .id();
    let child = app.world_mut().spawn(ChildOf(clipper)).id();

    let mut state = SystemState::<(
        Query<&ChildOf>,
        Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    )>::new(app.world_mut());
    let (parents, clippers) = state.get(app.world());

    assert!(!is_clipped_by_ancestors(
        child,
        Vec3::new(0.0, 0.0, 0.0),
        &parents,
        &clippers,
    ));
    assert!(is_clipped_by_ancestors(
        child,
        Vec3::new(80.0, 80.0, 0.0),
        &parents,
        &clippers,
    ));
}

#[test]
fn picking_prefers_the_higher_root_capsule_over_a_deeper_child_in_a_lower_root() {
    let mut app = App::new();
    app.add_message::<PointerHits>();
    app.add_systems(Update, univis_picking_backend);

    let mut camera = Camera::default();
    camera.computed = ComputedCameraValues {
        target_info: Some(RenderTargetInfo {
            physical_size: UVec2::new(800, 600),
            scale_factor: 1.0,
        }),
        clip_from_view: OrthographicProjection {
            area: Rect::new(-400.0, -300.0, 400.0, 300.0),
            ..OrthographicProjection::default_2d()
        }
        .get_clip_from_view(),
        ..default()
    };

    let camera_entity = app
        .world_mut()
        .spawn((
            camera,
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 1000.0)),
        ))
        .id();

    let lower_root = app.world_mut().spawn_empty().id();
    let (mut lower_resolved, mut lower_stack) = sample_root(
        lower_root,
        univis_ui_engine::layout::layout_system::UiSpace::World2d,
    );
    lower_resolved.camera_entity = Some(camera_entity);
    lower_stack.capsule_sort_key = 0.0;
    app.world_mut()
        .entity_mut(lower_root)
        .insert((lower_resolved, lower_stack));

    let upper_root = app.world_mut().spawn_empty().id();
    let (mut upper_resolved, mut upper_stack) = sample_root(
        upper_root,
        univis_ui_engine::layout::layout_system::UiSpace::World2d,
    );
    upper_resolved.camera_entity = Some(camera_entity);
    upper_stack.capsule_sort_key = 0.05;
    app.world_mut()
        .entity_mut(upper_root)
        .insert((upper_resolved, upper_stack));

    let lower_node = app
        .world_mut()
        .spawn((
            ChildOf(lower_root),
            UInteraction::default(),
            LayoutDepth(3),
            USelf {
                order: 32,
                ..default()
            },
            UNode {
                width: univis_ui_engine::layout::geometry::UVal::Px(200.0),
                height: univis_ui_engine::layout::geometry::UVal::Px(120.0),
                ..default()
            },
            ComputedSize {
                width: 200.0,
                height: 120.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 0.08)),
        ))
        .id();

    let upper_node = app
        .world_mut()
        .spawn((
            ChildOf(upper_root),
            UInteraction::default(),
            LayoutDepth(1),
            UNode {
                width: univis_ui_engine::layout::geometry::UVal::Px(200.0),
                height: univis_ui_engine::layout::geometry::UVal::Px(120.0),
                ..default()
            },
            ComputedSize {
                width: 200.0,
                height: 120.0,
                local_pos: Vec2::ZERO,
            },
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 0.02)),
        ))
        .id();

    app.world_mut().spawn((
        PointerId::Mouse,
        PointerLocation::new(Location {
            target: NormalizedRenderTarget::None {
                width: 800,
                height: 600,
            },
            position: Vec2::new(400.0, 300.0),
        }),
    ));

    app.update();

    let mut hits = app.world_mut().resource_mut::<Messages<PointerHits>>();
    let collected = hits.drain().collect::<Vec<_>>();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0].picks.len(), 2);
    assert_eq!(collected[0].picks[0].0, upper_node);
    assert_eq!(collected[0].picks[1].0, lower_node);
}

#[test]
fn picking_backend_hits_screen_nodes() {
    let hits = run_picking_for_space(
        univis_ui_engine::layout::layout_system::UiSpace::Screen,
        false,
    );
    assert_eq!(hits[0].order, 0.0);
}

#[test]
fn picking_backend_hits_world2d_nodes() {
    let hits = run_picking_for_space(
        univis_ui_engine::layout::layout_system::UiSpace::World2d,
        false,
    );
    assert_eq!(hits[0].order, 0.0);
}

#[test]
fn picking_backend_hits_world3d_nodes_with_perspective_camera() {
    let hits = run_picking_for_space(
        univis_ui_engine::layout::layout_system::UiSpace::World3d,
        true,
    );
    assert_eq!(hits[0].order, 0.0);
}
