use crate::layout::layout_system::{ResolvedRootStack, UiSpace};
use bevy::prelude::*;

/// Indicates the depth level of a node in the UI tree.
///
/// * `0` = Root
/// * `1` = Children of Root
/// * etc.
///
/// This is crucial for the layout engine to process nodes in the correct order
/// (Parents before Children or vice-versa).
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayoutDepth(pub usize);

/// Stable local stacking position inside one root capsule.
///
/// The value is normalized to `[0.0, 1.0)` and is derived from the node's
/// preorder paint position after sorting siblings by [`crate::layout::univis_node::USelf::order`]
/// and then preserving the authored `Children` order for ties.
#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct UiLocalStacking {
    pub normalized: f32,
}

/// Stores the "Intrinsic Size" of a UI element.
///
/// This value is calculated during the **Upward Pass** (`pass_up`).
/// It represents how much space the element *wants* based on its content and children,
/// before any external constraints are applied.
#[derive(Component, Default, Debug, Clone, Copy, Reflect)]
pub struct IntrinsicSize {
    /// Preferred intrinsic width used by the current sizing mode.
    pub width: f32,
    /// Preferred intrinsic height used by the current sizing mode.
    pub height: f32,
    /// Minimum intrinsic content width.
    pub min_width: f32,
    /// Maximum intrinsic content width.
    pub max_width: f32,
    /// Minimum intrinsic content height.
    pub min_height: f32,
    /// Maximum intrinsic content height.
    pub max_height: f32,
}

impl IntrinsicSize {
    /// Creates an intrinsic record whose preferred size matches its max-content size.
    pub fn from_max_size(size: Vec2) -> Self {
        Self {
            width: size.x,
            height: size.y,
            min_width: size.x,
            max_width: size.x,
            min_height: size.y,
            max_height: size.y,
        }
    }
}

/// A global resource that tracks the maximum depth of the UI tree.
///
/// This allows the layout systems to know how many iterations are needed
/// to traverse the entire tree.
#[derive(Resource, Default)]
pub struct LayoutTreeDepth {
    pub max_depth: usize,
}

/// Cached internal marker for the 3D render path.
///
/// This is derived from the nearest [`crate::layout::layout_system::ResolvedRootUi`]
/// and should not be treated as the source of truth.
#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct UI3d;

/// Per-node settlement generations for the main layout/render stages.
///
/// Each `*_input_generation` is bumped when a node receives new work for that
/// stage. The matching `*_done_generation` catches up only after the stage has
/// actually processed the latest inputs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiNodeStageVersions {
    pub measure_input_generation: u64,
    pub measure_done_generation: u64,
    pub solve_input_generation: u64,
    pub solve_done_generation: u64,
    pub render_input_generation: u64,
    pub render_done_generation: u64,
}

impl UiNodeStageVersions {
    pub fn mark_measure_dirty(&mut self, generation: u64) {
        self.measure_input_generation = self.measure_input_generation.max(generation);
    }

    pub fn complete_measure(&mut self) {
        self.measure_done_generation = self
            .measure_done_generation
            .max(self.measure_input_generation);
    }

    pub fn mark_solve_dirty(&mut self, generation: u64) {
        self.solve_input_generation = self.solve_input_generation.max(generation);
    }

    pub fn complete_solve(&mut self) {
        self.solve_done_generation = self.solve_done_generation.max(self.solve_input_generation);
    }

    pub fn mark_render_dirty(&mut self, generation: u64) {
        self.render_input_generation = self.render_input_generation.max(generation);
    }

    pub fn complete_render(&mut self) {
        self.render_done_generation = self
            .render_done_generation
            .max(self.render_input_generation);
    }
}

/// Cached root and clip ancestry derived during the hierarchy phase.
///
/// This avoids repeated parent-chain walks in solve, render, and picking hot
/// paths. The nearest enabled clip ancestor is stored separately so render and
/// hit tests can fetch the current clip geometry directly from that entity.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CachedUiContext {
    pub root_entity: Option<Entity>,
    pub camera_entity: Option<Entity>,
    pub space: UiSpace,
    pub ui_to_world_scale: f32,
    pub root_stack: ResolvedRootStack,
    pub clip_ancestor: Option<Entity>,
}

impl Default for CachedUiContext {
    fn default() -> Self {
        Self {
            root_entity: None,
            camera_entity: None,
            space: UiSpace::Screen,
            ui_to_world_scale: 1.0,
            root_stack: ResolvedRootStack::default(),
            clip_ancestor: None,
        }
    }
}
