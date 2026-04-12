//! Interaction crate for Univis UI.
//!
//! This crate provides pointer-driven picking, interaction state tracking, and
//! hover/press feedback helpers that integrate with Univis roots and widgets.

pub mod interaction;

/// The recommended import surface when depending on `univis_ui_interaction` directly.
pub mod prelude {
    pub use crate::interaction::prelude::*;
}
