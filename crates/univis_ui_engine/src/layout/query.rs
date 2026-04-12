//! Read-only query-facing layout types for cross-crate integrations.
//!
//! This module is the stable import surface for consumers that need resolved
//! layout output, root state, or cached spatial metadata without depending on
//! hidden engine internals such as `layout::core`.

use bevy::prelude::*;

use crate::layout::layout_system::UiSpace;

pub use crate::layout::components::{CachedUiContext, IntrinsicSize, LayoutDepth};
pub use crate::layout::geometry::ComputedSize;
pub use crate::layout::layout_system::{ResolvedRootStack, ResolvedRootUi};

/// Read-only picking metadata derived by the engine for one resolved node.
///
/// This snapshot is intended for interaction and hit-testing crates that need
/// stable spatial ordering without reconstructing hierarchy/cache internals.
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct UiPickingContext {
    pub root_entity: Option<Entity>,
    pub camera_entity: Option<Entity>,
    pub space: UiSpace,
    pub clip_ancestor: Option<Entity>,
    pub root_sort_key: f32,
    pub local_depth_key: f32,
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
        }
    }
}
