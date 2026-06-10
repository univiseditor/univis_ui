//! Read-only query-facing layout types for cross-crate integrations.
//!
//! This module is the stable import surface for consumers that need resolved
//! layout output, root state, or cached spatial metadata without depending on
//! hidden engine internals such as `layout::core`.

use bevy::prelude::*;

use crate::layout::layout_system::UiSpace;

pub use crate::layout::components::{CachedUiContext, IntrinsicSize, LayoutDepth, UI3d};
pub use crate::layout::geometry::ComputedSize;
pub use crate::layout::layout_system::{ResolvedRootStack, ResolvedRootUi};

/// Read-only picking metadata derived by the engine for one resolved node.
///
/// This snapshot is intended for interaction and hit-testing crates that need
/// stable spatial ordering without reconstructing hierarchy/cache internals.
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UiPickingContext {
    /// Entity ID of the resolved root capsule handling this node.
    pub root_entity: Option<Entity>,
    /// Entity ID of the camera this node renders to.
    pub camera_entity: Option<Entity>,
    /// The UI space (Screen, World2d, World3d) this node belongs to.
    pub space: UiSpace,
    /// Entity ID of the ancestor clip bounds for this node, if any.
    pub clip_ancestor: Option<Entity>,
    /// Global stacking key of the root capsule for root-level sorting.
    pub root_sort_key: f32,
    /// Local z-offset relative to the root for hierarchical sorting.
    pub local_depth_key: f32,
    /// Scaling factor converting logical UI units to world units.
    pub world_scale: f32,
}

impl Default for UiPickingContext {
    fn default() -> Self {
        Self {
            root_entity: None,
            camera_entity: None,
            space: UiSpace::Screen,
            clip_ancestor: None,
            root_sort_key: 0.0,
            local_depth_key: 0.0,
            world_scale: 1.0,
        }
    }
}
