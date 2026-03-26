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

/// Re-exports the full engine crate, including layout and rendering internals.
pub mod engine {
    pub use univis_ui_engine::*;
}

/// Re-exports the style crate, including fonts, icons, and the shared theme resource.
pub mod style {
    pub use univis_ui_style::style::*;
}

/// Re-exports the interaction crate, including picking and interaction state types.
pub mod interaction {
    pub use univis_ui_interaction::interaction::*;
}

/// Re-exports render-specific pieces from the engine crate.
pub mod render {
    pub use univis_ui_engine::layout::render::*;
}

/// Re-exports the layout system, root model, and node primitives.
pub mod layout {
    pub use univis_ui_engine::layout::*;

    /// Common layout-facing imports such as roots, nodes, and geometry helpers.
    pub mod prelude {
        pub use univis_ui_engine::layout::prelude::*;
    }
}

/// Re-exports the built-in widget modules.
pub mod widget {
    pub use univis_ui_widgets::widget::*;
}

/// The recommended import surface for most applications.
///
/// This bundles the facade plugin plus the core public types from the engine,
/// style, interaction, and widgets crates.
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
