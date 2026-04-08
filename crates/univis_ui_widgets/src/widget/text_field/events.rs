use super::*;

use crate::widget::text_field::model::UTextField;

/// Message emitted whenever the text buffer changes.
#[derive(Message)]
pub struct TextFieldChangedEvent {
    pub entity: Entity,
    pub text: String,
}

/// Message emitted when the user submits the current text.
#[derive(Message)]
pub struct TextFieldSubmitEvent {
    pub entity: Entity,
    pub text: String,
}

pub(super) fn emit_textfield_events(
    mut changed_events: MessageWriter<TextFieldChangedEvent>,
    mut submit_events: MessageWriter<TextFieldSubmitEvent>,
    mut textfield_query: Query<(Entity, &mut UTextField)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (entity, mut textfield) in textfield_query.iter_mut() {
        if textfield.text != textfield.previous_text {
            changed_events.write(TextFieldChangedEvent {
                entity,
                text: textfield.text.clone(),
            });

            textfield.previous_text = textfield.text.clone();
        }

        if textfield.focused && keyboard.just_pressed(KeyCode::Enter) {
            submit_events.write(TextFieldSubmitEvent {
                entity,
                text: textfield.text.clone(),
            });
        }
    }
}
