//! Picking and interaction state for Univis UI.
//!
//! The interaction plugin resolves hits against Univis roots and updates
//! high-level state such as [`feedback::UInteraction`].

use bevy::prelude::*;

use crate::internal_prelude::*;

/// Interaction state components and default pointer observers.
pub mod feedback;
#[doc(hidden)]
pub mod math;
/// Picking backend and hit-resolution helpers for Univis roots.
pub mod picking;

/// Common imports for interaction-related integrations.
pub mod prelude {
    pub use crate::interaction::{UnivisInteractionPlugin, feedback::*, picking::*};
}

/// Registers Univis pointer picking and the default interaction observers.
///
/// This is the main plugin to add when using `univis_ui_interaction`
/// directly instead of the full facade crate.
pub struct UnivisInteractionPlugin;

impl Plugin for UnivisInteractionPlugin {
    fn build(&self, app: &mut App) {
        // 1. Install the picking backend that computes pointer hits.
        app.init_resource::<UiRolloutConfig>()
            .init_resource::<UiValidationState>()
            .init_resource::<picking::PickingSyncState>()
            .add_systems(
                PreUpdate,
                (picking::track_pointer_generation, univis_picking_backend).chain(),
            )
            .add_systems(
                UiSettlementSchedule,
                picking::post_settle_picking_backend.in_set(UnivisPostUpdateSet::PickSync),
            );

        // 2. Register the default pointer observers.
        app.add_observer(feedback::on_pointer_over);
        app.add_observer(feedback::on_pointer_out);
        app.add_observer(feedback::on_pointer_press);
        app.add_observer(feedback::on_pointer_release);
        app.add_observer(feedback::on_pointer_click);
    }
}
