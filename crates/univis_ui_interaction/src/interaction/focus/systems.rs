//! Navigation systems for keyboard and gamepad input handling.

use bevy::prelude::*;
use core::time::Duration;
use univis_ui_engine::layout::{
    geometry::ComputedSize,
    univis_node::{UDisplay, ULayout},
};

use super::{
    spatial::{find_best_spatial_candidate, find_initial_focus_candidate, find_next_tab_candidate},
    types::{
        FocusModality, NavDirection, UFocusActivate, UFocusNavigationSettings, UFocusState,
        UFocusable, set_focus,
    },
};

/// Cleanup system verifying that the currently focused entity remains valid and enabled.
pub fn focus_cleanup_system(
    mut commands: Commands,
    mut focus_state: ResMut<UFocusState>,
    query: Query<(&UFocusable, Option<&ULayout>, Option<&Visibility>)>,
) {
    if let Some(focused) = focus_state.focused {
        if let Ok((focusable, layout_opt, vis_opt)) = query.get(focused) {
            let is_visible = vis_opt.is_none_or(|v| *v != Visibility::Hidden)
                && layout_opt.is_none_or(|l| l.display != UDisplay::None);

            if !focusable.enabled || !is_visible {
                let modality = focus_state.modality;
                set_focus(&mut commands, &mut focus_state, None, modality);
            }
        } else {
            focus_state.focused = None;
            focus_state.held_direction = None;
        }
    }
}

/// System processing keyboard navigation inputs (Tab, Arrow keys, Enter, Space).
pub fn focus_keyboard_navigation_system(
    mut commands: Commands,
    keyboard_input_opt: Option<Res<ButtonInput<KeyCode>>>,
    time_opt: Option<Res<Time>>,
    settings: Res<UFocusNavigationSettings>,
    mut focus_state: ResMut<UFocusState>,
    focusable_query: Query<(
        Entity,
        &UFocusable,
        &GlobalTransform,
        Option<&ComputedSize>,
        Option<&ULayout>,
        Option<&Visibility>,
    )>,
) {
    let Some(keyboard) = keyboard_input_opt else {
        return;
    };
    let delta = time_opt
        .as_ref()
        .map_or(Duration::from_millis(16), |t| t.delta());

    // 1. Activation (Enter / Space)
    if settings.activation
        && (keyboard.just_pressed(KeyCode::Enter)
            || keyboard.just_pressed(KeyCode::NumpadEnter)
            || keyboard.just_pressed(KeyCode::Space))
        && let Some(focused) = focus_state.focused
    {
        commands
            .entity(focused)
            .trigger(|entity| UFocusActivate { entity });
    }

    // 2. Tab Navigation
    if settings.tab_navigation && keyboard.just_pressed(KeyCode::Tab) {
        let backward =
            keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        let candidates: Vec<(Entity, Option<i32>, Vec2)> = focusable_query
            .iter()
            .filter(|(_, focusable, _, _, layout_opt, vis_opt)| {
                focusable.enabled
                    && vis_opt.is_none_or(|v| *v != Visibility::Hidden)
                    && layout_opt.is_none_or(|l| l.display != UDisplay::None)
            })
            .map(|(e, f, t, _, _, _)| (e, f.tab_index, t.translation().xy()))
            .collect();

        if let Some(target) = find_next_tab_candidate(focus_state.focused, backward, &candidates) {
            set_focus(
                &mut commands,
                &mut focus_state,
                Some(target),
                FocusModality::Keyboard,
            );
        }
        return;
    }

    // 3. Directional Spatial Navigation (Arrow keys)
    if !settings.spatial_navigation {
        return;
    }

    let up = keyboard.pressed(KeyCode::ArrowUp);
    let down = keyboard.pressed(KeyCode::ArrowDown);
    let left = keyboard.pressed(KeyCode::ArrowLeft);
    let right = keyboard.pressed(KeyCode::ArrowRight);

    let just_up = keyboard.just_pressed(KeyCode::ArrowUp);
    let just_down = keyboard.just_pressed(KeyCode::ArrowDown);
    let just_left = keyboard.just_pressed(KeyCode::ArrowLeft);
    let just_right = keyboard.just_pressed(KeyCode::ArrowRight);

    let requested_dir = if just_right || (right && !left && !up && !down) {
        Some((NavDirection::Right, just_right))
    } else if just_left || (left && !right && !up && !down) {
        Some((NavDirection::Left, just_left))
    } else if just_up || (up && !down && !left && !right) {
        Some((NavDirection::Up, just_up))
    } else if just_down || (down && !up && !left && !right) {
        Some((NavDirection::Down, just_down))
    } else {
        None
    };

    let Some((direction, just_pressed)) = requested_dir else {
        focus_state.held_direction = None;
        focus_state.repeat_repeating = false;
        return;
    };

    let mut should_navigate = just_pressed;

    if just_pressed {
        focus_state.held_direction = Some(direction);
        focus_state.repeat_repeating = false;
        focus_state
            .repeat_timer
            .set_duration(Duration::from_secs_f32(settings.repeat_delay));
        focus_state.repeat_timer.reset();
    } else if focus_state.held_direction == Some(direction) {
        focus_state.repeat_timer.tick(delta);
        if focus_state.repeat_timer.just_finished() {
            should_navigate = true;
            if !focus_state.repeat_repeating {
                focus_state.repeat_repeating = true;
                focus_state
                    .repeat_timer
                    .set_duration(Duration::from_secs_f32(settings.repeat_rate));
            }
            focus_state.repeat_timer.reset();
        }
    } else {
        focus_state.held_direction = Some(direction);
        focus_state.repeat_repeating = false;
        focus_state
            .repeat_timer
            .set_duration(Duration::from_secs_f32(settings.repeat_delay));
        focus_state.repeat_timer.reset();
    }

    if !should_navigate {
        return;
    }

    execute_spatial_navigation(
        &mut commands,
        &mut focus_state,
        direction,
        FocusModality::Keyboard,
        settings.wrap_around,
        &focusable_query,
    );
}

