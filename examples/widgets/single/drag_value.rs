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
                width: UVal::Px(400.0),
                padding: USides::all(32.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(panel),
        UTextLabel::new("Drag values left/right to change them."),
    ));

    // Standard Drag Value
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
    commands.spawn((ChildOf(row1), UTextLabel::new("Default (Step 1.0): ")));
    commands.spawn((ChildOf(row1), UDragValue::new().with_value(50.0)));

    // Ranged Drag Value
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
    commands.spawn((ChildOf(row2), UTextLabel::new("Clamped (0-100):    ")));
    commands.spawn((
        ChildOf(row2),
        UDragValue::new().with_value(25.0).with_range(0.0, 100.0),
    ));

    // Fine/Precise Drag Value
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
    commands.spawn((ChildOf(row3), UTextLabel::new("Precise (Step 0.01):")));
    commands.spawn((
        ChildOf(row3),
        UDragValue::new()
            .with_value(1.23)
            .with_step(0.01)
            .with_decimals(2),
    ));

    // Disabled Drag Value
    let row4 = commands
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
    commands.spawn((ChildOf(row4), UTextLabel::new("Disabled:           ")));
    commands.spawn((ChildOf(row4), UDragValue::new().with_value(42.0).disabled()));
}
