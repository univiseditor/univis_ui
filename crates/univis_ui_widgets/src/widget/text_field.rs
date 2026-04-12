mod events;
mod model;
mod runtime;
#[cfg(test)]
mod tests;
mod visuals;

use bevy::prelude::*;

use self::events::emit_textfield_events;
use self::runtime::{handle_global_unfocus, handle_textfield_input};
use self::visuals::{animate_textfield_cursor, init_textfield_visuals, update_textfield_visuals};
use crate::schedule::UnivisWidgetUpdateSet;

pub use self::events::{TextFieldChangedEvent, TextFieldSubmitEvent};
pub(crate) use self::model::TextFieldPluginInstalled;
pub use self::model::{TextFieldInputType, UTextField};

/// Registers `UTextField`, its focus/input behavior, and the related messages.
pub struct UnivisTextFieldPlugin;

impl Plugin for UnivisTextFieldPlugin {
    fn build(&self, app: &mut App) {
        if app
            .world()
            .get_resource::<TextFieldPluginInstalled>()
            .is_some()
        {
            return;
        }

        app.init_resource::<TextFieldPluginInstalled>()
            .register_type::<UTextField>()
            .add_message::<TextFieldChangedEvent>()
            .add_message::<TextFieldSubmitEvent>()
            .add_systems(
                Update,
                init_textfield_visuals.in_set(UnivisWidgetUpdateSet::Build),
            )
            .add_systems(
                Update,
                (handle_global_unfocus, handle_textfield_input)
                    .chain()
                    .in_set(UnivisWidgetUpdateSet::Logic),
            )
            .add_systems(
                Update,
                (update_textfield_visuals, animate_textfield_cursor)
                    .chain()
                    .in_set(UnivisWidgetUpdateSet::Visual),
            )
            .add_systems(
                Update,
                emit_textfield_events.in_set(UnivisWidgetUpdateSet::Events),
            );
    }

    fn is_unique(&self) -> bool {
        false
    }
}
