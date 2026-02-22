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
                background_color: Color::srgb(0.06, 0.09, 0.12),
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
                        width: UVal::Px(520.0),
                        height: UVal::Px(420.0),
                        background_color: Color::srgb(0.1, 0.13, 0.17),
                        border_radius: UCornerRadius::all(14.0),
                        padding: USides::all(14.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 10.0,
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn(UTextLabel {
                        text: "Layout Case: Radial".to_string(),
                        font_size: 28.0,
                        color: Color::WHITE,
                        ..default()
                    });

                    panel
                        .spawn((
                            UNode {
                                width: UVal::Percent(1.0),
                                height: UVal::Flex(1.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Radial,
                                ..default()
                            },
                        ))
                        .with_children(|radial| {
                            for i in 0..10 {
                                radial.spawn(UNode {
                                    width: UVal::Px(30.0 + (i % 2) as f32 * 16.0),
                                    height: UVal::Px(30.0 + (i % 3) as f32 * 10.0),
                                    background_color: Color::srgb(0.22 + i as f32 * 0.07, 0.8 - i as f32 * 0.05, 0.9),
                                    border_radius: UCornerRadius::all(10.0),
                                    ..default()
                                });
                            }
                        });
                });
        });
}
