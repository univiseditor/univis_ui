//! Built-in widgets for Univis UI.
//!
//! The widgets in this module are composable Bevy components layered on top of
//! the engine crate. Add [`UnivisWidgetPlugin`] for the common widget set, or
//! compose dedicated widget plugins manually when you want a narrower surface.

use bevy::asset::embedded_asset;
use bevy::prelude::*;
use univis_ui_engine::layout::image::UImage;
use univis_ui_engine::schedule::{
    UiSettlementSchedule, UnivisPostUpdateSet, UnivisWidgetUpdateSet,
};

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

/// Common widget imports for applications that depend on `univis_ui_widgets` directly.
pub mod prelude {
    pub use crate::widget::{
        badge::*, button::*, checkbox::*, divider::*, drag_value::*, icon_btn::*, image::*,
        panel::*, progress::*, radio::*, scroll_view::*, seekbar::*, select::*, text_field::*,
        text_label::*, toggle::*,
    };
}

/// Registers the default built-in widget suite.
///
/// This plugin covers text rendering, panels, buttons, scrolling, toggles,
/// text input, badges, selects, and other commonly used controls.
pub struct UnivisWidgetPlugin;

#[derive(Default)]
struct WidgetRuntimeWarnings {
    tag_runtime_limited: bool,
}

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<UImage>()
            .configure_sets(
                Update,
                (
                    UnivisWidgetUpdateSet::Build,
                    UnivisWidgetUpdateSet::Logic,
                    UnivisWidgetUpdateSet::Visual,
                    UnivisWidgetUpdateSet::Events,
                )
                    .chain(),
            )
            .add_systems(Update, warn_on_widget_runtime_limitations)
            .add_systems(
                UiSettlementSchedule,
                sync_image_geometry
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(UnivisPostUpdateSet::LayoutMeasure),
            );

        add_core_widget_plugins(app);
        add_default_widget_runtime_plugins(app);
    }
}

fn add_core_widget_plugins(app: &mut App) {
    app.add_plugins(UnivisTextPlugin)
        .add_plugins(UnivisProgressPlugin)
        .add_plugins(UnivisButtonPlugin)
        .add_plugins(UnivisRadioPlugin)
        .add_plugins(UnivisIconButtonPlugin)
        .add_plugins(UnivisTogglePlugin)
        .add_plugins(UnivisCheckboxPlugin)
        .add_plugins(UnivisSeekBarPlugin)
        .add_plugins(UnivisScrollViewPlugin)
        .add_plugins(UnivisDividerPlugin)
        .add_plugins(UnivisPanelPlugin);
}

pub(super) fn register_widget_embedded_assets(app: &mut App) {
    embedded_asset!(app, "shaders/text_label_sdf.wgsl");
}

fn add_default_widget_runtime_plugins(app: &mut App) {
    app.add_plugins(UnivisBadgePlugin)
        .add_plugins(UnivisDragValuePlugin)
        .add_plugins(UnivisSelectPlugin)
        .add_plugins(UnivisTextFieldPlugin);
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

    #[test]
    fn default_widget_runtime_plugins_install_text_field_and_badge_support() {
        let mut app = App::new();
        add_default_widget_runtime_plugins(&mut app);

        assert!(
            app.world()
                .get_resource::<TextFieldPluginInstalled>()
                .is_some()
        );
        assert!(app.world().get_resource::<BadgePluginInstalled>().is_some());
    }

    #[test]
    fn dedicated_widget_plugins_remain_safe_to_add_after_default_runtime_plugins() {
        let mut app = App::new();
        add_default_widget_runtime_plugins(&mut app);
        app.add_plugins((UnivisTextFieldPlugin, UnivisBadgePlugin));

        assert!(
            app.world()
                .get_resource::<TextFieldPluginInstalled>()
                .is_some()
        );
        assert!(app.world().get_resource::<BadgePluginInstalled>().is_some());
    }
}
