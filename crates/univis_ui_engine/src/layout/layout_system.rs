#![allow(deprecated)]

use bevy::prelude::*;

use crate::internal_prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum UiSpace {
    Screen,
    World2d,
    World3d,
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum UiCanvasSize {
    Viewport,
    Fixed(Vec2),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum UiCameraRef {
    Auto,
    Entity(Entity),
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
#[require(UNode, ResolvedRootUi, RootResolutionState)]
pub struct URootUi {
    /// Where the UI lives once projected into the scene.
    pub space: UiSpace,
    /// The logical canvas size used by layout.
    pub canvas: UiCanvasSize,
    /// Which camera resolves viewport-bound roots and interaction context.
    pub camera: UiCameraRef,
    /// Physical world size per logical UI unit for world-space roots.
    pub meters_per_unit: f32,
    /// Rendering quality multiplier that stays independent from physical world size.
    pub resolution_scale: f32,
}

impl URootUi {
    pub const DEFAULT_METERS_PER_UNIT: f32 = 0.001;
    pub const DEFAULT_RESOLUTION_SCALE: f32 = 1.0;

    /// Creates a screen-space root that behaves like a HUD tied to the resolved viewport.
    pub fn screen() -> Self {
        Self {
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    /// Creates a world-space 2D root with a fixed logical canvas size.
    ///
    /// The default `meters_per_unit` is `0.001`, so the physical world size is:
    /// `world_size = canvas_size * meters_per_unit`.
    pub fn world_2d(size: Vec2) -> Self {
        Self {
            space: UiSpace::World2d,
            canvas: UiCanvasSize::Fixed(size),
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    /// Creates a world-space 3D root with a fixed logical canvas size.
    ///
    /// The default `meters_per_unit` is `0.001`, so the physical world size is:
    /// `world_size = canvas_size * meters_per_unit`.
    pub fn world_3d(size: Vec2) -> Self {
        Self {
            space: UiSpace::World3d,
            canvas: UiCanvasSize::Fixed(size),
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }
}

const LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT: f32 = 1.0;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ResolvedRootUi {
    pub root_entity: Entity,
    /// Resolved UI space for this root.
    pub space: UiSpace,
    /// Final logical canvas size after resolving viewport or fixed sizing.
    pub canvas_size: Vec2,
    pub camera_entity: Option<Entity>,
    /// Physical world size per logical UI unit for world-space roots.
    pub meters_per_unit: f32,
    /// Rendering quality multiplier that stays independent from physical world size.
    pub resolution_scale: f32,
}

impl Default for ResolvedRootUi {
    fn default() -> Self {
        Self {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::Screen,
            canvas_size: Vec2::ZERO,
            camera_entity: None,
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
        }
    }
}

impl ResolvedRootUi {
    /// Returns how many world units correspond to one logical UI unit for this root.
    pub fn ui_units_to_world_scale(&self) -> f32 {
        match self.space {
            UiSpace::Screen => 1.0,
            UiSpace::World2d | UiSpace::World3d => normalize_meters_per_unit(self.meters_per_unit),
        }
    }

    /// Converts a logical UI size to its physical size in world units for this root.
    pub fn world_size_for(&self, logical_size: Vec2) -> Vec2 {
        logical_size * self.ui_units_to_world_scale()
    }

    /// Converts a logical scalar to world units for this root.
    pub fn world_scalar_for(&self, logical_value: f32) -> f32 {
        logical_value * self.ui_units_to_world_scale()
    }
}

/// Marker for the Screen Root node (HUD).
///
/// Deprecated compatibility wrapper during `alpha2`.
/// Prefer `URootUi::screen()` for new code.
#[deprecated(
    since = "0.2.0-alpha.2",
    note = "Use `URootUi::screen()` instead of `UScreenRoot`."
)]
#[derive(Component, Default)]
#[require(UNode, ResolvedRootUi, RootResolutionState)]
pub struct UScreenRoot;

/// Marker for World Space UI Root.
///
/// Legacy compatibility wrapper during the `alpha2` migration.
/// Prefer `URootUi::world_2d(size)` or `URootUi::world_3d(size)` for new code.
#[deprecated(
    since = "0.2.0-alpha.2",
    note = "Use `URootUi::world_2d(size)` or `URootUi::world_3d(size)` instead. If you need the exact legacy `UWorldRoot` physical sizing during alpha2, set `meters_per_unit = 1.0` explicitly on `URootUi`."
)]
#[derive(Component)]
#[require(UNode, ResolvedRootUi, RootResolutionState)]
pub struct UWorldRoot {
    pub size: Vec2,
    /// Legacy compatibility flag. New code should express this through `UiSpace`.
    pub is_3d: bool,
    /// Scaling factor for text resolution/quality relative to the size.
    pub resolution_scale: f32,
}

impl Default for UWorldRoot {
    fn default() -> Self {
        Self {
            size: Vec2::new(800.0, 600.0),
            is_3d: false,
            resolution_scale: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum RootResolutionIssue {
    #[default]
    None,
    AutoMissingCamera,
    AutoAmbiguousCamera,
    ExplicitCameraMissing,
    ExplicitCameraInactive,
    ViewportSizeUnavailable,
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RootResolutionState {
    issue: RootResolutionIssue,
}

#[derive(Clone, Copy)]
struct EffectiveRootUi {
    space: UiSpace,
    canvas: UiCanvasSize,
    camera: UiCameraRef,
    meters_per_unit: f32,
    resolution_scale: f32,
}

const PERSPECTIVE_SCREEN_DISTANCE: f32 = 1.0;

fn normalize_meters_per_unit(value: f32) -> f32 {
    if value.is_finite() && value > f32::EPSILON {
        value
    } else {
        URootUi::DEFAULT_METERS_PER_UNIT
    }
}

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

pub(crate) fn sync_screen_roots_to_camera(
    mut roots: Query<(&ResolvedRootUi, &mut Transform), Or<(With<UScreenRoot>, With<URootUi>)>>,
    cameras: Query<(&GlobalTransform, Option<&Projection>), With<Camera>>,
) {
    for (resolved, mut transform) in roots.iter_mut() {
        if resolved.space != UiSpace::Screen {
            continue;
        }

        let Some(camera_entity) = resolved.camera_entity else {
            continue;
        };
        let Ok((camera_transform, projection)) = cameras.get(camera_entity) else {
            continue;
        };
        let Some(next_transform) =
            compute_screen_root_transform(resolved, camera_transform, projection)
        else {
            continue;
        };

        if *transform != next_transform {
            *transform = next_transform;
        }
    }
}

pub fn sync_cached_ui3d(
    mut commands: Commands,
    roots_changed: Query<
        (),
        (
            Changed<ResolvedRootUi>,
            Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>,
        ),
    >,
    all_nodes: Query<(Entity, Has<UI3d>), Or<(With<UNode>, With<UI3d>)>>,
    incremental_nodes: Query<
        (Entity, Has<UI3d>),
        Or<(Added<UNode>, Added<UI3d>, Changed<ChildOf>)>,
    >,
    mut removed_children: RemovedComponents<ChildOf>,
    parents_query: Query<&ChildOf>,
    root_query: Query<&ResolvedRootUi, Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>>,
) {
    if roots_changed.is_empty() {
        for (entity, has_ui3d) in incremental_nodes.iter() {
            sync_cached_ui3d_for_entity(
                entity,
                has_ui3d,
                &parents_query,
                &root_query,
                &mut commands,
            );
        }

        for entity in removed_children.read() {
            let Ok((_, has_ui3d)) = all_nodes.get(entity) else {
                continue;
            };

            sync_cached_ui3d_for_entity(
                entity,
                has_ui3d,
                &parents_query,
                &root_query,
                &mut commands,
            );
        }
    } else {
        for (entity, has_ui3d) in all_nodes.iter() {
            sync_cached_ui3d_for_entity(
                entity,
                has_ui3d,
                &parents_query,
                &root_query,
                &mut commands,
            );
        }
    }
}

fn sync_cached_ui3d_for_entity(
    entity: Entity,
    has_ui3d: bool,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi, Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>>,
    commands: &mut Commands,
) {
    let should_have_ui3d = resolve_root_for_ui3d(entity, parents_query, root_query)
        .map(|root| matches!(root.space, UiSpace::World3d))
        .unwrap_or(false);

    match (should_have_ui3d, has_ui3d) {
        (true, false) => {
            commands.entity(entity).insert(UI3d);
        }
        (false, true) => {
            commands.entity(entity).remove::<UI3d>();
        }
        _ => {}
    }
}

fn resolve_root_for_ui3d(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi, Or<(With<URootUi>, With<UScreenRoot>, With<UWorldRoot>)>>,
) -> Option<ResolvedRootUi> {
    let mut current = entity;

    loop {
        if let Ok(root) = root_query.get(current) {
            return Some(*root);
        }

        let parent = parents_query.get(current).ok()?;
        current = parent.parent();
    }
}

fn effective_root_ui(
    root_ui: Option<&URootUi>,
    screen_root: Option<&UScreenRoot>,
    world_root: Option<&UWorldRoot>,
) -> Option<EffectiveRootUi> {
    if let Some(root) = root_ui {
        return Some(EffectiveRootUi {
            space: root.space,
            canvas: root.canvas,
            camera: root.camera,
            meters_per_unit: root.meters_per_unit,
            resolution_scale: root.resolution_scale,
        });
    }

    if let Some(root) = world_root {
        return Some(EffectiveRootUi {
            space: if root.is_3d {
                UiSpace::World3d
            } else {
                UiSpace::World2d
            },
            canvas: UiCanvasSize::Fixed(root.size),
            camera: UiCameraRef::Auto,
            // `UWorldRoot` stays on the old 1 UI unit = 1 world unit behavior during alpha2.
            meters_per_unit: LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT,
            resolution_scale: root.resolution_scale,
        });
    }

    screen_root.map(|_| EffectiveRootUi {
        space: UiSpace::Screen,
        canvas: UiCanvasSize::Viewport,
        camera: UiCameraRef::Auto,
        meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
        resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
    })
}

fn resolve_root_camera(
    camera_ref: UiCameraRef,
    cameras: &Query<(Entity, &Camera)>,
) -> (Option<Entity>, RootResolutionIssue) {
    match camera_ref {
        UiCameraRef::Entity(entity) => match cameras.get(entity) {
            Ok((_, camera)) if camera.is_active => (Some(entity), RootResolutionIssue::None),
            Ok(_) => (None, RootResolutionIssue::ExplicitCameraInactive),
            Err(_) => (None, RootResolutionIssue::ExplicitCameraMissing),
        },
        UiCameraRef::Auto => {
            let mut candidates = cameras
                .iter()
                .filter_map(|(entity, camera)| camera.is_active.then_some(entity));

            let first = candidates.next();
            let second = candidates.next();

            match (first, second) {
                (Some(entity), None) => (Some(entity), RootResolutionIssue::None),
                (Some(_), Some(_)) => (None, RootResolutionIssue::AutoAmbiguousCamera),
                _ => (None, RootResolutionIssue::AutoMissingCamera),
            }
        }
    }
}

fn resolve_root_canvas_size(
    canvas: UiCanvasSize,
    camera_entity: Option<Entity>,
    cameras: &Query<(Entity, &Camera)>,
    windows: &Query<&Window>,
    issue: &mut RootResolutionIssue,
) -> Vec2 {
    match canvas {
        UiCanvasSize::Fixed(size) => size,
        UiCanvasSize::Viewport => {
            if let Some(camera_entity) = camera_entity {
                if let Ok((_, camera)) = cameras.get(camera_entity) {
                    if let Some(size) = camera.logical_viewport_size() {
                        return size;
                    }
                }

                if *issue == RootResolutionIssue::None {
                    *issue = RootResolutionIssue::ViewportSizeUnavailable;
                }
            }

            if let Ok(window) = windows.single() {
                Vec2::new(window.width(), window.height())
            } else {
                Vec2::new(800.0, 600.0)
            }
        }
    }
}

fn emit_root_resolution_warning(entity: Entity, space: UiSpace, issue: RootResolutionIssue) {
    match issue {
        RootResolutionIssue::None => {}
        RootResolutionIssue::AutoMissingCamera => warn!(
            "URootUi on entity {:?} ({:?}) could not resolve `UiCameraRef::Auto`: no active compatible camera found.",
            entity, space
        ),
        RootResolutionIssue::AutoAmbiguousCamera => warn!(
            "URootUi on entity {:?} ({:?}) could not resolve `UiCameraRef::Auto`: multiple active compatible cameras were found. Bind `UiCameraRef::Entity` explicitly.",
            entity, space
        ),
        RootResolutionIssue::ExplicitCameraMissing => warn!(
            "URootUi on entity {:?} ({:?}) references a camera entity that does not exist or does not have a `Camera` component.",
            entity, space
        ),
        RootResolutionIssue::ExplicitCameraInactive => warn!(
            "URootUi on entity {:?} ({:?}) references an inactive camera.",
            entity, space
        ),
        RootResolutionIssue::ViewportSizeUnavailable => warn!(
            "URootUi on entity {:?} ({:?}) could not read a logical viewport size from the resolved camera yet.",
            entity, space
        ),
    }
}

fn compute_screen_root_transform(
    resolved: &ResolvedRootUi,
    camera_transform: &GlobalTransform,
    projection: Option<&Projection>,
) -> Option<Transform> {
    if resolved.space != UiSpace::Screen {
        return None;
    }

    let canvas_size = Vec2::new(
        resolved.canvas_size.x.max(1.0),
        resolved.canvas_size.y.max(1.0),
    );
    let camera_transform = camera_transform.compute_transform();

    // Screen roots inherit the camera pose so camera movement and rotation cancel out in view
    // space. The scale is then derived from the projection so the logical canvas stays visually
    // stable on screen instead of behaving like an ordinary world canvas.
    match projection {
        Some(Projection::Orthographic(orthographic)) => Some(Transform {
            translation: camera_transform.translation
                + camera_transform.forward().as_vec3() * orthographic_screen_distance(orthographic),
            rotation: camera_transform.rotation,
            scale: orthographic_screen_scale(orthographic, canvas_size),
        }),
        Some(Projection::Perspective(perspective)) => {
            let distance = perspective_screen_distance(perspective);
            let frustum_size = perspective_screen_frustum_size(perspective, canvas_size, distance);

            Some(Transform {
                translation: camera_transform.translation
                    + camera_transform.forward().as_vec3() * distance,
                rotation: camera_transform.rotation,
                scale: Vec3::new(
                    frustum_size.x / canvas_size.x,
                    frustum_size.y / canvas_size.y,
                    1.0,
                ),
            })
        }
        _ => Some(Transform {
            translation: camera_transform.translation,
            rotation: camera_transform.rotation,
            scale: Vec3::ONE,
        }),
    }
}

fn orthographic_screen_distance(projection: &OrthographicProjection) -> f32 {
    let distance = if projection.near >= 0.0 {
        (projection.near + projection.far) * 0.5
    } else {
        projection.far * 0.5
    };

    if distance <= f32::EPSILON {
        1.0
    } else {
        distance
    }
}

fn orthographic_screen_scale(projection: &OrthographicProjection, canvas_size: Vec2) -> Vec3 {
    let world_size = projection.area.size();
    Vec3::new(
        world_size.x / canvas_size.x,
        world_size.y / canvas_size.y,
        1.0,
    )
}

fn perspective_screen_distance(projection: &PerspectiveProjection) -> f32 {
    (projection.near * 4.0).max(PERSPECTIVE_SCREEN_DISTANCE)
}

fn perspective_screen_frustum_size(
    projection: &PerspectiveProjection,
    canvas_size: Vec2,
    distance: f32,
) -> Vec2 {
    let aspect = if canvas_size.y > 0.0 {
        canvas_size.x / canvas_size.y
    } else {
        projection.aspect_ratio.max(f32::EPSILON)
    };
    let height = 2.0 * distance * (projection.fov * 0.5).tan();
    Vec2::new(height * aspect, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy::ecs::system::SystemState;

    fn screen_root(canvas_size: Vec2) -> ResolvedRootUi {
        ResolvedRootUi {
            root_entity: Entity::PLACEHOLDER,
            space: UiSpace::Screen,
            canvas_size,
            camera_entity: Some(Entity::PLACEHOLDER),
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
            resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
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
            &camera_transform,
            Some(&Projection::Orthographic(projection)),
        )
        .expect("screen roots should resolve an orthographic transform");

        assert!(result
            .translation
            .abs_diff_eq(Vec3::new(12.0, -4.0, -498.0), 1e-4));
        assert!(result
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(0.35), 1e-5));
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
            &camera_transform,
            Some(&Projection::Perspective(projection)),
        )
        .expect("screen roots should resolve a perspective transform");

        assert!(result
            .translation
            .abs_diff_eq(Vec3::new(1.0, 2.0, 2.0), 1e-4));
        assert!(result.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(result.scale.abs_diff_eq(Vec3::new(0.02, 0.02, 1.0), 1e-5));
    }

    #[test]
    fn sync_screen_root_transforms_leaves_world_roots_attached_to_world_transforms() {
        let mut app = App::new();
        app.add_systems(Update, sync_screen_roots_to_camera);

        let original = Transform::from_xyz(3.0, -2.0, 7.0);
        let root = app
            .world_mut()
            .spawn((
                URootUi::world_2d(Vec2::new(640.0, 360.0)),
                original,
                ResolvedRootUi {
                    root_entity: Entity::PLACEHOLDER,
                    space: UiSpace::World2d,
                    canvas_size: Vec2::new(640.0, 360.0),
                    camera_entity: None,
                    meters_per_unit: 0.25,
                    resolution_scale: 1.0,
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

        assert!(transform
            .translation
            .abs_diff_eq(original.translation, 1e-5));
        assert!(transform.rotation.abs_diff_eq(original.rotation, 1e-5));
        assert!(transform.scale.abs_diff_eq(original.scale, 1e-5));
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
