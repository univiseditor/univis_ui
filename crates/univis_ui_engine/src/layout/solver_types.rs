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
    pub height_mode: SolverSizeMode,
    pub height_val: f32,
    pub height_flex: f32,
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

/// Configuration fragment shared between item and container alignment code paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SolverAlignConfig {
    pub justify_items: Option<UAlignItemsExt>,
    pub align_content: Option<UContentAlignExt>,
}
