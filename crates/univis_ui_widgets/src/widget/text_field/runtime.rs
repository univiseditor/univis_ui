use super::*;
use bevy::input::{ButtonState, keyboard::*};

use crate::widget::text_field::model::{TextFieldInputType, UTextField};

#[derive(Component)]
pub(super) struct TextFieldPressedThisFrame;

pub(super) fn on_textfield_click(
    trigger: On<Pointer<Press>>,
    mut commands: Commands,
    mut textfield_query: Query<(Entity, &mut UTextField)>,
) {
    let pressed_entity = trigger.entity.entity();

    for (entity, mut textfield) in textfield_query.iter_mut() {
        if entity == pressed_entity {
            commands.entity(entity).insert(TextFieldPressedThisFrame);
            if !textfield.disabled && !textfield.readonly {
                textfield.focused = true;
                textfield.cursor_visible = true;
                textfield.cursor_blink_timer = 0.0;
            }
        } else {
            textfield.focused = false;
        }
    }
}

pub(super) fn handle_global_unfocus(
    mut commands: Commands,
    mut textfield_query: Query<(Entity, &mut UTextField, Option<&TextFieldPressedThisFrame>)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, mut textfield, pressed_this_frame) in textfield_query.iter_mut() {
        if pressed_this_frame.is_none() {
            textfield.focused = false;
        }

        if pressed_this_frame.is_some() {
            commands
                .entity(entity)
                .remove::<TextFieldPressedThisFrame>();
        }
    }
}

pub(super) fn handle_textfield_input(
    mut textfield_query: Query<&mut UTextField>,
    mut keyboard_events: MessageReader<KeyboardInput>,
) {
    let mut focused_textfield: Option<Mut<UTextField>> = None;
    for textfield in textfield_query.iter_mut() {
        if textfield.focused && !textfield.disabled && !textfield.readonly {
            focused_textfield = Some(textfield);
            break;
        }
    }

    let Some(mut textfield) = focused_textfield else {
        return;
    };

    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {}
            Key::Backspace => {
                let cursor_position = textfield.cursor_position;
                if textfield.cursor_position > 0 && !textfield.text.is_empty() {
                    textfield.text.remove(cursor_position - 1);
                    textfield.cursor_position -= 1;
                }
            }
            Key::Delete => {
                let cursor_position = textfield.cursor_position;
                if textfield.cursor_position < textfield.text.len() {
                    textfield.text.remove(cursor_position);
                }
            }
            Key::ArrowLeft if textfield.cursor_position > 0 => {
                textfield.cursor_position -= 1;
            }
            Key::ArrowRight if textfield.cursor_position < textfield.text.len() => {
                textfield.cursor_position += 1;
            }
            Key::Home => {
                textfield.cursor_position = 0;
            }
            Key::End => {
                textfield.cursor_position = textfield.text.len();
            }
            Key::Character(char_str) => {
                if let Some(max) = textfield.max_length
                    && textfield.text.len() >= max
                {
                    continue;
                }

                let valid = match textfield.input_type {
                    TextFieldInputType::Number => char_str
                        .chars()
                        .all(|c| c.is_numeric() || c == '.' || c == '-'),
                    TextFieldInputType::Email => char_str.chars().all(|c| {
                        c.is_alphanumeric() || c == '@' || c == '.' || c == '_' || c == '-'
                    }),
                    _ => true,
                };

                if valid {
                    let cursor_position = textfield.cursor_position;
                    textfield.text.insert_str(cursor_position, char_str);
                    textfield.cursor_position += char_str.len();
                }
            }
            _ => {}
        }

        textfield.cursor_visible = true;
        textfield.cursor_blink_timer = 0.0;
    }
}
