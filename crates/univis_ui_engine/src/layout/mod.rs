//! Layout and root systems for Univis UI.
//!
//! This module contains:
//!
//! - [`crate::layout::layout_system`] for [`crate::layout::layout_system::URootUi`]
//!   and root resolution.
//! - [`crate::layout::univis_node`] for node, layout, and local positioning components.
//! - [`crate::layout::geometry`] for logical UI units and spacing helpers.
//! - [`crate::layout::render`] for mesh/material synchronization.
//!
//! Most users reach this module through [`crate::prelude`] or
//! [`crate::layout::prelude`].

#[doc(hidden)]
pub mod algorithms;
#[doc(hidden)]
pub mod components;
#[doc(hidden)]
pub mod core;
/// Logical UI units, spacing helpers, and box-side utilities.
pub mod geometry;
/// Image-backed node helpers.
pub mod image;
/// Public requests for external layout invalidation.
pub mod invalidation;
/// The public root model and root-resolution systems.
pub mod layout_system;
/// Physically based material overrides for `World3d` content.
pub mod pbr;
#[doc(hidden)]
pub mod pipeline;
mod plugin;
/// Optional profiling and layout diagnostics helpers.
pub mod profiling;
/// Read-only query-facing types for resolved layout and root state.
pub mod query;
mod registration;
/// Mesh/material synchronization and render-facing types.
pub mod render;
mod settlement_loop;
#[doc(hidden)]
pub mod solver_types;
#[cfg(test)]
mod tests;
/// Core node, layout, and local positioning components.
pub mod univis_node;

pub use self::plugin::UnivisLayoutPlugin;

/// Common imports for authoring layout trees and roots directly from the engine crate.
pub mod prelude {
    pub use crate::layout::UnivisLayoutPlugin;
    pub use crate::layout::geometry::{UCornerRadius, USides, UVal};
    pub use crate::layout::image::UImage;
    pub use crate::layout::invalidation::{UiInvalidateRequestQueue, UiLayoutInvalidation};
    pub use crate::layout::layout_system::{
        URootUi, UiCameraRef, UiCanvasSize, UiRootSettlementState, UiSpace,
    };
    pub use crate::layout::pbr::UPbr;
    pub use crate::layout::univis_node::*;
}
