//! ECS systems for keyboard input, caret animation, and visual synchronization.

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy::prelude::*;
use univis_ui_engine::layout::{
    geometry::{ComputedSize, UVal},
    text::{UTextLabel, UTextLabelLayoutCache},
    univis_node::{UNode, USelf},
};

use super::{
    actions::{
        clear_selection, cursor_index_from_x_offset, delete_backward, delete_forward,
        delete_word_backward, display_text, insert_text, move_cursor_left, move_cursor_right,
        move_cursor_to_end, move_cursor_to_start, select_all, select_word_at_cursor,
        selection_bounds,
    },
    types::{
        UTextInput, UTextInputCancelled, UTextInputCaret, UTextInputCaretMarker, UTextInputChanged,
        UTextInputSelectionMarker, UTextInputSubmit,
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

/// Synchronizes the visual child text label, caret, and selection highlight with the [`UTextInput`] state.
pub fn text_input_sync_visual_system(
    focus_state: Res<UFocusState>,
    inputs: Query<(Entity, &UTextInput, &UTextInputCaret)>,
    mut label_query: Query<(&mut UTextLabel, Option<&UTextLabelLayoutCache>)>,
    mut caret_query: Query<(&mut UNode, &mut USelf, &mut Visibility), With<UTextInputCaretMarker>>,
    mut selection_query: Query<
        (&mut UNode, &mut USelf, &mut Visibility),
        (
            With<UTextInputSelectionMarker>,
            Without<UTextInputCaretMarker>,
        ),
    >,
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

        let padding_left = 14.0;

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

        // 3. Update selection highlight quad position and visibility
        if let Some(sel_entity) = caret.selection_entity
            && let Ok((mut sel_node, mut sel_self, mut sel_vis)) =
                selection_query.get_mut(sel_entity)
        {
            if let Some((start_byte, end_byte)) = selection_bounds(input) {
                let total_bytes = input.value.len().max(1) as f32;
                let start_ratio = (start_byte as f32 / total_bytes).clamp(0.0, 1.0);
                let end_ratio = (end_byte as f32 / total_bytes).clamp(0.0, 1.0);

                let start_x = if measured_width > 0.0 {
                    padding_left + measured_width * start_ratio
                } else {
                    padding_left + start_byte as f32 * (input.font_size * 0.55)
                };
                let end_x = if measured_width > 0.0 {
                    padding_left + measured_width * end_ratio
                } else {
                    padding_left + end_byte as f32 * (input.font_size * 0.55)
                };
                let sel_w = (end_x - start_x).max(2.0);

                let target_w = UVal::Px(sel_w);
                if sel_node.width != target_w {
                    sel_node.width = target_w;
                }
                let target_left = UVal::Px(start_x);
                if sel_self.left != target_left {
                    sel_self.left = target_left;
                }
                let sel_h = caret.height.unwrap_or(input.font_size * 1.2);
                let target_h = UVal::Px(sel_h);
                if sel_node.height != target_h {
                    sel_node.height = target_h;
                }
                if sel_node.background_color != caret.selection_color {
                    sel_node.background_color = caret.selection_color;
                }
                if *sel_vis != Visibility::Inherited {
                    *sel_vis = Visibility::Inherited;
                }
            } else if *sel_vis != Visibility::Hidden {
                *sel_vis = Visibility::Hidden;
            }
        }
    }
}

fn hit_test_cursor(
    hit_world: Vec3,
    global_transform: &GlobalTransform,
    computed_size: &ComputedSize,
    input: &UTextInput,
    measured_width: f32,
) -> usize {
    let local_hit = global_transform
        .to_matrix()
        .inverse()
        .transform_point3(hit_world)
        .truncate();
    let padding_left = 14.0;
    let x_from_left = local_hit.x + computed_size.width / 2.0;
    let x_in_text = (x_from_left - padding_left).max(0.0);
    cursor_index_from_x_offset(input, x_in_text, measured_width)
}

/// Observer handling pointer down / press to position the caret, initiate drag selection,
/// or double-click to select word.
pub fn on_text_input_pointer_press(
    trigger: On<Pointer<Press>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut focus_state: ResMut<UFocusState>,
    mut query: Query<(
        Entity,
        &mut UTextInput,
        &mut UTextInputCaret,
        &UFocusable,
        &GlobalTransform,
        &ComputedSize,
    )>,
    label_query: Query<&UTextLabelLayoutCache>,
) {
    let entity = trigger.entity;
    let Ok((_, mut input, mut caret, focusable, gt, computed)) = query.get_mut(entity) else {
        return;
    };

    if !focusable.enabled || !input.enabled {
        return;
    }

    set_focus(
        &mut commands,
        &mut focus_state,
        Some(entity),
        FocusModality::Pointer,
    );

    let measured_width = caret
        .text_entity
        .and_then(|te| label_query.get(te).ok())
        .map_or(0.0, |cache| cache.measured_size.x);

    let hit_world = trigger.event().hit.position.unwrap_or(gt.translation());
    let new_cursor = hit_test_cursor(hit_world, gt, computed, &input, measured_width);

    let now = time.elapsed_secs_f64();
    let is_double_click =
        (now - caret.last_click_secs) < 0.35 && (now - caret.last_click_secs) > 0.0;
    caret.last_click_secs = now;

    if is_double_click {
        input.cursor_position = new_cursor;
        select_word_at_cursor(&mut input);
        caret.drag_anchor = None;
    } else if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        let anchor = input
            .selection
            .map(|(s, _)| s)
            .unwrap_or(input.cursor_position);
        input.cursor_position = new_cursor;
        if anchor != new_cursor {
            input.selection = Some((anchor, new_cursor));
        } else {
            input.selection = None;
        }
        caret.drag_anchor = Some(anchor);
    } else {
        input.cursor_position = new_cursor;
        input.selection = None;
        caret.drag_anchor = Some(new_cursor);
    }

    caret.reset_blink();
}

