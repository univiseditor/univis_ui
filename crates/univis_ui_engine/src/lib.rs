//! Core engine crate for Univis UI.
//!
//! This crate owns the root model, layout solver, rendering sync, and the
//! schedule sets that the rest of the workspace builds on top of.
//!
//! Most applications import these APIs through `univis_ui::prelude`, while
//! engine-level integrations typically use [`prelude`] or [`layout`] directly.

use bevy::prelude::*;

/// Layout, roots, rendering helpers, and node primitives.
pub mod layout;
/// Shared schedule sets used by engine, widgets, and interaction systems.
pub mod schedule;

#[allow(unused_imports)]
#[doc(hidden)]
pub(crate) mod internal {
    pub use crate::layout::components::{
        CachedUiContext, IntrinsicSize, LayoutDepth, LayoutTreeDepth, UI3d, UiNodeStageVersions,
    };
    pub use crate::layout::geometry::ComputedSize;
    pub use crate::layout::layout_system::{ResolvedRootStack, ResolvedRootUi};
    pub use crate::layout::render::system::{MaterialHandles, MaterialPool, MeshPool};
}

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub use crate::internal::*;
    pub use crate::layout::algorithms::prelude::*;
    pub use crate::layout::components::*;
    pub use crate::layout::core::prelude::*;
    pub use crate::layout::geometry::*;
    pub use crate::layout::image::*;
    pub use crate::layout::layout_system::*;
    pub use crate::layout::pbr::*;
    pub use crate::layout::pipeline::prelude::*;
    pub use crate::layout::profiling::*;
    pub use crate::layout::render::prelude::*;
    pub use crate::layout::solver_types::*;
    pub use crate::layout::univis_node::*;
    pub use crate::schedule::*;
    pub use univis_ui_style::prelude::*;
}

/// The recommended import surface when depending on `univis_ui_engine` directly.
///
/// Deprecated compatibility wrappers stay on explicit paths such as
/// [`crate::layout::layout_system::UScreenRoot`] and
/// [`crate::layout::layout_system::UWorldRoot`] instead of this default import.
pub mod prelude {
    pub use crate::layout::geometry::{UCornerRadius, USides, UVal};
    pub use crate::layout::image::UImage;
    pub use crate::layout::layout_system::{
        URootUi, UiCameraRef, UiCanvasSize, UiRootSettlementState, UiSpace,
    };
    pub use crate::layout::pbr::UPbr;
    pub use crate::layout::univis_node::*;
    pub use crate::schedule::{
        UiPickingRuntimeState, UiRolloutConfig, UiSettlementConfig, UiSettlementRuntimeState,
        UiValidationMode, UiValidationState,
    };
    pub use crate::{UnivisEnginePlugin, layout::prelude::*};
}

/// Registers the engine-level plugins responsible for node setup, layout
/// solving, and render synchronization.
pub struct UnivisEnginePlugin;

impl Plugin for UnivisEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<schedule::UiRolloutConfig>()
            .init_resource::<schedule::UiValidationState>()
            .add_plugins((
                layout::univis_node::UnivisNodePlugin,
                layout::UnivisLayoutPlugin,
                layout::render::UnivisRenderPlugin,
            ));
    }
}
