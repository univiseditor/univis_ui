use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.06, 0.08, 0.1),
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
                    UNode {
                        width: UVal::Px(420.0),
                        height: UVal::Px(280.0),
                        background_color: Color::srgb(0.12, 0.14, 0.18),
                        border_radius: UCornerRadius::all(14.0),
                        padding: USides::all(12.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 10.0,
                        ..default()
                    },
                ))
                .with_children(|card| {
                    card.spawn(UTextLabel {
                        text: "Layout Case: Stack".to_string(),
                        font_size: 26.0,
                        color: Color::WHITE,
                        ..default()
                    });

                    card
                        .spawn((
                            UNode {
                                width: UVal::Percent(1.0),
                                height: UVal::Flex(1.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Stack,
                                align_items: UAlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|stack| {
                            stack.spawn(UNode {
                                width: UVal::Px(180.0),
                                height: UVal::Px(120.0),
                                background_color: Color::srgb(0.28, 0.6, 0.95),
                                border_radius: UCornerRadius::all(12.0),
                                ..default()
                            });

                            stack.spawn(UNode {
                                width: UVal::Px(140.0),
                                height: UVal::Px(96.0),
                                background_color: Color::srgb(0.95, 0.66, 0.26).with_alpha(0.92),
                                border_radius: UCornerRadius::all(12.0),
                                ..default()
                            });

                            stack.spawn(UNode {
                                width: UVal::Px(100.0),
                                height: UVal::Px(70.0),
                                background_color: Color::srgb(0.26, 0.86, 0.54).with_alpha(0.92),
                                border_radius: UCornerRadius::all(12.0),
                                ..default()
                            });
                        });
                });
        });
}
