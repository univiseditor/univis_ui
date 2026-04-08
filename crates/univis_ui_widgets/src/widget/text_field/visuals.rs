use super::*;

use crate::widget::text_field::model::{TextFieldInputType, UTextField};
use crate::widget::text_field::runtime::on_textfield_click;
use crate::widget::text_label::UTextLabel;
use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
use univis_ui_engine::layout::univis_node::{
    UAlignItems, UBorder, UDisplay, UFlexDirection, UJustifyContent, ULayout, UNode,
};
use univis_ui_interaction::interaction::feedback::UInteraction;

#[derive(Component)]
pub(super) struct TextFieldTextLabel;

#[derive(Component)]
pub(super) struct TextFieldCursor;

pub(super) fn init_textfield_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UTextField), Added<UTextField>>,
) {
    for (entity, textfield) in query.iter() {
        let border_color = if textfield.focused {
            textfield.border_focused_color
        } else {
            textfield.border_color
        };

        let bg_color = if textfield.focused {
            textfield.background_focused_color
        } else {
            textfield.background_color
        };

        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(textfield.width),
                height: UVal::Px(textfield.height),
                background_color: bg_color,
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(textfield.padding),
                ..default()
            },
            UInteraction::default(),
            UBorder {
                color: border_color,
                width: 2.0,
                radius: UCornerRadius::all(8.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ));

        commands.entity(entity).observe(on_textfield_click);

        commands.entity(entity).with_children(|parent| {
            let display_text = if textfield.text.is_empty() {
                &textfield.placeholder
            } else if textfield.input_type == TextFieldInputType::Password {
                &"•".repeat(textfield.text.len())
            } else {
                &textfield.text
            };

            let text_color = if textfield.text.is_empty() {
                textfield.placeholder_color
            } else {
                textfield.text_color
            };

            parent.spawn((
                UTextLabel {
                    text: display_text.to_string(),
                    font_size: textfield.font_size,
                    color: text_color,
                    autosize: false,
                    ..default()
                },
                TextFieldTextLabel,
            ));

            if textfield.focused {
                spawn_cursor(parent, textfield);
            }
        });
    }
}

pub(super) fn update_textfield_visuals(
    textfield_query: Query<(Entity, &UTextField, &Children), Changed<UTextField>>,
    mut node_query: Query<&mut UNode>,
    mut border_query: Query<&mut UBorder>,
    mut text_query: Query<&mut UTextLabel, With<TextFieldTextLabel>>,
    cursor_query: Query<Entity, With<TextFieldCursor>>,
    mut commands: Commands,
) {
    for (entity, textfield, children) in textfield_query.iter() {
        if let Ok(mut node) = node_query.get_mut(entity) {
            node.background_color = if textfield.focused {
                textfield.background_focused_color
            } else {
                textfield.background_color
            };
        }

        if let Ok(mut border) = border_query.get_mut(entity) {
            border.color = if textfield.focused {
                textfield.border_focused_color
            } else {
                textfield.border_color
            };
        }

        for child in children.iter() {
            if let Ok(mut text_label) = text_query.get_mut(child) {
                let display_text = if textfield.text.is_empty() {
                    &textfield.placeholder
                } else if textfield.input_type == TextFieldInputType::Password {
                    &"•".repeat(textfield.text.len())
                } else {
                    &textfield.text
                };

                text_label.text = display_text.to_string();
                text_label.color = if textfield.text.is_empty() {
                    textfield.placeholder_color
                } else {
                    textfield.text_color
                };
            }
        }

        let has_cursor = children.iter().any(|child| cursor_query.get(child).is_ok());

        if textfield.focused && !has_cursor {
            commands.entity(entity).with_children(|parent| {
                spawn_cursor(parent, textfield);
            });
        } else if !textfield.focused && has_cursor {
            for child in children.iter() {
                if cursor_query.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

pub(super) fn animate_textfield_cursor(
    time: Res<Time>,
    mut textfield_query: Query<(&mut UTextField, &Children)>,
    mut cursor_query: Query<&mut Visibility, With<TextFieldCursor>>,
) {
    for (mut textfield, children) in textfield_query.iter_mut() {
        if !textfield.focused {
            continue;
        }

        textfield.cursor_blink_timer += time.delta_secs();

        if textfield.cursor_blink_timer >= textfield.cursor_blink_speed {
            textfield.cursor_blink_timer = 0.0;
            textfield.cursor_visible = !textfield.cursor_visible;

            for child in children.iter() {
                if let Ok(mut visibility) = cursor_query.get_mut(child) {
                    *visibility = if textfield.cursor_visible {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
            }
        }
    }
}

fn spawn_cursor(parent: &mut ChildSpawnerCommands, textfield: &UTextField) {
    parent.spawn((
        UNode {
            width: UVal::Px(2.0),
            height: UVal::Px(textfield.font_size * 1.2),
            background_color: textfield.cursor_color,
            border_radius: UCornerRadius::all(1.0),
            margin: USides::left(4.0),
            ..default()
        },
        TextFieldCursor,
    ));
}
