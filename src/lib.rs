//! Univis UI is the facade crate for the full `univis_ui_*` workspace.
//!
//! It gives applications a single import surface with:
//!
//! - [`UnivisUiPlugin`] to register style, engine, interaction, and widget plugins.
//! - [`prelude`] for the most common UI types and widgets.
//! - namespaced modules such as [`layout`], [`widget`], and [`interaction`] for
//!   deeper integration work.
//!
//! For high-level guides, examples, and migration notes, see the repository `docs/`
//! book and the example programs shipped with the workspace.

use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

pub use univis_ui_engine as engine_crate;
pub use univis_ui_interaction as interaction_crate;
pub use univis_ui_style as style_crate;
pub use univis_ui_widgets as widgets_crate;

/// Curated engine access for applications that need more than the recommended facade prelude.
///
/// Use [`crate::engine_crate`] when you intentionally need the full engine crate
/// surface, including lower-level compatibility and hidden modules.
pub mod engine {
    pub use univis_ui_engine::{UnivisEnginePlugin, layout, prelude, schedule};
}

/// Advanced style access, including fonts, icons, and the shared theme resource.
pub mod style {
    pub use univis_ui_style::{prelude, style};
}

/// Advanced interaction access, including picking and interaction state types.
pub mod interaction {
    pub use univis_ui_interaction::{interaction, prelude};
}

/// Advanced render-facing access for material and mesh integration work.
pub mod render {
    pub use univis_ui_engine::layout::render::{
        UnivisRenderPlugin, material, material_3d, prelude,
    };
}

/// Advanced layout access for roots, node primitives, and layout-specific systems.
pub mod layout {
    pub use univis_ui_engine::layout::{
        UnivisLayoutPlugin, geometry, image, invalidation, layout_system, pbr, profiling, query,
        render, univis_node,
    };

    /// Common layout-facing imports such as roots, nodes, and geometry helpers.
    pub mod prelude {
        pub use univis_ui_engine::layout::prelude::*;
    }
}

/// Re-exports the built-in widget modules.
pub mod widget {
    pub use univis_ui_widgets::{schedule, widget::*};
}

/// The recommended import surface for most applications.
///
/// This bundles the facade plugin plus the core public types from the engine,
/// style, interaction, and widgets crates.
///
/// Deprecated compatibility wrappers remain available only through explicit
/// paths such as [`crate::layout::layout_system::UScreenRoot`] and
/// [`crate::layout::layout_system::UWorldRoot`].
pub mod prelude {
    pub use crate::UnivisUiPlugin;
    pub use univis_ui_engine::prelude::*;
    pub use univis_ui_interaction::prelude::*;
    pub use univis_ui_style::prelude::*;
    pub use univis_ui_widgets::prelude::*;
}

/// Registers the full Univis UI stack in the recommended order:
/// style, engine, interaction, then widgets.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui::UnivisUiPlugin;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(UnivisUiPlugin);
/// ```
pub struct UnivisUiPlugin;

impl Plugin for UnivisUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UnivisUiStylePlugin)
            .add_plugins(UnivisEnginePlugin)
            .add_plugins(UnivisInteractionPlugin)
            .add_plugins(UnivisWidgetPlugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use bevy::{asset::AssetPlugin, text::TextPlugin};
    use univis_ui_engine::schedule::{UiRolloutConfig, UiValidationState};

    fn public_plugin_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), TextPlugin));
        app
    }

    #[test]
    fn facade_prelude_exposes_canonical_surface() {
        let _root = URootUi::screen();
        let _node = UNode::default();
        let _padding = USides::all(12.0);
        let _button = UButton::primary();
        let _panel = UPanel::card();
        let _select = USelect::new();
        let _label = UTextLabel::default();
        let _interaction = UInteraction::default();
        let _colors = UInteractionColors::default();
    }

    #[test]
    fn facade_plugin_registers_engine_resources() {
        let mut app = public_plugin_test_app();
        app.add_plugins(UnivisUiPlugin);

        assert!(app.world().contains_resource::<UiRolloutConfig>());
        assert!(app.world().contains_resource::<UiValidationState>());
    }

    #[test]
    fn direct_plugin_combo_registers_engine_resources() {
        let mut app = public_plugin_test_app();
        app.add_plugins((
            UnivisUiStylePlugin,
            UnivisEnginePlugin,
            UnivisInteractionPlugin,
            UnivisWidgetPlugin,
        ));

        assert!(app.world().contains_resource::<UiRolloutConfig>());
        assert!(app.world().contains_resource::<UiValidationState>());
    }
}
