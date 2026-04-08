use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, UVal};
use univis_ui_engine::layout::univis_node::{
    UAlignItems, UBorder, UClip, UDisplay, UFlexDirection, UJustifyContent, ULayout, UNode,
};
use univis_ui_interaction::interaction::feedback::{UInteraction, UInteractionColors};
use univis_ui_style::style::{Theme, icons::Icon};

use super::{USelect, selected_option};
use crate::widget::text_label::UTextLabel;

#[derive(Component)]
pub(super) struct SelectRuntime {
    pub(super) trigger_entity: Entity,
    pub(super) value_label_entity: Entity,
    pub(super) chevron_entity: Entity,
    pub(super) dropdown_entity: Option<Entity>,
}

#[derive(Component, Clone, Copy)]
pub(super) struct SelectTrigger {
    pub(super) select: Entity,
}

#[derive(Component)]
pub(super) struct SelectValueLabel;

#[derive(Component)]
pub(super) struct SelectChevronLabel;

#[derive(Component, Clone, Copy)]
pub(super) struct SelectDropdown {
    pub(super) select: Entity,
}

#[derive(Component, Clone, Copy)]
pub(super) struct SelectOptionRow {
    pub(super) select: Entity,
    pub(super) index: usize,
}

#[derive(Component)]
pub(super) struct SelectOptionLabel;

#[derive(Resource, Default)]
pub(super) struct ActiveSelect {
    pub(super) entity: Option<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SelectRuntimeError {
    MissingTrigger,
    MissingValueLabel,
    MissingChevron,
    DropdownSpawnFailed,
}

impl SelectRuntimeError {
    fn reason(self) -> &'static str {
        match self {
            Self::MissingTrigger => "select trigger child was not spawned",
            Self::MissingValueLabel => "select value-label child was not spawned",
            Self::MissingChevron => "select chevron child was not spawned",
            Self::DropdownSpawnFailed => "select dropdown child was not spawned",
        }
    }

    fn action(self) -> &'static str {
        match self {
            Self::MissingTrigger | Self::MissingValueLabel | Self::MissingChevron => {
                "verify the select build system still spawns the required child tree"
            }
            Self::DropdownSpawnFailed => {
                "verify the select dropdown builder still spawns the dropdown container"
            }
        }
    }
}

pub(super) fn log_select_runtime_error(entity: Entity, error: SelectRuntimeError) {
    bevy::log::warn!(
        "[widget/select_runtime] entity={:?} reason={} action={}",
        entity,
        error.reason(),
        error.action(),
    );
}

pub(super) fn init_select_visuals(
    mut commands: Commands,
    theme: Res<Theme>,
    mut query: Query<(Entity, &mut USelect), Added<USelect>>,
) {
    for (entity, mut select) in query.iter_mut() {
        super::sanitize_select(&mut select);

        let mut trigger_entity = None;
        let mut value_label_entity = None;
        let mut chevron_entity = None;

        commands
            .entity(entity)
            .insert((
                UNode {
                    width: UVal::Px(select.width),
                    height: UVal::Content,
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                    ..default()
                },
            ))
            .with_children(|parent| {
                let trigger = parent
                    .spawn((
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(select.trigger_height),
                            padding: select.padding,
                            background_color: select.background,
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        UBorder {
                            color: select.border_color,
                            width: 1.0,
                            radius: UCornerRadius::all(8.0),
                            offset: 0.0,
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::SpaceBetween,
                            align_items: UAlignItems::Center,
                            ..default()
                        },
                        Pickable::default(),
                        UInteraction::default(),
                        UInteractionColors {
                            normal: select.background,
                            hovered: select.hover_color,
                            pressed: select.pressed_color,
                        },
                        SelectTrigger { select: entity },
                    ))
                    .with_children(|trigger_parent| {
                        let text = selected_option(&select)
                            .map(|opt| opt.label.clone())
                            .unwrap_or_else(|| select.placeholder.clone());
                        let color = if select.selected_index.is_some() {
                            select.text_color
                        } else {
                            select.placeholder_color
                        };

                        let value_label = trigger_parent
                            .spawn((
                                UTextLabel {
                                    text,
                                    font_size: select.font_size,
                                    color,
                                    autosize: false,
                                    ..default()
                                },
                                SelectValueLabel,
                                Pickable::IGNORE,
                            ))
                            .id();

                        let chevron = trigger_parent
                            .spawn((
                                UTextLabel {
                                    text: Icon::CHEVRON_DOWN.to_string(),
                                    font_size: select.font_size,
                                    color: select.text_color,
                                    font: theme.icon.font.clone(),
                                    autosize: true,
                                    ..default()
                                },
                                SelectChevronLabel,
                                Pickable::IGNORE,
                            ))
                            .id();

                        value_label_entity = Some(value_label);
                        chevron_entity = Some(chevron);
                    })
                    .id();

                trigger_entity = Some(trigger);
            });

        match build_select_runtime(trigger_entity, value_label_entity, chevron_entity) {
            Ok(runtime) => {
                commands.entity(entity).insert(runtime);
            }
            Err(error) => {
                log_select_runtime_error(entity, error);
                select.is_open = false;
                select.previous_open = false;
            }
        }
    }
}

