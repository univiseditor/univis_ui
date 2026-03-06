//! # Univis UI Facade
//!
//! This crate is the unified entry point that composes all `univis_ui_*`
//! crates while preserving the existing `UnivisUiPlugin` and `prelude` UX.

use bevy::prelude::*;
use univis_ui_core::layout::univis_node::UnivisNodePlugin;
use univis_ui_core::style::UnivisUiStylePlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_layout::layout::UnivisLayoutPlugin;
use univis_ui_render::layout::render::UnivisRenderPlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

pub use univis_ui_core as core_crate;
pub use univis_ui_interaction as interaction_crate;
pub use univis_ui_layout as layout_crate;
pub use univis_ui_render as render_crate;
pub use univis_ui_widgets as widgets_crate;

pub mod style {
    pub use univis_ui_core::style::*;
}

pub mod interaction {
    pub use univis_ui_interaction::interaction::*;
}

pub mod widget {
    pub use univis_ui_widgets::widget::*;
}

pub mod render {
    pub use univis_ui_render::layout::render::*;
}

pub mod layout {
    pub use univis_ui_core::layout::components;
    pub use univis_ui_core::layout::geometry;
    pub use univis_ui_core::layout::layout_system;
    pub use univis_ui_core::layout::pbr;
    pub use univis_ui_core::layout::univis_node;
    pub use univis_ui_layout::layout::algorithms;
    pub use univis_ui_layout::layout::core;
    pub use univis_ui_layout::layout::pipeline;
    pub use univis_ui_layout::layout::profiling;
    pub use univis_ui_layout::layout::UnivisLayoutPlugin;
    pub use univis_ui_render::layout::render;

    pub mod prelude {
        pub use univis_ui_core::prelude::*;
        pub use univis_ui_layout::prelude::*;
        pub use univis_ui_render::prelude::*;
    }
}

pub mod prelude {
    pub use crate::UnivisUiPlugin;
    pub use univis_ui_core::prelude::*;
    pub use univis_ui_interaction::prelude::*;
    pub use univis_ui_layout::prelude::*;
    pub use univis_ui_render::prelude::*;
    pub use univis_ui_widgets::prelude::*;
}

pub struct UnivisUiPlugin;

impl Plugin for UnivisUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UnivisInteractionPlugin)
            .add_plugins(UnivisNodePlugin)
            .add_plugins(UnivisLayoutPlugin)
            .add_plugins(UnivisRenderPlugin)
            .add_plugins(UnivisUiStylePlugin)
            .add_plugins(UnivisWidgetPlugin);
    }
}
