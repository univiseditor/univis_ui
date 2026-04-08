use super::*;

use crate::widget::radio::model::{URadioButton, URadioGroup};

fn collect_descendants_recursive(
    children: &Children,
    children_query: &Query<&Children>,
    out: &mut Vec<Entity>,
) {
    for child in children.iter() {
        out.push(child);
        if let Ok(grand_children) = children_query.get(child) {
            collect_descendants_recursive(grand_children, children_query, out);
        }
    }
}

pub(super) fn sync_radio_groups(
    mut commands: Commands,
    mut group_query: Query<(Entity, &mut URadioGroup, &Children)>,
    children_query: Query<&Children>,
    mut radio_query: Query<&mut URadioButton>,
) {
    for (group_entity, mut group, group_children) in group_query.iter_mut() {
        commands.entity(group_entity).insert((
            UNode {
                width: UVal::Content,
                height: UVal::Content,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: group.direction,
                gap: group.gap,
                ..default()
            },
        ));

        let old_buttons = group.buttons.clone();

        let mut descendants = Vec::new();
        collect_descendants_recursive(group_children, &children_query, &mut descendants);

        let mut new_buttons = Vec::new();
        let mut first_enabled_value: Option<String> = None;
        let mut selected_exists = false;
        let selected_value = group.selected_value.clone();

        for entity in descendants {
            if let Ok(mut radio) = radio_query.get_mut(entity) {
                radio.group = Some(group_entity);
                new_buttons.push(entity);

                if !radio.disabled && first_enabled_value.is_none() {
                    first_enabled_value = Some(radio.value.clone());
                }

                if let Some(selected) = selected_value.as_ref()
                    && !radio.disabled
                    && radio.value == *selected
                {
                    selected_exists = true;
                }
            }
        }

        if group.require_selection {
            if !selected_exists {
                group.selected_value = first_enabled_value;
            }
        } else if !selected_exists {
            group.selected_value = None;
        }

        let final_selected = group.selected_value.clone();
        for &button_entity in &new_buttons {
            if let Ok(mut radio) = radio_query.get_mut(button_entity) {
                let is_selected = final_selected
                    .as_ref()
                    .map(|value| *value == radio.value)
                    .unwrap_or(false);
                radio.checked = is_selected;
                radio.current_scale = if is_selected { 1.0 } else { 0.0 };
            }
        }

        for old_button in old_buttons {
            if !new_buttons.contains(&old_button)
                && let Ok(mut radio) = radio_query.get_mut(old_button)
                && radio.group == Some(group_entity)
            {
                radio.group = None;
                radio.checked = false;
                radio.current_scale = 0.0;
            }
        }

        group.buttons = new_buttons;
        group.previous_value = group.selected_value.clone();
    }
}

pub(super) fn on_radio_press(
    trigger: On<Pointer<Press>>,
    mut radio_query: Query<&mut URadioButton>,
    mut group_query: Query<&mut URadioGroup>,
) {
    let radio_entity = trigger.entity.entity();

    let Ok(mut radio) = radio_query.get_mut(radio_entity) else {
        return;
    };

    if radio.disabled {
        return;
    }

    if let Some(group_entity) = radio.group {
        if let Ok(mut group) = group_query.get_mut(group_entity) {
            if radio.checked && !group.require_selection {
                radio.checked = false;
                radio.current_scale = 0.0;
                group.selected_value = None;
            } else {
                radio.checked = true;
                group.selected_value = Some(radio.value.clone());

                let buttons_to_deselect: Vec<Entity> = group
                    .buttons
                    .iter()
                    .filter(|&&entity| entity != radio_entity)
                    .copied()
                    .collect();

                for button_entity in buttons_to_deselect {
                    if let Ok(mut other_radio) = radio_query.get_mut(button_entity) {
                        other_radio.checked = false;
                        other_radio.current_scale = 0.0;
                    }
                }
            }
        }
    } else {
        radio.checked = !radio.checked;
    }
}

pub(super) fn animate_radio_check(time: Res<Time>, mut query: Query<&mut URadioButton>) {
    for mut radio in query.iter_mut() {
        let target_scale = if radio.checked { 1.0 } else { 0.0 };
        let diff = target_scale - radio.current_scale;

        if diff.abs() < 0.01 {
            radio.current_scale = target_scale;
            continue;
        }

        let delta = time.delta_secs() * radio.animation_speed;
        radio.current_scale += diff * delta;
    }
}
