use bevy::prelude::*;
use crate::prelude::*;
use crate::widget::{
    badge::BadgePluginInstalled,
    text_field::TextFieldPluginInstalled,
};

pub mod button;
pub mod text_label;
pub mod badge;
pub mod progress;
pub mod image;
pub mod seekbar;
pub mod checkbox;
mod menu; // Internal placeholder module; not part of public API yet.
pub mod icon_btn;
pub mod toggle;
pub mod radio;
pub mod text_field;
pub mod scroll_view;
pub mod divider;
pub mod panel;
pub mod drag_value;
pub mod select;

pub mod prelude {
    pub use crate::widget::{
        text_label::*,
        badge::*,
        progress::*,
        button::*,
        image::*,
        text_field::*,
        checkbox::*,
        radio::*,
        // menu::*,
        scroll_view::*,
        seekbar::*,
        icon_btn::*,
        toggle::*,
        divider::*,
        panel::*,
        drag_value::*,
        select::*,
    };
}

pub struct UnivisWidgetPlugin;

#[derive(Default)]
struct MissingOptionalWidgetPluginWarnings {
    text_field_missing_plugin: bool,
    badge_missing_plugin: bool,
    tag_runtime_limited: bool,
}

impl Plugin for UnivisWidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .register_type::<UImage>()
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

    if badge_plugin.is_none()
        && !warnings.badge_missing_plugin
        && !added_badges.is_empty()
    {
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
