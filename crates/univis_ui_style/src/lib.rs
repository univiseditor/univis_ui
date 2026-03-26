//! Style crate for Univis UI.
//!
//! This crate owns embedded fonts, icons, and the shared [`style::Theme`]
//! resource that widgets can read during setup.

pub mod style;

/// The recommended import surface when depending on `univis_ui_style` directly.
pub mod prelude {
    pub use crate::style::prelude::*;
}
