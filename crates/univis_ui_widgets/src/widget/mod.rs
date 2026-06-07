//! Built-in widgets for Univis UI.
//!
//! The widgets in this module are composable Bevy components layered on top of
//! the engine crate. Add [`UnivisWidgetPlugin`] for the full widget suite,
//! [`UnivisVisualWidgetPlugin`] for text/image/progress/divider/badge support,
//! or [`UnivisInteractiveWidgetPlugin`] for buttons, panels, forms, and other
//! interactive controls.

use bevy::asset::embedded_asset;
use bevy::prelude::*;
use univis_ui_engine::layout::image::UImage;
use univis_ui_engine::schedule::{UiSettlementSchedule, UnivisPostUpdateSet};

use crate::schedule::UnivisWidgetUpdateSet;
use crate::widget::{
    badge::{UTag, UnivisBadgePlugin},
    button::UnivisButtonPlugin,
    checkbox::UnivisCheckboxPlugin,
    divider::UnivisDividerPlugin,
    drag_value::UnivisDragValuePlugin,
    icon_btn::UnivisIconButtonPlugin,
    image::sync_image_geometry,
    panel::UnivisPanelPlugin,
    progress::UnivisProgressPlugin,
    radio::UnivisRadioPlugin,
    scroll_view::UnivisScrollViewPlugin,
    seekbar::UnivisSeekBarPlugin,
    select::UnivisSelectPlugin,
    text_field::UnivisTextFieldPlugin,
    text_label::UnivisTextPlugin,
    toggle::UnivisTogglePlugin,
};

/// Badge and tag widgets.
pub mod badge;
/// Button widgets.
pub mod button;
/// Checkbox widget.
pub mod checkbox;
/// Divider widget.
pub mod divider;
/// Drag-to-edit numeric widget.
pub mod drag_value;
/// Icon button widget.
pub mod icon_btn;
/// Image widget.
pub mod image;
/// Panel and panel-window widgets.
pub mod panel;
/// Progress-bar widget.
pub mod progress;
/// Radio button and radio-group widgets.
pub mod radio;
/// Scroll container widget.
pub mod scroll_view;
/// Seek-bar widget.
pub mod seekbar;
/// Select / dropdown widget.
pub mod select;
/// Editable text-field widget.
pub mod text_field;
/// Text rendering widget.
pub mod text_label;
/// Toggle / switch widget.
pub mod toggle;

/// Visual/layout-oriented widgets that stay independent from the interaction crate.
pub mod visual {
    pub use crate::widget::{
        UnivisVisualWidgetPlugin, badge::*, divider::*, image::*, progress::*, text_label::*,
    };
}

/// Interactive controls layered on top of the interaction crate.
pub mod interactive {
    pub use crate::widget::{
        UnivisInteractiveWidgetPlugin, button::*, checkbox::*, drag_value::*, icon_btn::*,
        panel::*, radio::*, scroll_view::*, seekbar::*, select::*, text_field::*, toggle::*,
    };
}

/// Common widget imports for applications that depend on `univis_ui_widgets` directly.
pub mod prelude {
    pub use crate::widget::{
        badge::*, button::*, checkbox::*, divider::*, drag_value::*, icon_btn::*, image::*,
        panel::*, progress::*, radio::*, scroll_view::*, seekbar::*, select::*, text_field::*,
        text_label::*, toggle::*,
    };
}

/// Registers the visual/layout-oriented widget layer.
pub struct UnivisVisualWidgetPlugin;

/// Registers the interactive widget layer.
pub struct UnivisInteractiveWidgetPlugin;

/// Registers the full built-in widget suite.
///
/// This plugin covers text rendering, panels, buttons, scrolling, toggles,
/// text input, badges, selects, and other commonly used controls.
pub struct UnivisWidgetPlugin;

#[derive(Resource, Default)]
struct WidgetRuntimeConfigured;

#[derive(Resource, Default)]
struct VisualWidgetPluginInstalled;

#[derive(Resource, Default)]
struct InteractiveWidgetPluginInstalled;

#[derive(Resource, Default)]
struct WidgetPluginInstalled;

#[derive(Default)]
struct WidgetRuntimeWarnings {
    tag_runtime_limited: bool,
}

impl Plugin for UnivisVisualWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        ensure_widget_runtime_configured(app);

        if app
            .world()
            .get_resource::<VisualWidgetPluginInstalled>()
            .is_some()
        {
            return;
        }

        app.init_resource::<VisualWidgetPluginInstalled>()
            .register_type::<UImage>()
            .add_systems(Update, warn_on_widget_runtime_limitations)
            .add_systems(
                UiSettlementSchedule,
                sync_image_geometry
                    .in_set(UnivisPostUpdateSet::ExternalPrepare)
                    .before(UnivisPostUpdateSet::LayoutMeasure),
            );

        add_visual_widget_plugins(app);
    }

    fn is_unique(&self) -> bool {
        false
    }
}

