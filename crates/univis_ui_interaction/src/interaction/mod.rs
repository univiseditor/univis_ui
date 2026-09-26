//! Picking and interaction state for Univis UI.
//!
//! The interaction plugin resolves hits against Univis roots and updates
//! high-level state such as [`feedback::UInteraction`].

use bevy::prelude::*;
use univis_ui_engine::schedule::{
    UiPickingRuntimeState, UiSettlementSchedule, UnivisPostUpdateSet, sync_picking_runtime_state,
};

/// Interaction state components and default pointer observers.
pub mod feedback;
/// Focus tracking, gamepad/keyboard spatial navigation, and visual focus states.
pub mod focus;
#[doc(hidden)]
pub mod math;
/// Picking backend and hit-resolution helpers for Univis roots.
pub mod picking;

use crate::interaction::picking::univis_picking_backend;

/// Common imports for interaction-related integrations.
pub mod prelude {
    pub use crate::interaction::{UnivisInteractionPlugin, feedback::*, focus::*, picking::*};
}

/// Registers Univis pointer picking, focus navigation, and default interaction observers.
///
/// This is the main plugin to add when using `univis_ui_interaction`
/// directly instead of the full facade crate.
pub struct UnivisInteractionPlugin;

impl Plugin for UnivisInteractionPlugin {
    fn build(&self, app: &mut App) {
        // 1. Install the picking backend that computes pointer hits.
        app.init_resource::<picking::PickingSyncState>()
            .init_resource::<picking::PickingValidationState>()
            .init_resource::<UiPickingRuntimeState>()
            .init_resource::<focus::UFocusNavigationSettings>()
            .init_resource::<focus::UFocusState>()
            .register_type::<focus::UFocusable>()
            .register_type::<focus::UFocused>()
            .register_type::<focus::UFocusVisual>()
            .register_type::<focus::UFocusNavigationSettings>()
            .register_type::<focus::UFocusState>()
            .register_type::<focus::FocusModality>()
            .register_type::<focus::NavDirection>()
            .add_systems(
                PreUpdate,
                (picking::track_pointer_generation, univis_picking_backend).chain(),
            )
            .add_systems(
                Update,
                (
                    focus::focus_cleanup_system,
                    focus::focus_keyboard_navigation_system,
                    focus::focus_gamepad_navigation_system,
                ),
            )
            .add_systems(
                UiSettlementSchedule,
                sync_picking_runtime_state
                    .in_set(UnivisPostUpdateSet::ExternalPostSolve)
                    .before(picking::post_settle_picking_backend),
            )
            .add_systems(
                UiSettlementSchedule,
                picking::post_settle_picking_backend.in_set(UnivisPostUpdateSet::ExternalPostSolve),
            );

        // 2. Register the default pointer and focus observers.
        app.add_observer(feedback::on_pointer_over);
        app.add_observer(feedback::on_pointer_out);
        app.add_observer(feedback::on_pointer_press);
        app.add_observer(feedback::on_pointer_release);
        app.add_observer(feedback::on_pointer_click);
        app.add_observer(focus::on_focus_gained_visual);
        app.add_observer(focus::on_focus_lost_visual);
        app.add_observer(focus::on_pointer_focus_click);
        app.add_observer(focus::on_focus_activate_default);
    }
}