pub(super) fn spawn_dropdown(
    commands: &mut Commands,
    select_entity: Entity,
    select: &USelect,
) -> Result<Entity, SelectRuntimeError> {
    let mut dropdown_entity = None;
    let max_visible = select.max_visible_options.max(1);
    let should_clip = select.options.len() > max_visible;
    let row_height = select.trigger_height.max(1.0);

    commands.entity(select_entity).with_children(|parent| {
        let dropdown = parent
            .spawn((
                UNode {
                    width: UVal::Px(select.width.max(1.0)),
                    height: if should_clip {
                        UVal::Px(row_height * max_visible as f32)
                    } else {
                        UVal::Content
                    },
                    background_color: select.dropdown_background,
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                UBorder {
                    color: select.border_color,
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                    offset: 0.0,
                },
                UClip {
                    enabled: should_clip,
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    ..default()
                },
                SelectDropdown {
                    select: select_entity,
                },
            ))
            .with_children(|list| {
                for (index, option) in select.options.iter().enumerate() {
                    let is_selected = select.selected_index == Some(index);
                    let is_highlighted = select.highlighted_index == Some(index);

                    let base_bg = if option.disabled {
                        Color::NONE
                    } else if is_selected {
                        select.option_selected_color
                    } else if is_highlighted {
                        select.option_hover_color
                    } else {
                        Color::NONE
                    };

                    list.spawn((
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(row_height),
                            padding: select.padding,
                            background_color: base_bg,
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            justify_content: UJustifyContent::Start,
                            ..default()
                        },
                        Pickable::default(),
                        UInteraction::default(),
                        UInteractionColors {
                            normal: Color::NONE,
                            hovered: if option.disabled {
                                Color::NONE
                            } else {
                                select.option_hover_color
                            },
                            pressed: if option.disabled {
                                Color::NONE
                            } else {
                                select.option_hover_color
                            },
                        },
                        SelectOptionRow {
                            select: select_entity,
                            index,
                        },
                    ))
                    .with_children(|row| {
                        row.spawn((
                            UTextLabel {
                                text: option.label.clone(),
                                font_size: select.font_size,
                                color: if option.disabled {
                                    select.option_disabled_text_color
                                } else {
                                    select.text_color
                                },
                                autosize: false,
                                ..default()
                            },
                            SelectOptionLabel,
                            Pickable::IGNORE,
                        ));
                    });
                }
            })
            .id();

        dropdown_entity = Some(dropdown);
    });

    dropdown_entity.ok_or(SelectRuntimeError::DropdownSpawnFailed)
}

fn build_select_runtime(
    trigger_entity: Option<Entity>,
    value_label_entity: Option<Entity>,
    chevron_entity: Option<Entity>,
) -> Result<SelectRuntime, SelectRuntimeError> {
    Ok(SelectRuntime {
        trigger_entity: trigger_entity.ok_or(SelectRuntimeError::MissingTrigger)?,
        value_label_entity: value_label_entity.ok_or(SelectRuntimeError::MissingValueLabel)?,
        chevron_entity: chevron_entity.ok_or(SelectRuntimeError::MissingChevron)?,
        dropdown_entity: None,
    })
}
