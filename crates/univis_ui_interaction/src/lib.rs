//! Interaction crate for Univis UI.
//!
//! This crate provides pointer-driven picking, interaction state tracking, and
//! hover/press feedback helpers that integrate with Univis roots and widgets.

pub mod interaction;

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub use crate::interaction::prelude::*;
    pub use univis_ui_engine::internal::{
        ComputedSize, LayoutDepth, ResolvedRootStack, ResolvedRootUi,
    };
    pub use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
    pub use univis_ui_engine::prelude::*;
    pub use univis_ui_engine::schedule::UnivisPostUpdateSet;
}

/// The recommended import surface when depending on `univis_ui_interaction` directly.
pub mod prelude {
    pub use crate::interaction::prelude::*;
}
