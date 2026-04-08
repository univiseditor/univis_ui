use super::*;

use crate::widget::radio::model::URadioButton;
use crate::widget::text_label::UTextLabel;

#[derive(Component)]
pub(super) struct RadioRing;

#[derive(Component)]
pub(super) struct RadioDot {
    pub(super) base_size: f32,
}

pub(super) fn init_radio_visuals(
    mut commands: Commands,
    query: Query<(Entity, &URadioButton), Added<URadioButton>>,
) {
    for (entity, radio) in query.iter() {
        let ring_color = if radio.checked {
            radio.ring_checked_color
        } else {
            radio.ring_color
        };

        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(radio.size),
                height: UVal::Px(radio.size),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ));

        commands.entity(entity).observe(on_radio_press);

        commands.entity(entity).with_children(|parent| {
            parent
                .spawn((
                    UNode {
                        width: UVal::Px(radio.size),
                        height: UVal::Px(radio.size),
                        background_color: Color::NONE,
                        border_radius: UCornerRadius::all(radio.size / 2.0),
                        ..default()
                    },
                    UBorder {
                        color: ring_color,
                        width: radio.ring_width,
                        radius: UCornerRadius::all(radio.size / 2.0),
                        offset: 0.0,
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        align_items: UAlignItems::Center,
                        justify_content: UJustifyContent::Center,
                        ..default()
                    },
                    RadioRing,
                ))
                .with_children(|ring_parent| {
                    let dot_size = radio.size * 0.5;
                    let dot_visible = radio.checked && radio.current_scale > 0.01;

                    ring_parent.spawn((
                        UNode {
                            width: UVal::Px(dot_size),
                            height: UVal::Px(dot_size),
                            background_color: radio.dot_color,
                            border_radius: UCornerRadius::all(dot_size / 2.0),
                            ..default()
                        },
                        Transform::from_scale(Vec3::splat(radio.current_scale.clamp(0.0, 1.0))),
                        if dot_visible {
                            Visibility::Inherited
                        } else {
                            Visibility::Hidden
                        },
                        RadioDot {
                            base_size: dot_size,
                        },
                    ));
                });
        });
    }
}

pub(super) fn update_radio_visuals(
    radio_query: Query<(&URadioButton, &Children), Changed<URadioButton>>,
    ring_query: Query<&Children, With<RadioRing>>,
    mut ring_visual_query: Query<(&mut UNode, &mut UBorder), (With<RadioRing>, Without<RadioDot>)>,
    mut dot_query: Query<
        (&mut UNode, &mut Transform, &mut Visibility, &mut RadioDot),
        (With<RadioDot>, Without<RadioRing>),
    >,
) {
    for (radio, children) in radio_query.iter() {
        for child in children.iter() {
            if let Ok((mut ring_node, mut border)) = ring_visual_query.get_mut(child) {
                ring_node.width = UVal::Px(radio.size);
                ring_node.height = UVal::Px(radio.size);
                ring_node.border_radius = UCornerRadius::all(radio.size / 2.0);
                border.color = if radio.checked {
                    radio.ring_checked_color
                } else {
                    radio.ring_color
                };
                border.width = radio.ring_width.max(0.5);
                border.radius = UCornerRadius::all(radio.size / 2.0);
            }

            if let Ok(ring_children) = ring_query.get(child) {
                for dot_entity in ring_children.iter() {
                    if let Ok((mut dot_node, mut dot_transform, mut dot_visibility, mut dot_meta)) =
                        dot_query.get_mut(dot_entity)
                    {
                        dot_meta.base_size = radio.size * 0.5;

                        dot_node.width = UVal::Px(dot_meta.base_size);
                        dot_node.height = UVal::Px(dot_meta.base_size);
                        dot_node.background_color = radio.dot_color;
                        dot_node.border_radius = UCornerRadius::all(dot_meta.base_size / 2.0);

                        let scale = radio.current_scale.clamp(0.0, 1.0);
                        dot_transform.scale = Vec3::splat(scale);
                        *dot_visibility = if radio.checked && scale > 0.01 {
                            Visibility::Inherited
                        } else {
                            Visibility::Hidden
                        };
                    }
                }
            }
        }
    }
}

/// Creates a radio button row with a text label.
pub fn create_radio_with_label(
    parent: &mut ChildSpawnerCommands,
    radio: URadioButton,
    label: &str,
) -> Entity {
    parent
        .spawn((
            UNode {
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
        ))
        .with_children(|row| {
            row.spawn(radio);
            row.spawn(UTextLabel {
                text: label.to_string(),
                font_size: 16.0,
                color: Color::srgb(0.9, 0.9, 0.95),
                ..default()
            });
        })
        .id()
}
