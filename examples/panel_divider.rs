use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    let font = theme.text.font.inter_regular.clone();
    commands.spawn(Camera2d);

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.07, 0.09, 0.12),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root
                .spawn((
                    UPanel::glass().with_gap(14.0),
                    UNode {
                        width: UVal::Px(560.0),
                        height: UVal::Px(300.0),
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn(UTextLabel {
                        text: "Panel + Divider Widget Demo".to_string(),
                        font_size: 28.0,
                        color: Color::WHITE,
                        font: font.clone(),
                        ..default()
                    });

                    panel.spawn(UDivider::horizontal().with_thickness(2.0));

                    panel.spawn(UTextLabel {
                        text: "First section content".to_string(),
                        font_size: 18.0,
                        color: Color::srgb(0.86, 0.9, 0.98),
                        font: font.clone(),
                        ..default()
                    });

                    panel.spawn(UDivider::horizontal().with_color(Color::srgb(0.2, 0.65, 1.0)));

                    panel
                        .spawn((
                            UNode {
                                width: UVal::Percent(1.0),
                                height: UVal::Px(120.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                justify_content: UJustifyContent::SpaceEvenly,
                                align_items: UAlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|row| {
                            row.spawn(UTextLabel {
                                text: "Left".to_string(),
                                font_size: 18.0,
                                color: Color::WHITE,
                                font: font.clone(),
                                ..default()
                            });

                            row.spawn(UDivider::vertical().with_length(UVal::Percent(1.0)).with_thickness(2.0));

                            row.spawn(UTextLabel {
                                text: "Right".to_string(),
                                font_size: 18.0,
                                color: Color::WHITE,
                                font: font.clone(),
                                ..default()
                            });
                        });
                });
        });
}
