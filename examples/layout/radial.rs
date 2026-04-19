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

    let shell = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(880.0),
                height: UVal::Px(720.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.14, 0.98),
                border_radius: UCornerRadius::all(28.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.72, 0.8, 0.9, 0.18),
                width: 1.0,
                radius: UCornerRadius::all(28.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(shell),
        label_node(),
        text("Radial layout", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Direct children are distributed around a circle. This is useful for menus, orbital HUDs, and sci-fi selectors.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let panel = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(832.0),
                height: UVal::Px(620.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.1, 0.13, 0.18, 0.96),
                border_radius: UCornerRadius::all(22.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.7, 0.8, 0.92, 0.12),
                width: 1.0,
                radius: UCornerRadius::all(22.0),
                offset: 0.0,
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
        ChildOf(panel),
        label_node(),
        text("Orbital selector", 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "The panel below uses `display = Radial` and varied item sizes to show the circular placement.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let radial = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(800.0),
                height: UVal::Px(530.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Radial,
                ..default()
            },
        ))
        .id();

    let labels = [
        "SCAN", "NAV", "LINK", "CORE", "SHLD", "DRON", "PING", "MAP", "LOG", "SYNC",
    ];
    let sizes = [
        (84.0, 84.0),
        (72.0, 72.0),
        (92.0, 68.0),
        (78.0, 78.0),
        (88.0, 64.0),
        (74.0, 74.0),
        (96.0, 72.0),
        (80.0, 80.0),
        (86.0, 62.0),
        (76.0, 76.0),
    ];
    let colors = [
        Color::srgba(0.2, 0.37, 0.82, 0.96),
        Color::srgba(0.13, 0.58, 0.74, 0.96),
        Color::srgba(0.17, 0.68, 0.49, 0.96),
        Color::srgba(0.36, 0.7, 0.25, 0.96),
        Color::srgba(0.9, 0.63, 0.18, 0.96),
        Color::srgba(0.84, 0.41, 0.2, 0.96),
        Color::srgba(0.77, 0.27, 0.31, 0.96),
        Color::srgba(0.52, 0.32, 0.82, 0.96),
        Color::srgba(0.16, 0.7, 0.66, 0.96),
        Color::srgba(0.3, 0.49, 0.92, 0.96),
    ];

    for ((label, (width, height)), color) in labels.into_iter().zip(sizes).zip(colors) {
        let chip = commands
            .spawn((
                ChildOf(radial),
                UNode {
                    width: UVal::Px(width),
                    height: UVal::Px(height),
                    padding: USides::all(10.0),
                    background_color: color,
                    border_radius: UCornerRadius::all(18.0),
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

        commands.spawn((ChildOf(chip), label_node(), text(label, 18.0, Color::WHITE)));
    }
}

fn label_node() -> UNode {
    UNode {
        background_color: Color::NONE,
        ..default()
    }
}

fn text(value: &str, font_size: f32, color: Color) -> UTextLabel {
    UTextLabel {
        text: value.to_string(),
        font_size,
        color,
        ..default()
    }
}
