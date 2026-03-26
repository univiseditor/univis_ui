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
    pub width: f32,
    pub height: f32,
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
