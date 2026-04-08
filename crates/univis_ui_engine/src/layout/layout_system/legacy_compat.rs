use super::roots::LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT;
use super::{URootUi, UScreenRoot, UWorldRoot, UiCameraRef, UiCanvasSize, UiSpace};

/// Canonical root configuration consumed by the root-resolution pipeline.
///
/// Legacy wrappers are translated into this shape before camera and canvas
/// resolution so the rest of the pipeline can stay focused on one model.
#[derive(Clone, Copy)]
pub(super) struct EffectiveRootUi {
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}

fn canonical_root_ui(root: &URootUi) -> EffectiveRootUi {
    EffectiveRootUi {
        space: root.space,
        canvas: root.canvas,
        camera: root.camera,
        meters_per_unit: root.meters_per_unit,
        resolution_scale: root.resolution_scale,
    }
}

fn legacy_world_root_ui(root: &UWorldRoot) -> EffectiveRootUi {
    EffectiveRootUi {
        space: if root.is_3d {
            UiSpace::World3d
        } else {
            UiSpace::World2d
        },
        canvas: UiCanvasSize::Fixed(root.size),
        camera: UiCameraRef::Auto,
        // `UWorldRoot` preserves the historical 1 UI unit = 1 world unit behavior.
        meters_per_unit: LEGACY_WORLD_ROOT_UNITS_PER_UI_UNIT,
        resolution_scale: root.resolution_scale,
    }
}

fn legacy_screen_root_ui() -> EffectiveRootUi {
    EffectiveRootUi {
        space: UiSpace::Screen,
        canvas: UiCanvasSize::Viewport,
        camera: UiCameraRef::Auto,
        meters_per_unit: URootUi::DEFAULT_METERS_PER_UNIT,
        resolution_scale: URootUi::DEFAULT_RESOLUTION_SCALE,
    }
}

pub(super) fn effective_root_ui(
    root_ui: Option<&URootUi>,
    screen_root: Option<&UScreenRoot>,
    world_root: Option<&UWorldRoot>,
) -> Option<EffectiveRootUi> {
    if let Some(root) = root_ui {
        return Some(canonical_root_ui(root));
    }

    if let Some(root) = world_root {
        return Some(legacy_world_root_ui(root));
    }

    screen_root.map(|_| legacy_screen_root_ui())
}