/// System processing gamepad navigation inputs (D-pad, Left Stick, South button).
pub fn focus_gamepad_navigation_system(
    mut commands: Commands,
    time_opt: Option<Res<Time>>,
    settings: Res<UFocusNavigationSettings>,
    mut focus_state: ResMut<UFocusState>,
    gamepads: Query<&Gamepad>,
    focusable_query: Query<(
        Entity,
        &UFocusable,
        &GlobalTransform,
        Option<&ComputedSize>,
        Option<&ULayout>,
        Option<&Visibility>,
    )>,
) {
    let delta = time_opt
        .as_ref()
        .map_or(Duration::from_millis(16), |t| t.delta());

    for gamepad in gamepads.iter() {
        // 1. Activation (South button: Xbox A / PlayStation Cross / Nintendo B)
        if settings.activation
            && gamepad.just_pressed(GamepadButton::South)
            && let Some(focused) = focus_state.focused
        {
            commands
                .entity(focused)
                .trigger(|entity| UFocusActivate { entity });
        }

        if !settings.spatial_navigation {
            continue;
        }

        // 2. D-pad Discrete Inputs
        let just_up = gamepad.just_pressed(GamepadButton::DPadUp);
        let just_down = gamepad.just_pressed(GamepadButton::DPadDown);
        let just_left = gamepad.just_pressed(GamepadButton::DPadLeft);
        let just_right = gamepad.just_pressed(GamepadButton::DPadRight);

        let dpad_up = gamepad.pressed(GamepadButton::DPadUp);
        let dpad_down = gamepad.pressed(GamepadButton::DPadDown);
        let dpad_left = gamepad.pressed(GamepadButton::DPadLeft);
        let dpad_right = gamepad.pressed(GamepadButton::DPadRight);

        // 3. Left Stick Analog Inputs
        let stick = gamepad.left_stick();
        let stick_active = stick.length() > settings.stick_deadzone;

        let stick_dir = if stick_active {
            if stick.x.abs() > stick.y.abs() {
                if stick.x > 0.0 {
                    Some(NavDirection::Right)
                } else {
                    Some(NavDirection::Left)
                }
            } else if stick.y > 0.0 {
                Some(NavDirection::Up)
            } else {
                Some(NavDirection::Down)
            }
        } else {
            None
        };

        let requested_dir = if just_right
            || (dpad_right && !dpad_left)
            || stick_dir == Some(NavDirection::Right)
        {
            Some((NavDirection::Right, just_right))
        } else if just_left || (dpad_left && !dpad_right) || stick_dir == Some(NavDirection::Left) {
            Some((NavDirection::Left, just_left))
        } else if just_up || (dpad_up && !dpad_down) || stick_dir == Some(NavDirection::Up) {
            Some((NavDirection::Up, just_up))
        } else if just_down || (dpad_down && !dpad_up) || stick_dir == Some(NavDirection::Down) {
            Some((NavDirection::Down, just_down))
        } else {
            None
        };

        let Some((direction, just_pressed)) = requested_dir else {
            focus_state.held_direction = None;
            focus_state.repeat_repeating = false;
            continue;
        };

        let mut should_navigate = just_pressed;

        if just_pressed {
            focus_state.held_direction = Some(direction);
            focus_state.repeat_repeating = false;
            focus_state
                .repeat_timer
                .set_duration(Duration::from_secs_f32(settings.repeat_delay));
            focus_state.repeat_timer.reset();
        } else if focus_state.held_direction == Some(direction) {
            focus_state.repeat_timer.tick(delta);
            if focus_state.repeat_timer.just_finished() {
                should_navigate = true;
                if !focus_state.repeat_repeating {
                    focus_state.repeat_repeating = true;
                    focus_state
                        .repeat_timer
                        .set_duration(Duration::from_secs_f32(settings.repeat_rate));
                }
                focus_state.repeat_timer.reset();
            }
        } else {
            focus_state.held_direction = Some(direction);
            focus_state.repeat_repeating = false;
            focus_state
                .repeat_timer
                .set_duration(Duration::from_secs_f32(settings.repeat_delay));
            focus_state.repeat_timer.reset();
        }

        if should_navigate {
            execute_spatial_navigation(
                &mut commands,
                &mut focus_state,
                direction,
                FocusModality::Gamepad,
                settings.wrap_around,
                &focusable_query,
            );
        }
    }
}

