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
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 24.0,
                ..default()
            },
        ))
        .id();

    // iOS Style
    let row1 = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.spawn((ChildOf(row1), UTextLabel::new("iOS Style:     ")));
    commands.spawn((ChildOf(row1), UToggle::ios_style().with_checked(true)));
    commands.spawn((ChildOf(row1), UToggle::ios_style().with_checked(false)));

    // Material Style
    let row2 = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.spawn((ChildOf(row2), UTextLabel::new("Material Style:")));
    commands.spawn((ChildOf(row2), UToggle::material_style().with_checked(true)));
    commands.spawn((ChildOf(row2), UToggle::material_style().with_checked(false)));

    // Sci-Fi Style
    let row3 = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.spawn((ChildOf(row3), UTextLabel::new("Sci-Fi Style:  ")));
    commands.spawn((ChildOf(row3), UToggle::sci_fi_style().with_checked(true)));
    commands.spawn((ChildOf(row3), UToggle::sci_fi_style().with_checked(false)));
}
