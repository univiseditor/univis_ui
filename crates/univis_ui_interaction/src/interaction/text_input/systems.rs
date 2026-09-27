//! ECS systems for keyboard input, caret animation, and visual synchronization.

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy::prelude::*;
use univis_ui_engine::layout::{
    geometry::UVal,
    text::{UTextLabel, UTextLabelLayoutCache},
    univis_node::{UNode, USelf},
};

use super::{
    actions::{
        clear_selection, delete_backward, delete_forward, delete_word_backward, display_text,
        insert_text, move_cursor_left, move_cursor_right, move_cursor_to_end, move_cursor_to_start,
        select_all,
    },
    types::{
        UTextInput, UTextInputCancelled, UTextInputCaret, UTextInputChanged, UTextInputSubmit,
    },
};
use crate::interaction::focus::{FocusLost, FocusModality, UFocusState, UFocusable, set_focus};

/// Keyboard processing system driving active text input fields.
pub fn text_input_keyboard_system(
    mut commands: Commands,
    focus_state: Res<UFocusState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut query: Query<(Entity, &mut UTextInput, &mut UTextInputCaret, &UFocusable)>,
) {
    let Some(focused_entity) = focus_state.focused else {
        return;
    };

    let Ok((entity, mut input, mut caret, focusable)) = query.get_mut(focused_entity) else {
        return;
    };

    if !focusable.enabled || !input.enabled {
        return;
    }

    let ctrl = keyboard.pressed(KeyCode::ControlLeft)
        || keyboard.pressed(KeyCode::ControlRight)
        || keyboard.pressed(KeyCode::SuperLeft)
        || keyboard.pressed(KeyCode::SuperRight);

    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    for ev in keyboard_events.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }

        if ctrl {
            match ev.key_code {
                KeyCode::KeyA => {
                    select_all(&mut input);
                    caret.reset_blink();
                }
                KeyCode::Backspace => {
                    if delete_word_backward(&mut input) {
                        commands.trigger(UTextInputChanged {
                            entity,
                            value: input.value.clone(),
                        });
                    }
                    caret.reset_blink();
                }
                _ => {}
            }
            continue;
        }

        match ev.key_code {
            KeyCode::Backspace => {
                if delete_backward(&mut input) {
                    commands.trigger(UTextInputChanged {
                        entity,
                        value: input.value.clone(),
                    });
                }
                caret.reset_blink();
            }
            KeyCode::Delete => {
                if delete_forward(&mut input) {
                    commands.trigger(UTextInputChanged {
                        entity,
                        value: input.value.clone(),
                    });
                }
                caret.reset_blink();
            }
            KeyCode::ArrowLeft => {
                move_cursor_left(&mut input, shift);
                caret.reset_blink();
            }
            KeyCode::ArrowRight => {
                move_cursor_right(&mut input, shift);
                caret.reset_blink();
            }
            KeyCode::Home => {
                move_cursor_to_start(&mut input, shift);
                caret.reset_blink();
            }
            KeyCode::End => {
                move_cursor_to_end(&mut input, shift);
                caret.reset_blink();
            }
            KeyCode::Enter | KeyCode::NumpadEnter => {
                commands.trigger(UTextInputSubmit {
                    entity,
                    value: input.value.clone(),
                });

                if input.clear_on_submit {
                    input.value.clear();
                    input.cursor_position = 0;
                    input.selection = None;
                    commands.trigger(UTextInputChanged {
                        entity,
                        value: String::new(),
                    });
                }
                caret.reset_blink();
            }
            KeyCode::Escape => {
                clear_selection(&mut input);
                commands.trigger(UTextInputCancelled { entity });
            }
            _ => {
                if let Key::Character(ref smol) = ev.logical_key {
                    let s = smol.as_str();
                    if !s.is_empty() && !s.chars().any(|c| c.is_control()) {
                        if insert_text(&mut input, s) {
                            commands.trigger(UTextInputChanged {
                                entity,
                                value: input.value.clone(),
                            });
                        }
                        caret.reset_blink();
                    }
                }
            }
        }
    }
}