fn execute_spatial_navigation(
    commands: &mut Commands,
    focus_state: &mut ResMut<UFocusState>,
    direction: NavDirection,
    modality: FocusModality,
    wrap_around: bool,
    query: &Query<(
        Entity,
        &UFocusable,
        &GlobalTransform,
        Option<&ComputedSize>,
        Option<&ULayout>,
        Option<&Visibility>,
    )>,
) {
    let is_node_active = |layout_opt: Option<&ULayout>, vis_opt: Option<&Visibility>| {
        vis_opt.is_none_or(|v| *v != Visibility::Hidden)
            && layout_opt.is_none_or(|l| l.display != UDisplay::None)
    };

    let Some(current_entity) = focus_state.focused else {
        let initial_candidates: Vec<(Entity, Option<i32>, Vec2)> = query
            .iter()
            .filter(|(_, focusable, _, _, layout_opt, vis_opt)| {
                focusable.enabled && is_node_active(*layout_opt, *vis_opt)
            })
            .map(|(e, f, t, _, _, _)| (e, f.tab_index, t.translation().xy()))
            .collect();

        if let Some(target) = find_initial_focus_candidate(&initial_candidates) {
            set_focus(commands, focus_state, Some(target), modality);
        }
        return;
    };

    let Ok((_, current_focusable, current_transform, current_size_opt, _, _)) =
        query.get(current_entity)
    else {
        return;
    };

    // 1. Check explicit direction overrides
    let explicit_target = match direction {
        NavDirection::Up => current_focusable.focus_up,
        NavDirection::Down => current_focusable.focus_down,
        NavDirection::Left => current_focusable.focus_left,
        NavDirection::Right => current_focusable.focus_right,
    };

    if let Some(target) = explicit_target
        && let Ok((_, target_focusable, _, _, target_layout, target_vis)) = query.get(target)
        && target_focusable.enabled
        && is_node_active(target_layout, target_vis)
    {
        set_focus(commands, focus_state, Some(target), modality);
        return;
    }

    // 2. Spatial candidate search
    let origin_pos = current_transform.translation().xy();
    let origin_size = current_size_opt
        .map(|c| c.size())
        .unwrap_or(Vec2::new(100.0, 40.0));

    let candidates: Vec<(Entity, Vec2, Vec2)> = query
        .iter()
        .filter(|(entity, focusable, _, _, layout_opt, vis_opt)| {
            *entity != current_entity && focusable.enabled && is_node_active(*layout_opt, *vis_opt)
        })
        .map(|(entity, _, transform, size_opt, _, _)| {
            let pos = transform.translation().xy();
            let size = size_opt.map(|c| c.size()).unwrap_or(Vec2::new(100.0, 40.0));
            (entity, pos, size)
        })
        .collect();

    if let Some(target) =
        find_best_spatial_candidate(origin_pos, origin_size, direction, &candidates, wrap_around)
    {
        set_focus(commands, focus_state, Some(target), modality);
    }
}
