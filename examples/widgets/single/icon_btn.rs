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
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(panel), UTextLabel::new("Standard Icon Buttons")));

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
    commands.spawn((ChildOf(row1), UIconButton::primary(Icon::PLAY)));
    commands.spawn((ChildOf(row1), UIconButton::secondary(Icon::PAUSE)));
    commands.spawn((ChildOf(row1), UIconButton::danger(Icon::X)));
    commands.spawn((ChildOf(row1), UIconButton::success(Icon::CHECK)));
    commands.spawn((ChildOf(row1), UIconButton::primary(Icon::HOUSE)));

    commands.spawn((ChildOf(panel), UDivider::horizontal()));

    commands.spawn((ChildOf(panel), UTextLabel::new("Customized Icon Buttons")));

    let row2 = commands
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
    commands.spawn((
        ChildOf(row2),
        UIconButton {
            icon: Icon::SETTINGS,
            icon_size: 32.0,
            padding: USides::all(20.0),
            background: Color::srgba(0.2, 0.2, 0.2, 0.8),
            border_radius: UCornerRadius::all(16.0),
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(row2),
        UIconButton {
            icon: Icon::HEART,
            icon_size: 24.0,
            icon_color: Color::srgb(1.0, 0.3, 0.4),
            padding: USides::axes(24.0, 16.0),
            background: Color::srgba(0.1, 0.1, 0.1, 0.9),
            border_radius: UCornerRadius::all(32.0),
            ..default()
        },
    ));
}
