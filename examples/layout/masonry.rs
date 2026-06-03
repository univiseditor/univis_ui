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
                width: UVal::Px(1120.0),
                height: UVal::Px(740.0),
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
        text("Masonry layout", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Both boards use the same cards. Only `grid_columns` changes to show how the shortest-column placement behaves.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let content = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(1072.0),
                height: UVal::Px(620.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    spawn_masonry_board(&mut commands, content, "3 columns", 3, 526.0);
    spawn_masonry_board(&mut commands, content, "5 columns", 5, 526.0);
}

fn spawn_masonry_board(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    columns: u32,
    width: f32,
) {
    let panel = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(width),
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
        text(title, 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "Cards are always inserted into the currently shortest column.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let board = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(width - 32.0),
                height: UVal::Px(540.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Masonry,
                grid_columns: columns,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(12.0),
                        column_gap: Some(12.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let heights = [
        92.0, 66.0, 132.0, 84.0, 118.0, 72.0, 146.0, 94.0, 76.0, 124.0, 88.0, 138.0,
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
        Color::srgba(0.72, 0.33, 0.64, 0.96),
        Color::srgba(0.34, 0.63, 0.23, 0.96),
    ];

    for (index, (height, color)) in heights.into_iter().zip(colors).enumerate() {
        let card = commands
            .spawn((
                ChildOf(board),
                UNode {
                    width: UVal::Flex(1.0),
                    height: UVal::Px(height),
                    padding: USides::all(12.0),
                    background_color: color,
                    border_radius: UCornerRadius::all(16.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    gap: 4.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(card),
            label_node(),
            text(&(index + 1).to_string(), 20.0, Color::WHITE),
        ));
        commands.spawn((
            ChildOf(card),
            label_node(),
            text(
                &format!("{height:.0}px"),
                13.0,
                Color::srgba(1.0, 1.0, 1.0, 0.86),
            ),
        ));
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
