use crate::internal_prelude::*;
use bevy::prelude::*;

/// The output result of the solver for a single item.
#[derive(Debug, Clone, Copy, Default)]
pub struct SolverResult {
    pub size: Vec2,
    pub pos: Vec2,
}

/// Combines spec, result, and margin for processing.
pub struct SolverItem<'a> {
    pub spec: SolverSpec,
    pub result: &'a mut SolverResult,
    pub margin: USides,
}

/// Configuration for the solver run (Container properties).
#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub layout: ULayout,
    pub gap: f32,
    pub row_gap: Option<f32>,
    pub column_gap: Option<f32>,
    pub padding: USides,
    pub grid_columns: u32,
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
    pub flex_wrap: UFlexWrap,
    pub flex_align_content: Option<UContentAlignExt>,
    pub grid_template_columns: Vec<UTrackSize>,
    pub grid_template_rows: Vec<UTrackSize>,
    pub grid_auto_flow: UGridAutoFlow,
    pub grid_auto_rows: UTrackSize,
    pub grid_auto_columns: UTrackSize,

    // Width/Height modes to determine sizing constraints
    pub width_mode: SolverSizeMode,
    pub height_mode: SolverSizeMode,
}

/// Context passed to Placers to help position items.
pub struct PlacementContext {
    pub container_main_size: f32,
    pub container_cross_size: f32,
    pub padding_main_start: f32,
    pub padding_main_end: f32,
    pub padding_cross_start: f32,

    pub gap: f32,
    pub main_gap: f32,
    pub cross_gap: f32,
    pub justify_content: UJustifyContent,
    pub align_items: UAlignItems,
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
    pub flex_wrap: UFlexWrap,
    pub grid_columns: u32,
    pub grid_template_columns: Vec<UTrackSize>,
    pub grid_template_rows: Vec<UTrackSize>,
    pub grid_auto_flow: UGridAutoFlow,
    pub grid_auto_rows: UTrackSize,
    pub grid_auto_columns: UTrackSize,
}