impl Plugin for UnivisInteractiveWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        ensure_widget_runtime_configured(app);

        if app
            .world()
            .get_resource::<InteractiveWidgetPluginInstalled>()
            .is_some()
        {
            return;
        }

        app.init_resource::<InteractiveWidgetPluginInstalled>();
        add_interactive_widget_plugins(app);
    }

    fn is_unique(&self) -> bool {
        false
    }
}

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        if app
            .world()
            .get_resource::<WidgetPluginInstalled>()
            .is_some()
        {
            return;
        }

        app.init_resource::<WidgetPluginInstalled>()
            .add_plugins((UnivisVisualWidgetPlugin, UnivisInteractiveWidgetPlugin));
    }

    fn is_unique(&self) -> bool {
        false
    }
}

fn ensure_widget_runtime_configured(app: &mut App) {
    if app
        .world()
        .get_resource::<WidgetRuntimeConfigured>()
        .is_some()
    {
        return;
    }

    app.init_resource::<WidgetRuntimeConfigured>()
        .configure_sets(
            Update,
            (
                UnivisWidgetUpdateSet::Build,
                UnivisWidgetUpdateSet::Logic,
                UnivisWidgetUpdateSet::Visual,
                UnivisWidgetUpdateSet::Events,
            )
                .chain(),
        );
}

fn add_visual_widget_plugins(app: &mut App) {
    app.add_plugins(UnivisTextPlugin)
        .add_plugins(UnivisProgressPlugin)
        .add_plugins(UnivisDividerPlugin)
        .add_plugins(UnivisBadgePlugin);
}

fn add_interactive_widget_plugins(app: &mut App) {
    app.add_plugins(UnivisButtonPlugin)
        .add_plugins(UnivisRadioPlugin)
        .add_plugins(UnivisIconButtonPlugin)
        .add_plugins(UnivisTogglePlugin)
        .add_plugins(UnivisCheckboxPlugin)
        .add_plugins(UnivisSeekBarPlugin)
        .add_plugins(UnivisScrollViewPlugin)
        .add_plugins(UnivisPanelPlugin)
        .add_plugins(UnivisDragValuePlugin)
        .add_plugins(UnivisSelectPlugin)
        .add_plugins(UnivisTextFieldPlugin);
}

pub(super) fn register_widget_embedded_assets(app: &mut App) {
    embedded_asset!(app, "shaders/text_label_sdf.wgsl");
    embedded_asset!(app, "shaders/text_label_sdf_3d.wgsl");
}

fn warn_on_widget_runtime_limitations(
    added_tags: Query<(), Added<UTag>>,
    mut warnings: Local<WidgetRuntimeWarnings>,
) {
    if !warnings.tag_runtime_limited && !added_tags.is_empty() {
        bevy::log::warn!(
            "UTag detected. UTag runtime systems are currently limited; validate behavior in your scene."
        );
        warnings.tag_runtime_limited = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{badge::BadgePluginInstalled, text_field::TextFieldPluginInstalled};
    use bevy::{asset::AssetPlugin, text::TextPlugin};

    fn widget_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), TextPlugin));
        app
    }

    #[test]
    fn visual_widget_plugin_installs_visual_support_without_interactive_runtime_plugins() {
        let mut app = widget_test_app();
        app.add_plugins(UnivisVisualWidgetPlugin);

        assert!(app.world().get_resource::<BadgePluginInstalled>().is_some());
        assert!(
            app.world()
                .get_resource::<TextFieldPluginInstalled>()
                .is_none()
        );
    }

    #[test]
    fn interactive_widget_plugin_installs_interactive_runtime_plugins_without_badge_support() {
        let mut app = widget_test_app();
        app.add_plugins(UnivisInteractiveWidgetPlugin);

        assert!(
            app.world()
                .get_resource::<TextFieldPluginInstalled>()
                .is_some()
        );
        assert!(app.world().get_resource::<BadgePluginInstalled>().is_none());
    }

    #[test]
    fn layered_widget_plugins_install_visual_and_interactive_support_together() {
        let mut app = widget_test_app();
        app.add_plugins((UnivisVisualWidgetPlugin, UnivisInteractiveWidgetPlugin));

        assert!(
            app.world()
                .get_resource::<TextFieldPluginInstalled>()
                .is_some()
        );
        assert!(app.world().get_resource::<BadgePluginInstalled>().is_some());
    }

    #[test]
    fn full_widget_plugin_remains_safe_to_add_after_layered_plugins() {
        let mut app = widget_test_app();
        app.add_plugins((UnivisVisualWidgetPlugin, UnivisInteractiveWidgetPlugin));
        app.add_plugins(UnivisWidgetPlugin);

        assert!(
            app.world()
                .get_resource::<TextFieldPluginInstalled>()
                .is_some()
        );
        assert!(app.world().get_resource::<BadgePluginInstalled>().is_some());
    }
}
