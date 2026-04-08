mod events;
mod model;
mod runtime;
mod visuals;

use bevy::prelude::*;

use self::events::emit_radio_events;
use self::runtime::{animate_radio_check, on_radio_press, sync_radio_groups};
use self::visuals::{init_radio_visuals, update_radio_visuals};
use univis_ui_engine::layout::geometry::{UCornerRadius, UVal};
use univis_ui_engine::layout::univis_node::{
    UAlignItems, UBorder, UDisplay, UFlexDirection, UJustifyContent, ULayout, UNode,
};
use univis_ui_interaction::interaction::feedback::UInteraction;

pub use self::events::RadioButtonChangedEvent;
pub use self::model::{URadioButton, URadioGroup};
pub use self::visuals::create_radio_with_label;

/// Registers radio buttons, radio groups, and their change messages.
pub struct UnivisRadioPlugin;

impl Plugin for UnivisRadioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<URadioButton>()
            .register_type::<URadioGroup>()
            .add_message::<RadioButtonChangedEvent>()
            .add_systems(
                Update,
                (
                    init_radio_visuals,
                    sync_radio_groups,
                    update_radio_visuals,
                    animate_radio_check,
                    emit_radio_events,
                )
                    .chain(),
            );
    }
}
