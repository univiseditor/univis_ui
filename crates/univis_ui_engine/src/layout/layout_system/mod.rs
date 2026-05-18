#![allow(deprecated)]
#![allow(clippy::type_complexity)]
//! Root model and root-resolution systems for Univis UI.
//!
//! [`URootUi`](crate::layout::layout_system::URootUi) is the modern public
//! entry point for authoring UI roots. Every root resolves into a
//! [`ResolvedRootUi`](crate::layout::layout_system::ResolvedRootUi) plus an
//! internal [`ResolvedRootStack`](crate::layout::layout_system::ResolvedRootStack)
//! capsule, which keeps cross-root stacking sealed:
//! local ordering stays inside the root and descendants do not automatically
//! interleave above another root.

use bevy::prelude::*;

mod legacy_compat;
mod root_resolution;
mod root_stacking;
mod roots;
mod screen_transform;
mod ui3d_sync;

use self::legacy_compat::effective_root_ui;
use self::root_resolution::{
    emit_root_resolution_warning, resolve_root_camera, resolve_root_canvas_size,
};
pub(crate) use self::root_stacking::{
    resolve_root_stacking, sync_fit_content_root_canvas_sizes, sync_root_capsule_transforms,
};
pub(crate) use self::roots::RootSpawnRankCounter;
pub use self::roots::{
    ResolvedRootStack, ResolvedRootUi, URootUi, UScreenRoot, UWorldRoot, UiCameraRef, UiCanvasSize,
    UiRootSettlementState, UiSpace,
};
use self::roots::{RootResolutionState, RootSpawnRank};
pub use self::ui3d_sync::sync_cached_ui3d;

#[cfg(test)]
use self::root_stacking::root_capsule_band_width;
#[cfg(test)]
use self::roots::{LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT, ROOT_CAPSULE_LOCAL_LAYER_DIVISOR};
#[cfg(test)]
use self::screen_transform::compute_screen_root_transform;

pub(crate) fn resolve_root_ui(
    mut roots: Query<
        (
            Entity,
            Option<&URootUi>,
            Option<&UScreenRoot>,
            Option<&UWorldRoot>,
            &mut ResolvedRootUi,
            &mut RootResolutionState,
        ),
        Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>,
    >,
    cameras: Query<(Entity, &Camera)>,
    windows: Query<&Window>,
) {
    for (entity, root_ui, screen_root, world_root, mut resolved, mut state) in roots.iter_mut() {
        let Some(effective) = effective_root_ui(root_ui, screen_root, world_root) else {
            continue;
        };

        let (camera_entity, mut issue) = resolve_root_camera(effective.camera, &cameras);
        let canvas_size = resolve_root_canvas_size(
            effective.canvas,
            camera_entity,
            &cameras,
            &windows,
            &mut issue,
        );

        let next = ResolvedRootUi {
            root_entity: entity,
            space: effective.space,
            canvas: effective.canvas,
            canvas_size,
            camera_entity,
            meters_per_unit: effective.meters_per_unit,
            resolution_scale: effective.resolution_scale,
        };

        if *resolved != next {
            *resolved = next;
        }

        if state.issue != issue {
            emit_root_resolution_warning(entity, effective.space, issue);
            state.issue = issue;
        }
    }
}

