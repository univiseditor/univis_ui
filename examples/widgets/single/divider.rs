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

    let panel = commands
        .spawn((
            ChildOf(root),
            UPanel::glass().with_gap(16.0),
            UNode {
                width: UVal::Px(500.0),
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(panel), UTextLabel::new("Horizontal Dividers")));
    commands.spawn((ChildOf(panel), UDivider::horizontal()));
    commands.spawn((
        ChildOf(panel),
        UDivider::horizontal()
            .with_thickness(4.0)
            .with_color(Color::srgb(0.2, 0.6, 0.9)),
    ));
    commands.spawn((
        ChildOf(panel),
        UDivider::horizontal()
            .with_length(UVal::Percent(0.5))
            .with_color(Color::srgb(0.9, 0.2, 0.2)),
    ));

    commands.spawn((
        ChildOf(panel),
        UNode {
            height: UVal::Px(20.0),
            ..default()
        },
    ));

    commands.spawn((ChildOf(panel), UTextLabel::new("Vertical Dividers")));

    let row = commands
        .spawn((
            ChildOf(panel),
            UNode {
                height: UVal::Px(100.0),
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(row), UTextLabel::new("Item 1")));
    commands.spawn((ChildOf(row), UDivider::vertical()));
    commands.spawn((ChildOf(row), UTextLabel::new("Item 2")));
    commands.spawn((
        ChildOf(row),
        UDivider::vertical()
            .with_thickness(4.0)
            .with_color(Color::srgb(0.2, 0.8, 0.4)),
    ));
    commands.spawn((ChildOf(row), UTextLabel::new("Item 3")));
}