/// Blinks the caret cursor on active focused inputs.
pub fn text_input_caret_blink_system(
    time: Res<Time>,
    focus_state: Res<UFocusState>,
    mut carets: Query<(Entity, &mut UTextInputCaret)>,
) {
    for (entity, mut caret) in carets.iter_mut() {
        if focus_state.focused == Some(entity) {
            caret.timer.tick(time.delta());
            if caret.timer.just_finished() {
                caret.visible = !caret.visible;
            }
        } else {
            caret.visible = false;
        }
    }
}

/// Synchronizes the visual child text label and caret node with the [`UTextInput`] state.
pub fn text_input_sync_visual_system(
    focus_state: Res<UFocusState>,
    inputs: Query<(Entity, &UTextInput, &UTextInputCaret)>,
    mut label_query: Query<(&mut UTextLabel, Option<&UTextLabelLayoutCache>)>,
    mut caret_query: Query<(&mut UNode, &mut USelf, &mut Visibility)>,
) {
    for (entity, input, caret) in inputs.iter() {
        let is_focused = focus_state.focused == Some(entity);

        // 1. Update text label
        let mut measured_width = 0.0;
        if let Some(text_entity) = caret.text_entity
            && let Ok((mut label, cache_opt)) = label_query.get_mut(text_entity)
        {
            let (target_text, target_color) = if input.value.is_empty() {
                (input.placeholder.as_str(), input.placeholder_color)
            } else {
                (&display_text(input) as &str, input.text_color)
            };

            if label.text != target_text {
                label.text = target_text.to_string();
            }
            if label.color != target_color {
                label.color = target_color;
            }
            if label.font_size != input.font_size {
                label.font_size = input.font_size;
            }

            if let Some(cache) = cache_opt {
                measured_width = cache.measured_size.x;
            }
        }

        // 2. Update caret node position and visibility
        if let Some(caret_entity) = caret.caret_entity
            && let Ok((mut caret_node, mut caret_self, mut visibility)) =
                caret_query.get_mut(caret_entity)
        {
            // Visibility
            let target_vis = if is_focused && caret.visible && input.enabled {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            if *visibility != target_vis {
                *visibility = target_vis;
            }

            // Caret color & dimensions
            if caret_node.background_color != caret.color {
                caret_node.background_color = caret.color;
            }

            let caret_h = caret.height.unwrap_or(input.font_size * 1.2);
            let target_h = UVal::Px(caret_h);
            if caret_node.height != target_h {
                caret_node.height = target_h;
            }

            let target_w = UVal::Px(caret.width);
            if caret_node.width != target_w {
                caret_node.width = target_w;
            }

            // Horizontal position calculation
            let padding_left = 14.0;
            let x_pos = if input.value.is_empty() || input.cursor_position == 0 {
                padding_left
            } else if measured_width > 0.0 {
                let total_bytes = input.value.len().max(1) as f32;
                let cursor_ratio = (input.cursor_position as f32 / total_bytes).clamp(0.0, 1.0);
                padding_left + measured_width * cursor_ratio
            } else {
                padding_left + input.cursor_position as f32 * (input.font_size * 0.55)
            };

            let target_left = UVal::Px(x_pos);
            if caret_self.left != target_left {
                caret_self.left = target_left;
            }
        }
    }
}

/// Observer focusing a text input field on click and resetting its caret blink.
pub fn on_text_input_pointer_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut focus_state: ResMut<UFocusState>,
    mut query: Query<(&mut UTextInput, &mut UTextInputCaret, &UFocusable)>,
) {
    let entity = trigger.entity;
    if let Ok((_, mut caret, focusable)) = query.get_mut(entity)
        && focusable.enabled
    {
        set_focus(
            &mut commands,
            &mut focus_state,
            Some(entity),
            FocusModality::Pointer,
        );
        caret.reset_blink();
    }
}

/// Observer clearing selections when a text input loses focus.
pub fn on_text_input_focus_lost(
    trigger: On<FocusLost>,
    mut query: Query<(&mut UTextInput, &mut UTextInputCaret)>,
) {
    let entity = trigger.entity;
    if let Ok((mut input, mut caret)) = query.get_mut(entity) {
        clear_selection(&mut input);
        caret.visible = false;
    }
}
