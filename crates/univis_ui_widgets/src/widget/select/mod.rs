use crate::schedule::UnivisWidgetUpdateSet;
use bevy::prelude::*;

mod events;
mod interaction;
mod model;
mod runtime;
mod visuals;

use self::events::emit_select_events;
pub use self::events::{SelectChangedEvent, SelectOpenStateChangedEvent};
use self::interaction::{
    close_select_on_outside_click, enforce_select_invariants, handle_select_keyboard,
    handle_select_option_interaction, handle_select_trigger_interaction, sync_select_dropdown_tree,
};
pub use self::model::{USelect, USelectOption};
use self::model::{
    first_enabled_index, is_enabled_index, next_enabled_index, sanitize_select, selected_option,
};
use self::runtime::{
    ActiveSelect, SelectChevronLabel, SelectDropdown, SelectOptionLabel, SelectOptionRow,
    SelectRuntime, SelectTrigger, SelectValueLabel, init_select_visuals,
};
use self::visuals::update_select_visuals;

/// Registers the select / dropdown widget and its messages.
pub struct UnivisSelectPlugin;

impl Plugin for UnivisSelectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<USelect>()
            .register_type::<USelectOption>()
            .add_message::<SelectChangedEvent>()
            .add_message::<SelectOpenStateChangedEvent>()
            .init_resource::<ActiveSelect>()
            .add_systems(
                Update,
                init_select_visuals.in_set(UnivisWidgetUpdateSet::Build),
            )
            .add_systems(
                Update,
                (sync_select_dropdown_tree, update_select_visuals)
                    .chain()
                    .in_set(UnivisWidgetUpdateSet::Visual),
            )
            .add_systems(
                Update,
                emit_select_events.in_set(UnivisWidgetUpdateSet::Events),
            )
            .add_systems(
                Update,
                (
                    enforce_select_invariants,
                    handle_select_trigger_interaction,
                    handle_select_option_interaction,
                    handle_select_keyboard,
                    close_select_on_outside_click,
                )
                    .chain()
                    .in_set(UnivisWidgetUpdateSet::Logic),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_options() -> Vec<USelectOption> {
        vec![
            USelectOption::new("One", "one"),
            USelectOption::new("Two", "two").disabled(),
            USelectOption::new("Three", "three"),
        ]
    }

    #[test]
    fn first_enabled_index_returns_first_enabled() {
        let options = sample_options();
        assert_eq!(first_enabled_index(&options), Some(0));
    }

    #[test]
    fn next_enabled_index_skips_disabled_and_wraps() {
        let options = sample_options();
        assert_eq!(next_enabled_index(&options, Some(0), 1), Some(2));
        assert_eq!(next_enabled_index(&options, Some(2), 1), Some(0));
        assert_eq!(next_enabled_index(&options, Some(0), -1), Some(2));
    }

    #[test]
    fn with_selected_value_matches_existing_option() {
        let select = USelect::new()
            .with_options(sample_options())
            .with_selected_value("three");
        assert_eq!(select.selected_index, Some(2));
    }

    #[test]
    fn enforce_select_invariants_resets_out_of_range_index() {
        let mut select = USelect::new().with_options(sample_options());
        select.selected_index = Some(99);
        sanitize_select(&mut select);
        assert_eq!(select.selected_index, None);
    }

    #[test]
    fn keyboard_navigation_never_targets_disabled_option() {
        let options = sample_options();
        let next = next_enabled_index(&options, Some(0), 1);
        assert_eq!(next, Some(2));
        assert!(!is_enabled_index(&options, Some(1)));
    }
}
