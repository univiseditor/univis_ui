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

    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.05, 0.07, 0.1),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let viewport = commands
        .spawn((
            ChildOf(root),
            UClip::enabled(true),
            UScrollContainer::new(),
            UInteraction::default(),
            UNode {
                width: UVal::Px(300.0),
                height: UVal::Px(400.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
        ))
        .id();

    let list = commands
        .spawn((
            ChildOf(viewport),
            USelf::default(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Content,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(list),
        UTextLabel {
            text: "Scroll View Content".to_string(),
            font_size: 24.0,
            ..default()
        },
    ));

    for index in 1..=20 {
        let card = commands
            .spawn((
                ChildOf(list),
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(60.0),
                    background_color: Color::srgba(0.1, 0.2, 0.3, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Center,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(card),
            UTextLabel {
                text: format!("List Item {}", index),
                font_size: 18.0,
                ..default()
            },
        ));
    }
}
