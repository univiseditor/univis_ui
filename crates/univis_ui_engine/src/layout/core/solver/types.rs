use crate::internal_prelude::*;
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

/// The output result of the solver for a single item.
#[derive(Debug, Clone, Copy, Default)]
pub struct SolverResult {
    pub size: Vec2,
    pub pos: Vec2,
}

#[derive(Clone, Copy)]
pub struct SolverResultHandle(*mut SolverResult);

// SAFETY: `SolverResultHandle` only appears inside short-lived solver data that
// stays scoped to one system run. The raw pointer always targets a result slot
// owned by that same run, and we never share it across concurrent threads.
unsafe impl Send for SolverResultHandle {}
unsafe impl Sync for SolverResultHandle {}

impl SolverResultHandle {
    pub fn new(result: &mut SolverResult) -> Self {
        Self(result as *mut SolverResult)
    }
}

impl Deref for SolverResultHandle {
    type Target = SolverResult;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `SolverResultHandle` is only created from a live mutable
        // reference, and callers keep ownership of the backing `SolverResult`
        // for the full solver pass where this handle is used.
        unsafe { &*self.0 }
    }
}

impl DerefMut for SolverResultHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: same reasoning as `Deref`; solver passes treat each handle
        // as the unique mutable access path to its backing result slot.
        unsafe { &mut *self.0 }
    }
}

/// Combines spec, result, and margin for processing.
pub struct SolverItem {
    pub spec: SolverSpec,
    pub result: SolverResultHandle,
    pub margin: USides,
}

impl SolverItem {
    pub fn new(spec: SolverSpec, result: &mut SolverResult, margin: USides) -> Self {
        Self {
            spec,
            result: SolverResultHandle::new(result),
            margin,
        }
    }
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
