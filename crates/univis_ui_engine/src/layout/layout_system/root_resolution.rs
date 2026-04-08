use bevy::prelude::*;

use super::{URootUi, UiCameraRef, UiCanvasSize, UiSpace};

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

impl RootResolutionIssue {
    fn reason(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::AutoMissingCamera => {
                Some("no active compatible camera matched `UiCameraRef::Auto`")
            }
            Self::AutoAmbiguousCamera => {
                Some("multiple active compatible cameras matched `UiCameraRef::Auto`")
            }
            Self::ExplicitCameraMissing => {
                Some("the configured camera entity is missing or does not contain `Camera`")
            }
            Self::ExplicitCameraInactive => Some("the configured camera entity is inactive"),
            Self::ViewportSizeUnavailable => {
                Some("logical viewport size was not available from the resolved camera")
            }
        }
    }

    fn action(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::AutoMissingCamera => {
                Some("spawn one active compatible camera or bind `UiCameraRef::Entity` explicitly")
            }
            Self::AutoAmbiguousCamera => {
                Some("bind `UiCameraRef::Entity` explicitly for this root")
            }
            Self::ExplicitCameraMissing => Some("update the root to point at a live camera entity"),
            Self::ExplicitCameraInactive => {
                Some("activate the target camera or bind a different active camera")
            }
            Self::ViewportSizeUnavailable => {
                Some("wait for the camera viewport to initialize or provide a fixed canvas size")
            }
        }
    }
}

pub(super) fn normalize_meters_per_unit(value: f32) -> f32 {
    if value.is_finite() && value > f32::EPSILON {
        value
    } else {
        URootUi::DEFAULT_METERS_PER_UNIT
    }
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
    let Some(reason) = issue.reason() else {
        return;
    };
    let action = issue.action().unwrap_or("inspect the root configuration");

    bevy::log::warn!(
        "[layout/root_resolution] root={:?} space={:?} reason={} action={}",
        entity,
        space,
        reason,
        action
    );
}
