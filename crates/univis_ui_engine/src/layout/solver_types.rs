use bevy::prelude::*;

use crate::layout::geometry::UVal;
use crate::layout::univis_node::{
    UAlignItemsExt, UAlignSelf, UAlignSelfExt, UContentAlignExt, UOverflowPosition, UPositionType,
};

/// Represents the resolved sizing mode for the solver.
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
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

/// A normalized specification of a node's layout properties for the solver.
#[derive(Debug, Clone, Copy)]
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

/// Configuration fragment shared between item and container alignment code paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SolverAlignConfig {
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
}
