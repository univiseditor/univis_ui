use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, UVal};
use univis_ui_engine::layout::univis_node::{UBorder, UClip, UNode};
use univis_ui_interaction::interaction::feedback::{UInteraction, UInteractionColors};
use univis_ui_style::style::icons::Icon;

use super::{
    SelectChevronLabel, SelectDropdown, SelectOptionLabel, SelectOptionRow, SelectRuntime,
    SelectTrigger, SelectValueLabel, USelect, selected_option,
};
use crate::widget::text_label::UTextLabel;

pub(super) fn update_select_visuals(
    mut root_query: Query<
        (Entity, &USelect, &SelectRuntime, &mut UNode),
        (
            With<USelect>,
            Without<SelectTrigger>,
            Without<SelectDropdown>,
            Without<SelectOptionRow>,
        ),
    >,
    mut trigger_query: Query<
        (
            &UInteraction,
            &mut UNode,
            &mut UBorder,
            &mut UInteractionColors,
        ),
        (
            With<SelectTrigger>,
            Without<SelectOptionRow>,
            Without<SelectDropdown>,
        ),
    >,
    mut value_label_query: Query<
        &mut UTextLabel,
        (
            With<SelectValueLabel>,
            Without<SelectChevronLabel>,
            Without<SelectOptionLabel>,
        ),
    >,
    mut chevron_label_query: Query<
        &mut UTextLabel,
        (
            With<SelectChevronLabel>,
            Without<SelectValueLabel>,
            Without<SelectOptionLabel>,
        ),
    >,
    mut dropdown_query: Query<
        (
            &SelectDropdown,
            &mut UNode,
            &mut UBorder,
            &mut UClip,
            &Children,
        ),
        (
            With<SelectDropdown>,
            Without<SelectTrigger>,
            Without<SelectOptionRow>,
        ),
    >,
    mut row_query: Query<
        (
            &SelectOptionRow,
            &UInteraction,
            &mut UNode,
            &mut UInteractionColors,
            &Children,
        ),
        (
            With<SelectOptionRow>,
            Without<SelectTrigger>,
            Without<SelectDropdown>,
        ),
    >,
    mut option_label_query: Query<
        &mut UTextLabel,
        (
            With<SelectOptionLabel>,
            Without<SelectValueLabel>,
            Without<SelectChevronLabel>,
        ),
    >,
) {
    for (entity, select, runtime, mut root_node) in root_query.iter_mut() {
        // Write-only-on-change — unconditional assignment marks the UNode
        // changed every frame and keeps the incremental renderer redrawing
        // the whole panel every frame.
        let width_px = UVal::Px(select.width.max(1.0));
        if root_node.width != width_px {
            root_node.width = width_px;
        }
        if root_node.height != UVal::Content {
            root_node.height = UVal::Content;
        }
        if root_node.background_color != Color::NONE {
            root_node.background_color = Color::NONE;
        }

        if let Ok((interaction, mut trigger_node, mut trigger_border, mut trigger_colors)) =
            trigger_query.get_mut(runtime.trigger_entity)
        {
            trigger_node.width = UVal::Percent(1.0);
            trigger_node.height = UVal::Px(select.trigger_height.max(1.0));
            trigger_node.padding = select.padding;
            trigger_node.border_radius = UCornerRadius::all(8.0);

            trigger_colors.normal = select.background;
            trigger_colors.hovered = if select.disabled {
                select.background
            } else {
                select.hover_color
            };
            trigger_colors.pressed = if select.disabled {
                select.background
            } else {
                select.pressed_color
            };

            trigger_border.color = select.border_color;
            trigger_border.width = 1.0;
            trigger_border.radius = UCornerRadius::all(8.0);

            trigger_node.background_color = if select.disabled {
                select.background
            } else {
                match *interaction {
                    UInteraction::Pressed | UInteraction::Clicked => select.pressed_color,
                    UInteraction::Hovered | UInteraction::Released => select.hover_color,
                    UInteraction::Normal => select.background,
                }
            };
        }

        if let Ok(mut label) = value_label_query.get_mut(runtime.value_label_entity) {
            if let Some(option) = selected_option(select) {
                label.text = option.label.clone();
                label.color = select.text_color;
            } else {
                label.text = select.placeholder.clone();
                label.color = select.placeholder_color;
            }
            label.font_size = select.font_size;
        }

        if let Ok(mut chevron) = chevron_label_query.get_mut(runtime.chevron_entity) {
            chevron.text = if select.is_open {
                Icon::CHEVRONS_UP.to_string()
            } else {
                Icon::CHEVRON_DOWN.to_string()
            };
            chevron.font_size = select.font_size;
            chevron.color = select.text_color;
        }

        if let Some(dropdown_entity) = runtime.dropdown_entity
            && let Ok((
                dropdown,
                mut dropdown_node,
                mut dropdown_border,
                mut clip,
                dropdown_children,
            )) = dropdown_query.get_mut(dropdown_entity)
        {
            if dropdown.select != entity {
                continue;
            }

            let max_visible = select.max_visible_options.max(1);
            let should_clip = select.options.len() > max_visible;
            dropdown_node.width = UVal::Px(select.width.max(1.0));
            dropdown_node.height = if should_clip {
                UVal::Px(select.trigger_height.max(1.0) * max_visible as f32)
            } else {
                UVal::Content
            };
            dropdown_node.background_color = select.dropdown_background;
            dropdown_node.border_radius = UCornerRadius::all(8.0);
            dropdown_border.color = select.border_color;
            dropdown_border.width = 1.0;
            dropdown_border.radius = UCornerRadius::all(8.0);
            clip.enabled = should_clip;

            for child in dropdown_children.iter() {
                if let Ok((row, interaction, mut row_node, mut row_colors, row_children)) =
                    row_query.get_mut(child)
                {
                    let Some(option) = select.options.get(row.index) else {
                        continue;
                    };

                    let is_selected = select.selected_index == Some(row.index);
                    let is_highlighted = select.highlighted_index == Some(row.index);

                    row_node.width = UVal::Percent(1.0);
                    row_node.height = UVal::Px(select.trigger_height.max(1.0));
                    row_node.padding = select.padding;

                    row_colors.normal = Color::NONE;
                    row_colors.hovered = if option.disabled {
                        Color::NONE
                    } else {
                        select.option_hover_color
                    };
                    row_colors.pressed = if option.disabled {
                        Color::NONE
                    } else {
                        select.option_hover_color
                    };

                    row_node.background_color = if option.disabled {
                        Color::NONE
                    } else if is_selected {
                        select.option_selected_color
                    } else if is_highlighted {
                        select.option_hover_color
                    } else {
                        match *interaction {
                            UInteraction::Hovered
                            | UInteraction::Pressed
                            | UInteraction::Clicked
                            | UInteraction::Released => select.option_hover_color,
                            UInteraction::Normal => Color::NONE,
                        }
                    };

                    for row_child in row_children.iter() {
                        if let Ok(mut label) = option_label_query.get_mut(row_child) {
                            label.text = option.label.clone();
                            label.font_size = select.font_size;
                            label.color = if option.disabled {
                                select.option_disabled_text_color
                            } else {
                                select.text_color
                            };
                        }
                    }
                }
            }
        }
    }
}
