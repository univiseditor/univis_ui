use bevy::{ecs::relationship::Relationship, prelude::*};

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
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}

impl URootUi {
    pub const DEFAULT_METERS_PER_UNIT: f32 = 0.001;
    pub const DEFAULT_RESOLUTION_SCALE: f32 = 1.0;

    pub fn screen() -> Self {
        Self {
            space: UiSpace::Screen,
            canvas: UiCanvasSize::Viewport,
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

    pub fn world_2d(size: Vec2) -> Self {
        Self {
            space: UiSpace::World2d,
            canvas: UiCanvasSize::Fixed(size),
            camera: UiCameraRef::Auto,
            meters_per_unit: Self::DEFAULT_METERS_PER_UNIT,
            resolution_scale: Self::DEFAULT_RESOLUTION_SCALE,
        }
    }

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

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ResolvedRootUi {
    pub root_entity: Entity,
    pub space: UiSpace,
    pub canvas_size: Vec2,
    pub camera_entity: Option<Entity>,
    pub meters_per_unit: f32,
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

/// Marker for the Screen Root node (HUD).
///
/// Use this for UI that stays fixed to the camera/screen.
/// It typically takes its size automatically from the window dimensions.
#[derive(Component, Default)]
#[require(UNode, ResolvedRootUi, RootResolutionState)]
pub struct UScreenRoot;

/// Marker for World Space UI Root.
///
/// Legacy compatibility wrapper during the `alpha2` migration.
/// Prefer `URootUi::world_2d(size)` or `URootUi::world_3d(size)` for new code.
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
    mut roots: Query<
        (&ResolvedRootUi, &mut Transform),
        Or<(With<UScreenRoot>, With<URootUi>)>,
    >,
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

pub fn auto_propagate_ui3d(
    mut commands: Commands,

    // 1. مراقبة الجذور (Roots) التي تغيرت إعداداتها
    root_query: Query<(Entity, &UWorldRoot), (Changed<UWorldRoot>, Without<UI3d>)>,

    // 2. مراقبة الأبناء (Children) الذين ليس لديهم UI3d بعد
    // نبحث عن أي UNode له أب، ولكن ينقصه مكون UI3d
    child_query: Query<(Entity, &ChildOf), (With<UNode>, Without<UI3d>)>,

    // 3. استعلام للتحقق مما إذا كان الأب يمتلك UI3d
    parent_check: Query<&UI3d>,
) {
    // أ) معالجة الجذر: هل طلب المستخدم وضع 3D؟
    for (entity, root) in root_query.iter() {
        if root.is_3d {
            commands.entity(entity).insert(UI3d);
        }
    }

    // ب) التوريث المتسلسل:
    // نمر على كل الأبناء الذين ليس لديهم UI3d
    for (child_entity, parent) in child_query.iter() {
        // إذا كان "الأب" يمتلك العلامة UI3d
        if parent_check.get(parent.get()).is_ok() {
            // إذن "الابن" يجب أن يصبح 3D أيضاً
            commands.entity(child_entity).insert(UI3d);
        }
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
            meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
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
        assert!(result.rotation.abs_diff_eq(Quat::from_rotation_z(0.35), 1e-5));
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
}
