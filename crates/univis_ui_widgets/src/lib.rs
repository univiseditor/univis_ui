//! Built-in widget crate for Univis UI.
//!
//! This crate layers higher-level controls on top of the engine and interaction
//! crates, including text, buttons, forms, panels, scrolling, and selection widgets.

pub mod widget;

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub use crate::widget::prelude::*;
    pub use univis_ui_engine::internal::{ComputedSize, ResolvedRootStack, ResolvedRootUi};
    pub use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
    pub use univis_ui_engine::prelude::*;
    pub use univis_ui_engine::schedule::{
        UiSettlementSchedule, UnivisPostUpdateSet, UnivisWidgetUpdateSet,
    };
    pub use univis_ui_interaction::prelude::*;
    pub use univis_ui_style::prelude::*;
}

/// The recommended import surface when depending on `univis_ui_widgets` directly.
pub mod prelude {
    pub use crate::widget::prelude::*;
}
