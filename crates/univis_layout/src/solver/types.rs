//! Types and structures used by the layout solver.

use crate::geometry::{USides, UVal};
use crate::types::{
    UAlignItems, UAlignItemsExt, UAlignSelf, UAlignSelfExt, UContentAlignExt, UDisplay,
    UFlexDirection, UFlexWrap, UGridAutoFlow, UJustifyContent, UOverflowPosition, UPositionType,
    UTrackSize,
};
use glam::Vec2;

/// Represents the resolved sizing mode for the solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum SolverSizeMode {
    #[default]
    Fixed,
    Percent,
    Calc,
    MinContent,
    Content,
    Flex,
    Auto,
}

/// The output result of the solver for a single item.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct SolverResult {
    /// Calculated size in logical UI units.
    pub size: Vec2,
    /// Calculated local position in logical UI units.
    pub pos: Vec2,
}

/// A normalized specification of a node's layout properties for the solver.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolverSpec {
    pub width_mode: SolverSizeMode,
    pub width_val: f32,
    pub width_flex: f32,
    pub width_uval: UVal,
    pub min_width: f32,
    pub max_width: f32,
    pub height_mode: SolverSizeMode,
    pub height_val: f32,
    pub height_flex: f32,
    pub height_uval: UVal,
    pub min_height: f32,
    pub max_height: f32,
    pub aspect_ratio: Option<f32>,
    pub position_type: UPositionType,
    pub left: UVal,
    pub right: UVal,
    pub top: UVal,
    pub bottom: UVal,
    pub align_self: Option<UAlignSelf>,
    pub align_self_ext: Option<UAlignSelfExt>,
    pub justify_self_ext: Option<UAlignSelfExt>,
    pub justify_overflow: UOverflowPosition,
    pub align_overflow: UOverflowPosition,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<UVal>,
    pub grid_column_start: Option<u32>,
    pub grid_column_span: u32,
    pub grid_row_start: Option<u32>,
    pub grid_row_span: u32,
    pub order: i32,
}

impl Default for SolverSpec {
    fn default() -> Self {
        Self {
            width_mode: SolverSizeMode::Auto,
            width_val: 0.0,
            width_flex: 0.0,
            width_uval: UVal::Auto,
            min_width: 0.0,
            max_width: f32::INFINITY,
            height_mode: SolverSizeMode::Auto,
            height_val: 0.0,
            height_flex: 0.0,
            height_uval: UVal::Auto,
            min_height: 0.0,
            max_height: f32::INFINITY,
            aspect_ratio: None,
            position_type: UPositionType::Relative,
            left: UVal::Auto,
            right: UVal::Auto,
            top: UVal::Auto,
            bottom: UVal::Auto,
            align_self: None,
            align_self_ext: None,
            justify_self_ext: None,
            justify_overflow: UOverflowPosition::Unsafe,
            align_overflow: UOverflowPosition::Unsafe,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            grid_column_start: None,
            grid_column_span: 1,
            grid_row_start: None,
            grid_row_span: 1,
            order: 0,
        }
    }
}

/// An individual layout item processed by the solver.
///
/// Holds the item's input spec, margin, and the output result computed by the solver.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SolverItem {
    pub spec: SolverSpec,
    pub margin: USides,
    pub result: SolverResult,
}

impl SolverItem {
    /// Creates a new `SolverItem` with a default result.
    pub fn new(spec: SolverSpec, margin: USides) -> Self {
        Self {
            spec,
            margin,
            result: SolverResult::default(),
        }
    }

    /// Sets the initial result for this item.
    pub fn with_result(mut self, result: SolverResult) -> Self {
        self.result = result;
        self
    }
}

/// Configuration for the solver run (Container properties).
#[derive(Debug, Clone, PartialEq)]
pub struct SolverConfig {
    pub display: UDisplay,
    pub flex_direction: UFlexDirection,
    pub justify_content: UJustifyContent,
    pub align_items: UAlignItems,
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

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Start,
            gap: 0.0,
            row_gap: None,
            column_gap: None,
            padding: USides::default(),
            grid_columns: 1,
            justify_items: None,
            align_content: None,
            flex_wrap: UFlexWrap::NoWrap,
            flex_align_content: None,
            grid_template_columns: Vec::new(),
            grid_template_rows: Vec::new(),
            grid_auto_flow: UGridAutoFlow::Row,
            grid_auto_rows: UTrackSize::Auto,
            grid_auto_columns: UTrackSize::Auto,
            width_mode: SolverSizeMode::Auto,
            height_mode: SolverSizeMode::Auto,
        }
    }
}

/// Context passed to Placers to help position items.
#[derive(Debug, Clone)]
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
