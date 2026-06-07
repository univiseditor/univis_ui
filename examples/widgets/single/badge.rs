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
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    // Standard sizes
    let row1 = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 16.0,
                ..default()
            },
        ))
        .id();
    commands
        .spawn((ChildOf(row1), UBadge::primary()))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Primary"));
        });
    commands
        .spawn((ChildOf(row1), UBadge::info()))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Info"));
        });
    commands
        .spawn((ChildOf(row1), UBadge::danger()))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Danger"));
        });
    commands
        .spawn((ChildOf(row1), UBadge::success()))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Success"));
        });
    commands
        .spawn((ChildOf(row1), UBadge::warning()))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Warning"));
        });

    // Different sizes
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
    commands
        .spawn((ChildOf(row2), UBadge::success().small()))
        .with_children(|b| {
            b.spawn(UTextLabel {
                text: "Small Badge".into(),
                font_size: 12.0,
                ..default()
            });
        });
    commands
        .spawn((ChildOf(row2), UBadge::primary()))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Normal Badge"));
        });
    commands
        .spawn((ChildOf(row2), UBadge::danger().large()))
        .with_children(|b| {
            b.spawn(UTextLabel {
                text: "Large Badge".into(),
                font_size: 20.0,
                ..default()
            });
        });
}
