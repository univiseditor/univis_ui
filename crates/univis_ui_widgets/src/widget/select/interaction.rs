use bevy::prelude::*;
use univis_ui_interaction::interaction::feedback::UInteraction;

use super::runtime::{log_select_runtime_error, spawn_dropdown};
use super::{
    ActiveSelect, SelectOptionRow, SelectRuntime, SelectTrigger, USelect, first_enabled_index,
    is_enabled_index, next_enabled_index, sanitize_select,
};

pub(super) fn enforce_select_invariants(
    mut query: Query<(Entity, &mut USelect)>,
    mut active: ResMut<ActiveSelect>,
) {
    let mut first_open = None;
    let mut active_is_valid = false;

    for (entity, mut select) in query.iter_mut() {
        sanitize_select(&mut select);

        if select.disabled {
            select.is_open = false;
        }

        if select.is_open {
            if first_open.is_none() {
                first_open = Some(entity);
            }
            if active.entity == Some(entity) {
                active_is_valid = true;
            }
        }
    }

    if active.entity.is_none() {
        active.entity = first_open;
    } else if !active_is_valid {
        active.entity = first_open;
    }
}

pub(super) fn handle_select_trigger_interaction(
    mut select_query: Query<&mut USelect>,
    trigger_query: Query<(&SelectTrigger, &UInteraction)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveSelect>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let mut pressed_select = None;
    for (marker, interaction) in trigger_query.iter() {
        if *interaction == UInteraction::Pressed {
            pressed_select = Some(marker.select);
            break;
        }
    }

    let Some(select_entity) = pressed_select else {
        return;
    };

    let should_open = {
        let Ok(mut select) = select_query.get_mut(select_entity) else {
            return;
        };

        if select.disabled {
            return;
        }

        let should_open = !select.is_open;
        select.is_open = should_open;

        if should_open && !is_enabled_index(&select.options, select.highlighted_index) {
            select.highlighted_index = select
                .selected_index
                .filter(|idx| is_enabled_index(&select.options, Some(*idx)))
                .or_else(|| first_enabled_index(&select.options));
        }

        should_open
    };

    if should_open {
        if let Some(other_entity) = active.entity
            && other_entity != select_entity
            && let Ok(mut other_select) = select_query.get_mut(other_entity)
        {
            other_select.is_open = false;
        }
        active.entity = Some(select_entity);
    } else if active.entity == Some(select_entity) {
        active.entity = None;
    }
}

pub(super) fn handle_select_option_interaction(
    mut select_query: Query<&mut USelect>,
    option_query: Query<(&SelectOptionRow, &UInteraction)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveSelect>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let mut pressed_row = None;
    for (row, interaction) in option_query.iter() {
        if *interaction == UInteraction::Pressed {
            pressed_row = Some(*row);
            break;
        }
    }

    let Some(row) = pressed_row else {
        return;
    };

    let Ok(mut select) = select_query.get_mut(row.select) else {
        return;
    };

    if !select.is_open || select.disabled {
        return;
    }

    let Some(option) = select.options.get(row.index) else {
        return;
    };

    if option.disabled {
        return;
    }

    select.selected_index = Some(row.index);
    select.highlighted_index = Some(row.index);
    select.is_open = false;

    if active.entity == Some(row.select) {
        active.entity = None;
    }
}

pub(super) fn handle_select_keyboard(
    mut select_query: Query<&mut USelect>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActiveSelect>,
) {
    let has_input = keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape);

    if !has_input {
        return;
    }

    let Some(active_entity) = active.entity else {
        return;
    };

    let Ok(mut select) = select_query.get_mut(active_entity) else {
        active.entity = None;
        return;
    };

    if !select.is_open || select.disabled {
        active.entity = None;
        return;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        select.is_open = false;
        active.entity = None;
        return;
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        select.highlighted_index = next_enabled_index(&select.options, select.highlighted_index, 1);
    }

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        select.highlighted_index =
            next_enabled_index(&select.options, select.highlighted_index, -1);
    }

    if keyboard.just_pressed(KeyCode::Enter)
        && let Some(highlighted) = select.highlighted_index
        && is_enabled_index(&select.options, Some(highlighted))
    {
        select.selected_index = Some(highlighted);
        select.is_open = false;
        active.entity = None;
    }
}

pub(super) fn close_select_on_outside_click(
    mut select_query: Query<&mut USelect>,
    trigger_query: Query<&UInteraction, With<SelectTrigger>>,
    option_query: Query<&UInteraction, With<SelectOptionRow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveSelect>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let clicked_on_select = trigger_query
        .iter()
        .any(|state| *state == UInteraction::Pressed)
        || option_query
            .iter()
            .any(|state| *state == UInteraction::Pressed);

    if clicked_on_select {
        return;
    }

    let mut closed_any = false;
    for mut select in select_query.iter_mut() {
        if select.is_open {
            select.is_open = false;
            closed_any = true;
        }
    }

    if closed_any {
        active.entity = None;
    }
}

pub(super) fn sync_select_dropdown_tree(
    mut commands: Commands,
    mut query: Query<(Entity, &mut USelect, &mut SelectRuntime)>,
    children_query: Query<(&Children, &super::SelectDropdown)>,
    option_row_query: Query<&SelectOptionRow>,
) {
    for (entity, mut select, mut runtime) in query.iter_mut() {
        if !select.is_open {
            if let Some(dropdown_entity) = runtime.dropdown_entity.take() {
                commands.entity(dropdown_entity).despawn();
            }
            continue;
        }

        let mut needs_rebuild = runtime.dropdown_entity.is_none();
        if let Some(dropdown_entity) = runtime.dropdown_entity {
            if let Ok((children, marker)) = children_query.get(dropdown_entity) {
                if marker.select != entity {
                    needs_rebuild = true;
                }
                let row_count = children
                    .iter()
                    .filter(|child| option_row_query.get(*child).is_ok())
                    .count();
                if row_count != select.options.len() {
                    needs_rebuild = true;
                }
            } else {
                needs_rebuild = true;
            }
        }

        if needs_rebuild {
            if let Some(dropdown_entity) = runtime.dropdown_entity.take() {
                commands.entity(dropdown_entity).despawn();
            }
            match spawn_dropdown(&mut commands, entity, &select) {
                Ok(dropdown_entity) => {
                    runtime.dropdown_entity = Some(dropdown_entity);
                }
                Err(error) => {
                    log_select_runtime_error(entity, error);
                    select.is_open = false;
                    runtime.dropdown_entity = None;
                }
            }
        }
    }
}
