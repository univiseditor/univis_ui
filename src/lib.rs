//! # Univis UI Facade
//!
//! This crate is the unified entry point that composes all `univis_ui_*`
//! crates while preserving the existing `UnivisUiPlugin` and `prelude` UX.

use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

pub use univis_ui_engine as engine_crate;
pub use univis_ui_interaction as interaction_crate;
pub use univis_ui_style as style_crate;
pub use univis_ui_widgets as widgets_crate;

pub mod engine {
    pub use univis_ui_engine::*;
}

pub mod style {
    pub use univis_ui_style::style::*;
}

pub mod interaction {
    pub use univis_ui_interaction::interaction::*;
}

pub mod render {
    pub use univis_ui_engine::layout::render::*;
}

pub mod layout {
    pub use univis_ui_engine::layout::*;

    pub mod prelude {
        pub use univis_ui_engine::layout::prelude::*;
    }
}

pub mod widget {
    pub use univis_ui_widgets::widget::*;
}

pub mod prelude {
    pub use crate::UnivisUiPlugin;
    pub use univis_ui_engine::prelude::*;
    pub use univis_ui_interaction::prelude::*;
    pub use univis_ui_style::prelude::*;
    pub use univis_ui_widgets::prelude::*;
}

pub struct UnivisUiPlugin;

impl Plugin for UnivisUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnivisUiStylePlugin)
            .add_plugins(UnivisEnginePlugin)
            .add_plugins(UnivisInteractionPlugin)
            .add_plugins(UnivisWidgetPlugin);
    }
}