/// Observer handling pointer dragging across the text input to update selection range.
pub fn on_text_input_pointer_drag(
    trigger: On<Pointer<Drag>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(
        Entity,
        &mut UTextInput,
        &mut UTextInputCaret,
        &GlobalTransform,
        &ComputedSize,
    )>,
    label_query: Query<&UTextLabelLayoutCache>,
) {
    let entity = trigger.entity;
    let Ok((_, mut input, mut caret, gt, computed)) = query.get_mut(entity) else {
        return;
    };

    if !input.enabled {
        return;
    }

    let Some(anchor) = caret.drag_anchor else {
        return;
    };

    let measured_width = caret
        .text_entity
        .and_then(|te| label_query.get(te).ok())
        .map_or(0.0, |cache| cache.measured_size.x);

    let cursor_world_2d = cameras.iter().find_map(|(camera, cam_gt)| {
        camera
            .viewport_to_world_2d(cam_gt, trigger.pointer_location.position)
            .ok()
    });

    let hit_world = cursor_world_2d
        .map(|pos| Vec3::new(pos.x, pos.y, gt.translation().z))
        .unwrap_or(gt.translation());
    let drag_cursor = hit_test_cursor(hit_world, gt, computed, &input, measured_width);

    input.cursor_position = drag_cursor;
    if anchor != drag_cursor {
        input.selection = Some((anchor, drag_cursor));
    } else {
        input.selection = None;
    }

    caret.reset_blink();
}

/// Observer clearing the drag anchor when the pointer is released.
pub fn on_text_input_pointer_release(
    trigger: On<Pointer<Release>>,
    mut query: Query<(&mut UTextInput, &mut UTextInputCaret)>,
) {
    let entity = trigger.entity;
    if let Ok((mut input, mut caret)) = query.get_mut(entity) {
        caret.drag_anchor = None;
        if let Some((s, e)) = input.selection {
            if s == e {
                input.selection = None;
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
        caret.drag_anchor = None;
    }
}
