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
                flex_direction: UFlexDirection::Row,
                gap: 48.0,
                ..default()
            },
        ))
        .id();

    // Standard Vertical Group
    let col1 = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(col1),
        UTextLabel {
            text: "Vertical Group".into(),
            font_size: 18.0,
            ..default()
        },
    ));
    let group1 = commands
        .spawn((ChildOf(col1), URadioGroup::new().with_default("apple")))
        .id();
    commands
        .spawn((ChildOf(group1), URadioButton::new("apple")))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Apple"));
        });
    commands
        .spawn((ChildOf(group1), URadioButton::new("banana")))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Banana"));
        });
    commands
        .spawn((ChildOf(group1), URadioButton::new("cherry")))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Cherry"));
        });

    // Stylized Group (Danger/Success styles)
    let col2 = commands
        .spawn((
            ChildOf(panel),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(col2),
        UTextLabel {
            text: "Styled Buttons".into(),
            font_size: 18.0,
            ..default()
        },
    ));
    let group2 = commands
        .spawn((ChildOf(col2), URadioGroup::new().allow_deselect()))
        .id();
    commands
        .spawn((ChildOf(group2), URadioButton::primary_style("opt1")))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Primary Option"));
        });
    commands
        .spawn((ChildOf(group2), URadioButton::success_style("opt2")))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Success Option"));
        });
    commands
        .spawn((ChildOf(group2), URadioButton::danger_style("opt3")))
        .with_children(|b| {
            b.spawn(UTextLabel::new("Danger Option"));
        });
}
