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
