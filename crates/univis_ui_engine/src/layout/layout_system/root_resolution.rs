use bevy::prelude::*;

use super::{
    LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT, URootUi, UScreenRoot, UWorldRoot, UiCameraRef,
    UiCanvasSize, UiSpace,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum RootResolutionIssue {
    #[default]
    None,
    AutoMissingCamera,
    AutoAmbiguousCamera,
    ExplicitCameraMissing,
    ExplicitCameraInactive,
    ViewportSizeUnavailable,
}

#[derive(Clone, Copy)]
pub(super) struct EffectiveRootUi {
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}

pub(super) fn normalize_meters_per_unit(value: f32) -> f32 {
    if value.is_finite() && value > f32::EPSILON {
        value
    } else {
        URootUi::DEFAULT_METERS_PER_UNIT
    }
}

pub(super) fn effective_root_ui(
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

pub(super) fn resolve_root_camera(
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

pub(super) fn resolve_root_canvas_size(
    canvas: UiCanvasSize,
    camera_entity: Option<Entity>,
    cameras: &Query<(Entity, &Camera)>,
    windows: &Query<&Window>,
    issue: &mut RootResolutionIssue,
) -> Vec2 {
    match canvas {
        UiCanvasSize::Fixed(size) => size,
        UiCanvasSize::FitContent { min, max } => clamp_canvas_size(min.max(Vec2::ZERO), min, max),
        UiCanvasSize::Viewport => {
            if let Some(camera_entity) = camera_entity {
                if let Ok((_, camera)) = cameras.get(camera_entity)
                    && let Some(size) = camera.logical_viewport_size()
                {
                    return size;
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

pub(super) fn clamp_canvas_size(size: Vec2, min: Vec2, max: Option<Vec2>) -> Vec2 {
    let mut clamped = Vec2::new(size.x.max(min.x), size.y.max(min.y));

    if let Some(max) = max {
        clamped.x = clamped.x.min(max.x);
        clamped.y = clamped.y.min(max.y);
    }

    clamped
}

pub(super) fn emit_root_resolution_warning(
    entity: Entity,
    space: UiSpace,
    issue: RootResolutionIssue,
) {
    match issue {
        RootResolutionIssue::None => {}
        RootResolutionIssue::AutoMissingCamera => bevy::log::warn!(
            "URootUi on entity {:?} ({:?}) could not resolve `UiCameraRef::Auto`: no active compatible camera found.",
            entity,
            space
        ),
        RootResolutionIssue::AutoAmbiguousCamera => bevy::log::warn!(
            "URootUi on entity {:?} ({:?}) could not resolve `UiCameraRef::Auto`: multiple active compatible cameras were found. Bind `UiCameraRef::Entity` explicitly.",
            entity,
            space
        ),
        RootResolutionIssue::ExplicitCameraMissing => bevy::log::warn!(
            "URootUi on entity {:?} ({:?}) references a camera entity that does not exist or does not have a `Camera` component.",
            entity,
            space
        ),
        RootResolutionIssue::ExplicitCameraInactive => bevy::log::warn!(
            "URootUi on entity {:?} ({:?}) references an inactive camera.",
            entity,
            space
        ),
        RootResolutionIssue::ViewportSizeUnavailable => bevy::log::warn!(
            "URootUi on entity {:?} ({:?}) could not read a logical viewport size from the resolved camera yet.",
            entity,
            space
        ),
    }
}
