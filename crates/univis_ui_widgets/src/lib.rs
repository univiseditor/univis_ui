//! Built-in widget crate for Univis UI.
//!
//! This crate layers higher-level controls on top of the engine and interaction
//! crates, including text, buttons, forms, panels, scrolling, and selection widgets.
//! The public surface also exposes grouped visual and interactive widget layers.

pub mod schedule;
pub mod widget;

/// The recommended import surface when depending on `univis_ui_widgets` directly.
pub mod prelude {
    pub use crate::widget::prelude::*;
}
