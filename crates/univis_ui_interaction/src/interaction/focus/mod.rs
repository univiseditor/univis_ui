//! Focus tracking, gamepad/keyboard navigation, and visual focus states for Univis UI.
//!
//! This module provides composable ECS primitives for:
//! - [`UFocusable`](crate::interaction::focus::UFocusable): Marks a node as eligible for focus and navigation.
//! - [`UFocused`](crate::interaction::focus::UFocused): Marker component added to the currently focused entity.
//! - [`UFocusVisual`](crate::interaction::focus::UFocusVisual): Declarative visual feedback (borders, colors, spring scaling) for focused state.
//! - [`UFocusState`](crate::interaction::focus::UFocusState): Global resource tracking the active focused entity and input modality.
//! - [`UFocusNavigationSettings`](crate::interaction::focus::UFocusNavigationSettings): Configurable navigation options (repeat delays, wrap-around).
//! - 2D Spatial Navigation across arbitrary layouts (directional arrows, D-pad, analog stick).
//! - Tab Navigation (Tab / Shift+Tab) with explicit `tab_index` support.
//! - Action Activation (Enter, Space, Gamepad South button) triggering [`UFocusActivate`](crate::interaction::focus::UFocusActivate) and clicks.

pub mod spatial;
pub mod systems;
pub mod types;
pub mod visual;

pub use spatial::{
    find_best_spatial_candidate, find_initial_focus_candidate, find_next_tab_candidate,
};
pub use systems::{
    focus_cleanup_system, focus_gamepad_navigation_system, focus_keyboard_navigation_system,
};
pub use types::{
    FocusGained, FocusLost, FocusModality, NavDirection, UFocusActivate, UFocusNavigationSettings,
    UFocusState, UFocusable, UFocused, set_focus,
};
pub use visual::{
    UFocusVisual, on_focus_activate_default, on_focus_gained_visual, on_focus_lost_visual,
    on_pointer_focus_click,
};
