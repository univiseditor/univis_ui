use super::*;
use bevy::camera::{
    CameraProjection, ComputedCameraValues, NormalizedRenderTarget, RenderTargetInfo,
};
use bevy::ecs::system::SystemState;
use bevy::picking::pointer::Location;
use univis_ui_engine::layout::layout_system::{URootUi, UiCanvasSize, UiSpace};

#[test]
fn panel_resize_edge_to_cursor_icon() {
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::N),
        SystemCursorIcon::NsResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::S),
        SystemCursorIcon::NsResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::E),
        SystemCursorIcon::EwResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::W),
        SystemCursorIcon::EwResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::NE),
        SystemCursorIcon::NeswResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::SW),
        SystemCursorIcon::NeswResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::NW),
        SystemCursorIcon::NwseResize
    );
    assert_eq!(
        cursor_icon_for_edge(PanelResizeEdge::SE),
        SystemCursorIcon::NwseResize
    );
}

#[test]
fn panel_resize_west_clamp_updates_left() {
    let mut rect = PanelRect {
        width: 200.0,
        height: 120.0,
        left: 20.0,
        top: 10.0,
    };
    apply_resize_delta_to_rect(
        PanelResizeEdge::W,
        Vec2::new(80.0, 0.0),
        150.0,
        80.0,
        &mut rect,
    );

    assert_eq!(rect.width, 150.0);
    assert_eq!(rect.left, 70.0);
}

#[test]
fn panel_resize_north_clamp_updates_top() {
    let mut rect = PanelRect {
        width: 220.0,
        height: 200.0,
        left: 0.0,
        top: 30.0,
    };
    apply_resize_delta_to_rect(
        PanelResizeEdge::N,
        Vec2::new(0.0, -80.0),
        100.0,
        150.0,
        &mut rect,
    );

    assert_eq!(rect.height, 150.0);
    assert_eq!(rect.top, 80.0);
}

#[test]
fn panel_resize_corner_updates_both_axes() {
    let mut rect = PanelRect {
        width: 220.0,
        height: 180.0,
        left: 40.0,
        top: 20.0,
    };
    apply_resize_delta_to_rect(
        PanelResizeEdge::NW,
        Vec2::new(-30.0, 20.0),
        120.0,
        100.0,
        &mut rect,
    );

    assert_eq!(rect.width, 250.0);
    assert_eq!(rect.left, 10.0);
    assert_eq!(rect.height, 200.0);
    assert_eq!(rect.top, 0.0);
}

#[test]
fn panel_cursor_priority_prefers_pressed() {
    let icon = pick_cursor_icon(
        [
            (PanelResizeEdge::NW, UInteraction::Hovered),
            (PanelResizeEdge::E, UInteraction::Pressed),
        ]
        .into_iter(),
    );
    assert_eq!(icon, SystemCursorIcon::EwResize);

    let corner_icon = pick_cursor_icon(
        [
            (PanelResizeEdge::E, UInteraction::Hovered),
            (PanelResizeEdge::NW, UInteraction::Hovered),
        ]
        .into_iter(),
    );
    assert_eq!(corner_icon, SystemCursorIcon::NwseResize);
}

#[test]
fn resolve_parent_size_uses_root_canvas_when_panel_has_no_sized_parent() {
    let mut app = App::new();
    let root = app
        .world_mut()
        .spawn(ResolvedRootUi {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            canvas_size: Vec2::new(1280.0, 720.0),
            camera_entity: None,
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        })
        .id();
    app.world_mut()
        .entity_mut(root)
        .get_mut::<ResolvedRootUi>()
        .unwrap()
        .root_entity = root;
    let panel = app.world_mut().spawn(ChildOf(root)).id();

    let mut state = SystemState::<(
        Query<&ComputedSize>,
        Query<&ChildOf>,
        Query<&ResolvedRootUi>,
        Query<&Window, With<PrimaryWindow>>,
    )>::new(app.world_mut());
    let (parent_sizes, parents, roots, windows) = state.get(app.world());

    let size = resolve_parent_size(
        panel,
        Some(&ChildOf(root)),
        &parent_sizes,
        &parents,
        &roots,
        &windows,
    )
    .expect("panel should inherit root canvas size when no parent size exists");

    assert_eq!(size, Vec2::new(1280.0, 720.0));
}

#[test]
fn intersect_ray_with_ui_plane_returns_intersection_on_ui_plane() {
    let plane = GlobalTransform::default();
    let hit = math::intersect_ray_with_ui_plane(Vec3::new(0.0, 0.0, 5.0), Vec3::NEG_Z, &plane)
        .expect("ray should intersect the plane");

    assert!(hit.abs_diff_eq(Vec3::ZERO, 1e-5));
}

#[test]
fn cursor_in_parent_space_uses_resolved_root_camera() {
    let mut app = App::new();

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

    let root = app
        .world_mut()
        .spawn((
            ResolvedRootUi {
                root_entity: Entity::PLACEHOLDER,
                space: UiSpace::Screen,
                canvas: UiCanvasSize::Viewport,
                canvas_size: Vec2::new(800.0, 600.0),
                camera_entity: Some(camera_entity),
                meters_per_unit: 1.0,
                resolution_scale: 1.0,
            },
            GlobalTransform::default(),
        ))
        .id();
    app.world_mut()
        .entity_mut(root)
        .get_mut::<ResolvedRootUi>()
        .unwrap()
        .root_entity = root;

    let panel = app
        .world_mut()
        .spawn((ChildOf(root), GlobalTransform::default()))
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

    let mut state = SystemState::<(
        Query<&ChildOf>,
        Query<&ResolvedRootUi>,
        Query<(&PointerId, &PointerLocation)>,
        Query<(Entity, &Camera, &GlobalTransform), With<Camera>>,
        Query<&GlobalTransform>,
    )>::new(app.world_mut());
    let (parents, roots, pointers, cameras, globals) = state.get(app.world());
    let parent = parents.get(panel).ok();

    let cursor = cursor_in_parent_space(
        panel, parent, &parents, &roots, &pointers, &cameras, &globals,
    )
    .expect("cursor should resolve against the root camera");

    assert!(cursor.abs_diff_eq(Vec2::ZERO, 1e-5));
}
