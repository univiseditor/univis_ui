//! Built-in widgets for Univis UI.
//!
//! The widgets in this module are composable Bevy components layered on top of
//! the engine crate. Add [`UnivisWidgetPlugin`] for the common widget set, and
//! opt into dedicated plugins such as `UnivisTextFieldPlugin` when a widget
//! advertises extra runtime systems or events.

use crate::internal_prelude::*;
use crate::widget::{badge::BadgePluginInstalled, text_field::TextFieldPluginInstalled};
use bevy::prelude::*;

pub mod badge;
pub mod button;
pub mod checkbox;
pub mod divider;
pub mod drag_value;
pub mod icon_btn;
pub mod image;
mod menu; // Internal placeholder module; not part of public API yet.
pub mod panel;
pub mod progress;
pub mod radio;
pub mod scroll_view;
pub mod seekbar;
pub mod select;
pub mod text_field;
pub mod text_label;
pub mod toggle;

/// Common widget imports for applications that depend on `univis_ui_widgets` directly.
pub mod prelude {
    pub use crate::widget::{
        badge::*,
        button::*,
        checkbox::*,
        divider::*,
        drag_value::*,
        icon_btn::*,
        image::*,
        panel::*,
        progress::*,
        radio::*,
        // menu::*,
        scroll_view::*,
        seekbar::*,
        select::*,
        text_field::*,
        text_label::*,
        toggle::*,
    };
}

/// Registers the default built-in widget suite.
///
/// This plugin covers text rendering, panels, buttons, scrolling, toggles,
/// selects, and other commonly used controls.
pub struct UnivisWidgetPlugin;

#[derive(Default)]
struct MissingOptionalWidgetPluginWarnings {
    text_field_missing_plugin: bool,
    badge_missing_plugin: bool,
    tag_runtime_limited: bool,
}

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<UImage>()
            .add_systems(Update, warn_on_missing_optional_widget_plugins)
            .add_systems(
                PostUpdate,
                sync_image_geometry
                    .in_set(UnivisPostUpdateSet::WidgetSync)
                    .before(UnivisPostUpdateSet::LayoutMeasure),
            )
            .add_plugins(UnivisTextPlugin)
            .add_plugins(UnivisProgressPlugin)
            .add_plugins(UnivisButtonPlugin)
            .add_plugins(UnivisRadioPlugin)
            .add_plugins(UnivisIconButtonPlugin)
            .add_plugins(UnivisTogglePlugin)
            .add_plugins(UnivisCheckboxPlugin)
            .add_plugins(UnivisSeekBarPlugin)
            .add_plugins(UnivisScrollViewPlugin)
            .add_plugins(UnivisDividerPlugin)
            .add_plugins(UnivisPanelPlugin)
            // NOTE: UnivisBadgePlugin is intentionally optional and must be added explicitly.
            .add_plugins(UnivisDragValuePlugin)
            .add_plugins(UnivisSelectPlugin);
    }
}

fn warn_on_missing_optional_widget_plugins(
    added_text_fields: Query<(), Added<UTextField>>,
    added_badges: Query<(), Added<UBadge>>,
    added_tags: Query<(), Added<UTag>>,
    text_field_plugin: Option<Res<TextFieldPluginInstalled>>,
    badge_plugin: Option<Res<BadgePluginInstalled>>,
    mut warnings: Local<MissingOptionalWidgetPluginWarnings>,
) {
    if text_field_plugin.is_none()
        && !warnings.text_field_missing_plugin
        && !added_text_fields.is_empty()
    {
        bevy::log::warn!(
            "UTextField detected, but UnivisTextFieldPlugin is not added. Add .add_plugins(UnivisTextFieldPlugin) to enable text-field behavior and events."
        );
        warnings.text_field_missing_plugin = true;
    }

    if badge_plugin.is_none() && !warnings.badge_missing_plugin && !added_badges.is_empty() {
        bevy::log::warn!(
            "UBadge detected, but UnivisBadgePlugin is not added. Add .add_plugins(UnivisBadgePlugin) to enable badge visual update systems."
        );
        warnings.badge_missing_plugin = true;
    }

    if !warnings.tag_runtime_limited && !added_tags.is_empty() {
        bevy::log::warn!(
            "UTag detected. UTag runtime systems are currently limited; validate behavior in your scene."
        );
        warnings.tag_runtime_limited = true;
    }
}