pub(crate) fn assign_root_spawn_ranks(
    mut counter: ResMut<RootSpawnRankCounter>,
    mut roots: Query<&mut RootSpawnRank, Added<RootSpawnRank>>,
) {
    for mut rank in roots.iter_mut() {
        rank.0 = counter.next;
        counter.next += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::root_resolution::RootResolutionIssue;
    use super::*;
    use crate::internal_prelude::UI3d;
    use crate::prelude::UNode;
    use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy::ecs::system::SystemState;

    fn screen_root(canvas_size: Vec2) -> ResolvedRootUi {
        ResolvedRootUi {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            canvas_size,
            camera_entity: Some(Entity::PLACEHOLDER),
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        }
    }

    fn screen_stack(base: f32) -> ResolvedRootStack {
        ResolvedRootStack {
            capsule_band_base: base,
            capsule_sort_key: base,
            ..default()
        }
    }

    #[test]
    fn orthographic_screen_transform_tracks_camera_and_compensates_zoom() {
        let resolved = screen_root(Vec2::new(800.0, 600.0));
        let camera_transform = GlobalTransform::from(Transform {
            translation: Vec3::new(12.0, -4.0, 2.0),
            rotation: Quat::from_rotation_z(0.35),
            ..default()
        });
        let projection = OrthographicProjection {
            area: Rect::new(-800.0, -600.0, 800.0, 600.0),
            far: 1000.0,
            ..OrthographicProjection::default_2d()
        };

        let result = compute_screen_root_transform(
            &resolved,
            &screen_stack(0.0),
            &camera_transform,
            Some(&Projection::Orthographic(projection)),
        )
        .expect("screen roots should resolve an orthographic transform");

        assert!(
            result
                .translation
                .abs_diff_eq(Vec3::new(12.0, -4.0, -498.0), 1e-4)
        );
        assert!(
            result
                .rotation
                .abs_diff_eq(Quat::from_rotation_z(0.35), 1e-5)
        );
        assert!(result.scale.abs_diff_eq(Vec3::new(2.0, 2.0, 1.0), 1e-5));
    }

    #[test]
    fn orthographic_screen_transform_stays_in_front_of_3d_camera() {
        let resolved = screen_root(Vec2::new(800.0, 600.0));
        let camera_transform = GlobalTransform::from(Transform::default());
        let projection = OrthographicProjection {
            near: 0.1,
            far: 100.0,
            ..OrthographicProjection::default_3d()
        };

        let result = compute_screen_root_transform(
            &resolved,
            &screen_stack(0.0),
            &camera_transform,
            Some(&Projection::Orthographic(projection)),
        )
        .expect("screen roots should resolve an orthographic transform");

        assert!(result.translation.z < 0.0);
    }

    #[test]
    fn perspective_screen_transform_uses_camera_forward_and_frustum_size() {
        let resolved = screen_root(Vec2::new(200.0, 100.0));
        let camera_transform = GlobalTransform::from(Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            ..default()
        });
        let projection = PerspectiveProjection {
            fov: core::f32::consts::FRAC_PI_2,
            aspect_ratio: 2.0,
            near: 0.1,
            ..default()
        };

        let result = compute_screen_root_transform(
            &resolved,
            &screen_stack(0.0),
            &camera_transform,
            Some(&Projection::Perspective(projection)),
        )
        .expect("screen roots should resolve a perspective transform");

        assert!(
            result
                .translation
                .abs_diff_eq(Vec3::new(1.0, 2.0, 2.0), 1e-4)
        );
        assert!(result.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(result.scale.abs_diff_eq(Vec3::new(0.02, 0.02, 1.0), 1e-5));
    }

    #[test]
    fn sync_root_capsule_transforms_preserves_world_root_xy_rotation_and_scale() {
        let mut app = App::new();
        app.add_systems(Update, sync_root_capsule_transforms);

        let original = Transform::from_xyz(3.0, -2.0, 7.0);
        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(640.0, 360.0)),
                original,
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(640.0, 360.0)),
                    canvas_size: Vec2::new(640.0, 360.0),
                    camera_entity: None,
                    meters_per_unit: 0.25,
                    resolution_scale: 1.0,
                },
                ResolvedRootStack {
                    authored_root_z: original.translation.z,
                    capsule_sort_key: original.translation.z,
                    capsule_band_base: original.translation.z,
                    capsule_band_width: root_capsule_band_width(&ResolvedRootUi {
                        root_entity: Entity::PLACEHOLDER,
                        space: UiSpace::World2d,
                        canvas: UiCanvasSize::Fixed(Vec2::new(640.0, 360.0)),
                        canvas_size: Vec2::new(640.0, 360.0),
                        camera_entity: None,
                        meters_per_unit: 0.25,
                        resolution_scale: 1.0,
                    }),
                    capsule_band_step: root_capsule_band_width(&ResolvedRootUi {
                        root_entity: Entity::PLACEHOLDER,
                        space: UiSpace::World2d,
                        canvas: UiCanvasSize::Fixed(Vec2::new(640.0, 360.0)),
                        canvas_size: Vec2::new(640.0, 360.0),
                        camera_entity: None,
                        meters_per_unit: 0.25,
                        resolution_scale: 1.0,
                    }) / ROOT_CAPSULE_LOCAL_LAYER_DIVISOR,
                    applied_root_z: original.translation.z,
                    initialized: true,
                    ..default()
                },
            ))
            .id();

        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have resolved state")
            .root_entity = root;

        app.update();

        let transform = app
            .world()
            .entity(root)
            .get::<Transform>()
            .copied()
            .expect("root should keep a transform");

        assert!(
            transform
                .translation
                .abs_diff_eq(original.translation, 1e-5)
        );
        assert!(transform.rotation.abs_diff_eq(original.rotation, 1e-5));
        assert!(transform.scale.abs_diff_eq(original.scale, 1e-5));
    }

    #[test]
    fn resolve_root_stacking_packs_same_z_world_roots_by_spawn_order() {
        let mut app = App::new();
        app.init_resource::<RootSpawnRankCounter>();
        app.add_systems(
            Update,
            (
                assign_root_spawn_ranks,
                resolve_root_stacking,
                sync_root_capsule_transforms,
            )
                .chain(),
        );

        let resolved = ResolvedRootUi {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::World2d,
            canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
            canvas_size: Vec2::new(320.0, 180.0),
            camera_entity: None,
            meters_per_unit: 1.0,
            resolution_scale: 1.0,
        };

        let older = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::default(),
                resolved,
            ))
            .id();
        let newer = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::default(),
                resolved,
            ))
            .id();

        app.world_mut()
            .entity_mut(older)
            .get_mut::<ResolvedRootUi>()
            .expect("older root should have resolved ui")
            .root_entity = older;
        app.world_mut()
            .entity_mut(newer)
            .get_mut::<ResolvedRootUi>()
            .expect("newer root should have resolved ui")
            .root_entity = newer;

        app.update();

        let older_stack = *app
            .world()
            .entity(older)
            .get::<ResolvedRootStack>()
            .expect("older root should have a stack");
        let newer_stack = *app
            .world()
            .entity(newer)
            .get::<ResolvedRootStack>()
            .expect("newer root should have a stack");

        assert!(newer_stack.capsule_band_base > older_stack.capsule_ceiling());
    }

    #[test]
    fn resolve_root_stacking_preserves_authored_world_root_gaps_when_they_do_not_overlap() {
        let mut app = App::new();
        app.init_resource::<RootSpawnRankCounter>();
        app.add_systems(
            Update,
            (assign_root_spawn_ranks, resolve_root_stacking).chain(),
        );

        let lower = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
                    canvas_size: Vec2::new(320.0, 180.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();
        let upper = app
            .world_mut()
            .spawn((
                URootUi {
                    meters_per_unit: 1.0,
                    ..URootUi::world_2d(Vec2::new(320.0, 180.0))
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
                    canvas_size: Vec2::new(320.0, 180.0),
                    camera_entity: None,
                    meters_per_unit: 1.0,
                    resolution_scale: 1.0,
                },
            ))
            .id();

        app.world_mut()
            .entity_mut(lower)
            .get_mut::<ResolvedRootUi>()
            .expect("lower root should have resolved ui")
            .root_entity = lower;
        app.world_mut()
            .entity_mut(upper)
            .get_mut::<ResolvedRootUi>()
            .expect("upper root should have resolved ui")
            .root_entity = upper;

        app.update();

        let lower_stack = *app
            .world()
            .entity(lower)
            .get::<ResolvedRootStack>()
            .expect("lower root should have a stack");
        let upper_stack = *app
            .world()
            .entity(upper)
            .get::<ResolvedRootStack>()
            .expect("upper root should have a stack");

        assert!((lower_stack.capsule_band_base - 0.0).abs() <= 1e-5);
        assert!((upper_stack.capsule_band_base - 1.0).abs() <= 1e-5);
    }

    #[test]
    fn resolve_root_stacking_packs_same_z_screen_roots_by_spawn_order() {
        let mut app = App::new();
        app.init_resource::<RootSpawnRankCounter>();
        app.add_systems(
            Update,
            (assign_root_spawn_ranks, resolve_root_stacking).chain(),
        );

        let older = app
            .world_mut()
            .spawn((
                URootUi::screen(),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::Screen,
                    canvas: UiCanvasSize::Viewport,
                    canvas_size: Vec2::new(800.0, 600.0),
                    camera_entity: Some(Entity::PLACEHOLDER),
                    meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                    resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
                },
            ))
            .id();
        let newer = app
            .world_mut()
            .spawn((
                URootUi::screen(),
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::Screen,
                    canvas: UiCanvasSize::Viewport,
                    canvas_size: Vec2::new(800.0, 600.0),
                    camera_entity: Some(Entity::PLACEHOLDER),
                    meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                    resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
                },
            ))
            .id();

        app.world_mut()
            .entity_mut(older)
            .get_mut::<ResolvedRootUi>()
            .expect("older screen root should have resolved ui")
            .root_entity = older;
        app.world_mut()
            .entity_mut(newer)
            .get_mut::<ResolvedRootUi>()
            .expect("newer screen root should have resolved ui")
            .root_entity = newer;

        app.update();

        let older_stack = *app
            .world()
            .entity(older)
            .get::<ResolvedRootStack>()
            .expect("older screen root should have a stack");
        let newer_stack = *app
            .world()
            .entity(newer)
            .get::<ResolvedRootStack>()
            .expect("newer screen root should have a stack");

        assert!(newer_stack.capsule_band_base > older_stack.capsule_ceiling());
    }

    #[test]
    fn resolve_root_camera_auto_reports_missing_single_and_ambiguous_states() {
        let mut app = App::new();

        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());
        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Auto, &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::AutoMissingCamera);

        let expected = app.world_mut().spawn(Camera::default()).id();
        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());
        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Auto, &cameras);
        assert_eq!(camera_entity, Some(expected));
        assert_eq!(issue, RootResolutionIssue::None);

        app.world_mut().spawn(Camera::default());
        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());
        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Auto, &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::AutoAmbiguousCamera);
    }

    #[test]
    fn resolve_root_camera_reports_missing_and_inactive_explicit_targets() {
        let mut app = App::new();
        let inactive = app
            .world_mut()
            .spawn(Camera {
                is_active: false,
                ..default()
            })
            .id();

        let mut camera_state = SystemState::<Query<(Entity, &Camera)>>::new(app.world_mut());
        let cameras = camera_state.get(app.world());

        let (camera_entity, issue) =
            resolve_root_camera(UiCameraRef::Entity(Entity::PLACEHOLDER), &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::ExplicitCameraMissing);

        let (camera_entity, issue) = resolve_root_camera(UiCameraRef::Entity(inactive), &cameras);
        assert_eq!(camera_entity, None);
        assert_eq!(issue, RootResolutionIssue::ExplicitCameraInactive);
    }

    #[test]
    fn effective_root_ui_prefers_canonical_root_when_multiple_root_components_exist() {
        let canonical = URootUi {
            space: UiSpace::World3d,
            canvas: UiCanvasSize::Fixed(Vec2::new(320.0, 180.0)),
            camera: UiCameraRef::Entity(Entity::from_bits(42)),
            meters_per_unit: 2.5,
            resolution_scale: 1.25,
        };
        let legacy_world = UWorldRoot {
            size: Vec2::new(800.0, 600.0),
            is_3d: false,
            resolution_scale: 0.5,
        };

        let effective =
            effective_root_ui(Some(&canonical), Some(&UScreenRoot), Some(&legacy_world))
                .expect("canonical roots should resolve");

        assert_eq!(effective.space, UiSpace::World3d);
        assert_eq!(
            effective.canvas,
            UiCanvasSize::Fixed(Vec2::new(320.0, 180.0))
        );
        assert_eq!(effective.camera, UiCameraRef::Entity(Entity::from_bits(42)));
        assert_eq!(effective.meters_per_unit, 2.5);
        assert_eq!(effective.resolution_scale, 1.25);
    }

    #[test]
    fn effective_root_ui_maps_legacy_world_root_into_canonical_resolution_inputs() {
        let legacy_world = UWorldRoot {
            size: Vec2::new(512.0, 256.0),
            is_3d: true,
            resolution_scale: 1.5,
        };

        let effective = effective_root_ui(None, None, Some(&legacy_world))
            .expect("legacy world roots should resolve");

        assert_eq!(effective.space, UiSpace::World3d);
        assert_eq!(
            effective.canvas,
            UiCanvasSize::Fixed(Vec2::new(512.0, 256.0))
        );
        assert_eq!(effective.camera, UiCameraRef::Auto);
        assert_eq!(
            effective.meters_per_unit,
            LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT
        );
        assert_eq!(effective.resolution_scale, 1.5);
    }

    #[test]
    fn resolve_root_canvas_size_returns_fixed_size_verbatim() {
        let mut app = App::new();
        app.world_mut().spawn(Window {
            resolution: (1280, 720).into(),
            ..default()
        });

        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Fixed(Vec2::new(320.0, 240.0)),
            None,
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(320.0, 240.0));
        assert_eq!(issue, RootResolutionIssue::None);
    }

    #[test]
    fn resolve_root_canvas_size_reads_logical_viewport_size_from_camera() {
        let mut app = App::new();
        let camera = app
            .world_mut()
            .spawn(Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::ZERO,
                    physical_size: UVec2::new(600, 300),
                    depth: 0.0..1.0,
                }),
                computed: ComputedCameraValues {
                    target_info: Some(RenderTargetInfo {
                        physical_size: UVec2::new(1200, 900),
                        scale_factor: 1.0,
                    }),
                    ..default()
                },
                ..default()
            })
            .id();

        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Viewport,
            Some(camera),
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(600.0, 300.0));
        assert_eq!(issue, RootResolutionIssue::None);
    }

    #[test]
    fn resolve_root_canvas_size_falls_back_to_window_when_viewport_size_is_unavailable() {
        let mut app = App::new();
        app.world_mut().spawn(Window {
            resolution: (1280, 720).into(),
            ..default()
        });

        let missing_camera = app.world_mut().spawn_empty().id();

        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Viewport,
            Some(missing_camera),
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(1280.0, 720.0));
        assert_eq!(issue, RootResolutionIssue::ViewportSizeUnavailable);
    }

    #[test]
    fn resolve_root_canvas_size_uses_default_when_no_window_exists() {
        let mut app = App::new();

        let missing_camera = app.world_mut().spawn_empty().id();
        let mut state =
            SystemState::<(Query<(Entity, &Camera)>, Query<&Window>)>::new(app.world_mut());
        let (cameras, windows) = state.get(app.world());
        let mut issue = RootResolutionIssue::None;

        let size = resolve_root_canvas_size(
            UiCanvasSize::Viewport,
            Some(missing_camera),
            &cameras,
            &windows,
            &mut issue,
        );

        assert_eq!(size, Vec2::new(800.0, 600.0));
        assert_eq!(issue, RootResolutionIssue::ViewportSizeUnavailable);
    }

    fn world_root(space: UiSpace) -> (UWorldRoot, ResolvedRootUi) {
        (
            UWorldRoot {
                is_3d: matches!(space, UiSpace::World3d),
                ..default()
            },
            ResolvedRootUi {
                root_entity: Entity::PLACEHOLDER,
                space,
                canvas: UiCanvasSize::Fixed(Vec2::new(800.0, 600.0)),
                canvas_size: Vec2::new(800.0, 600.0),
                camera_entity: None,
                meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
                resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
            },
        )
    }

    #[test]
    fn cached_ui3d_is_added_for_world3d_roots_and_children() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();
        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        assert!(app.world().entity(root).contains::<UI3d>());
        assert!(app.world().entity(child).contains::<UI3d>());
    }

    #[test]
    fn cached_ui3d_is_removed_when_root_leaves_world3d() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();
        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        app.world_mut()
            .entity_mut(root)
            .get_mut::<ResolvedRootUi>()
            .expect("root should have a resolved context")
            .space = UiSpace::World2d;

        app.update();

        assert!(!app.world().entity(root).contains::<UI3d>());
        assert!(!app.world().entity(child).contains::<UI3d>());
    }

    #[test]
    fn cached_ui3d_is_applied_to_children_added_after_root_resolution() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();

        app.update();

        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        assert!(app.world().entity(child).contains::<UI3d>());
    }

    #[test]
    fn cached_ui3d_is_removed_when_child_detaches_from_world3d_root() {
        let mut app = App::new();
        app.add_systems(Update, sync_cached_ui3d);

        let (root_component, resolved) = world_root(UiSpace::World3d);
        let root = app
            .world_mut()
            .spawn((UNode::default(), root_component, resolved))
            .id();
        let child = app
            .world_mut()
            .spawn((UNode::default(), ChildOf(root)))
            .id();

        app.update();

        app.world_mut().entity_mut(child).remove::<ChildOf>();

        app.update();

        assert!(!app.world().entity(child).contains::<UI3d>());
    }
}
