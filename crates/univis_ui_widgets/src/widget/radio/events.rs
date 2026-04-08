use super::*;

use crate::widget::radio::model::{URadioButton, URadioGroup};

#[derive(Message)]
pub struct RadioButtonChangedEvent {
    pub entity: Entity,
    pub value: String,
    pub checked: bool,
    pub group_entity: Option<Entity>,
    pub group_value: Option<String>,
}

pub(super) fn emit_radio_events(
    mut events: MessageWriter<RadioButtonChangedEvent>,
    mut radio_query: Query<(Entity, &mut URadioButton)>,
    group_query: Query<&URadioGroup>,
) {
    for (entity, mut radio) in radio_query.iter_mut() {
        if radio.checked != radio.previous_checked {
            let group_value = radio
                .group
                .and_then(|group| group_query.get(group).ok())
                .and_then(|group| group.selected_value.clone());

            events.write(RadioButtonChangedEvent {
                entity,
                value: radio.value.clone(),
                checked: radio.checked,
                group_entity: radio.group,
                group_value,
            });

            radio.previous_checked = radio.checked;
        }
    }
}
