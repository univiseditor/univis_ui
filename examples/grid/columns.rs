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
                width: UVal::Px(980.0),
                height: UVal::Px(600.0),
                padding: USides::all(24.0),
                background_color: Color::srgba(0.08, 0.1, 0.14, 0.98),
                border_radius: UCornerRadius::all(26.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.72, 0.8, 0.9, 0.18),
                width: 1.0,
                radius: UCornerRadius::all(26.0),
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
        text(
            "Grid: simple `grid_columns` mode",
            30.0,
            Color::srgb(0.96, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "This is the basic grid path: set `display = Grid` and choose `grid_columns`.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let grid = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(920.0),
                height: UVal::Px(430.0),
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
                display: UDisplay::Grid,
                grid_columns: 4,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(14.0),
                        column_gap: Some(14.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let tiles = [
        ("1", "Card one", Color::srgba(0.19, 0.34, 0.78, 0.96)),
        ("2", "Card two", Color::srgba(0.15, 0.58, 0.73, 0.96)),
        ("3", "Card three", Color::srgba(0.16, 0.67, 0.47, 0.96)),
        ("4", "Card four", Color::srgba(0.46, 0.69, 0.24, 0.96)),
        ("5", "Card five", Color::srgba(0.88, 0.66, 0.2, 0.96)),
        ("6", "Card six", Color::srgba(0.82, 0.44, 0.19, 0.96)),
        ("7", "Card seven", Color::srgba(0.77, 0.26, 0.31, 0.96)),
        ("8", "Card eight", Color::srgba(0.49, 0.22, 0.67, 0.96)),
    ];

    for (index, subtitle, color) in tiles {
        spawn_tile(&mut commands, grid, index, subtitle, color, None);
    }
}

fn spawn_tile(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
    background_color: Color,
    uself: Option<USelf>,
) {
    let entity = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                min_height: 82.0,
                padding: USides::all(14.0),
                background_color,
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    if let Some(uself) = uself {
        commands.entity(entity).insert(uself);
    }

    commands.spawn((
        ChildOf(entity),
        label_node(),
        text(title, 26.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(entity),
        label_node(),
        text(subtitle, 15.0, Color::srgba(1.0, 1.0, 1.0, 0.88)),
    ));
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
