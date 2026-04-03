use bevy::prelude::*;

use super::{ResolvedRootStack, ResolvedRootUi, UiSpace};

const PERSPECTIVE_SCREEN_DISTANCE: f32 = 1.0;

pub(super) fn compute_screen_root_transform(
    resolved: &ResolvedRootUi,
    stack: &ResolvedRootStack,
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
    let stacking_offset = camera_transform.back().as_vec3() * stack.capsule_band_base;

    match projection {
        Some(Projection::Orthographic(orthographic)) => Some(Transform {
            translation: camera_transform.translation
                + camera_transform.forward().as_vec3() * orthographic_screen_distance(orthographic)
                + stacking_offset,
            rotation: camera_transform.rotation,
            scale: orthographic_screen_scale(orthographic, canvas_size),
        }),
        Some(Projection::Perspective(perspective)) => {
            let distance = perspective_screen_distance(perspective);
            let frustum_size = perspective_screen_frustum_size(perspective, canvas_size, distance);

            Some(Transform {
                translation: camera_transform.translation
                    + camera_transform.forward().as_vec3() * distance
                    + stacking_offset,
                rotation: camera_transform.rotation,
                scale: Vec3::new(
                    frustum_size.x / canvas_size.x,
                    frustum_size.y / canvas_size.y,
                    1.0,
                ),
            })
        }
        _ => Some(Transform {
            translation: camera_transform.translation + stacking_offset,
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
