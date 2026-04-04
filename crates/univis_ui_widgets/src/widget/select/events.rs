use bevy::prelude::*;

use super::USelect;

/// Message emitted when the selected option changes.
#[derive(Message)]
pub struct SelectChangedEvent {
    pub entity: Entity,
    pub selected_index: usize,
    pub value: String,
    pub label: String,
}

/// Message emitted when the dropdown opens or closes.
#[derive(Message)]
pub struct SelectOpenStateChangedEvent {
    pub entity: Entity,
    pub is_open: bool,
}

pub(super) fn emit_select_events(
    mut changed_events: MessageWriter<SelectChangedEvent>,
    mut open_events: MessageWriter<SelectOpenStateChangedEvent>,
    mut query: Query<(Entity, &mut USelect)>,
) {
    for (entity, mut select) in query.iter_mut() {
        if select.selected_index != select.previous_selected_index {
            if let Some(index) = select.selected_index
                && let Some(option) = select.options.get(index)
                && !option.disabled
            {
                changed_events.write(SelectChangedEvent {
                    entity,
                    selected_index: index,
                    value: option.value.clone(),
                    label: option.label.clone(),
                });
            }
            select.previous_selected_index = select.selected_index;
        }

        if select.is_open != select.previous_open {
            open_events.write(SelectOpenStateChangedEvent {
                entity,
                is_open: select.is_open,
            });
            select.previous_open = select.is_open;
        }
    }
}
