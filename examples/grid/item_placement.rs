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
                width: UVal::Px(1040.0),
                height: UVal::Px(680.0),
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
        text(
            "Grid item placement: `start` + `span`",
            30.0,
            Color::srgb(0.96, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "This example uses `ULayoutGridItem` on `USelf.item_ext.grid` to place cards explicitly.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let grid = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(990.0),
                height: UVal::Px(540.0),
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
                        row_gap: Some(12.0),
                        column_gap: Some(12.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: vec![
                            UTrackSize::Px(228.0),
                            UTrackSize::Px(228.0),
                            UTrackSize::Px(228.0),
                            UTrackSize::Px(228.0),
                        ],
                        template_rows: vec![
                            UTrackSize::Px(86.0),
                            UTrackSize::Px(150.0),
                            UTrackSize::Px(150.0),
                            UTrackSize::Px(94.0),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    spawn_card(
        &mut commands,
        grid,
        "Header",
        "column 1..4",
        Color::srgba(0.19, 0.34, 0.78, 0.96),
        grid_item(1, 4, 1, 1),
    );
    spawn_card(
        &mut commands,
        grid,
        "Sidebar",
        "col 1, rows 2..3",
        Color::srgba(0.16, 0.57, 0.73, 0.96),
        grid_item(1, 1, 2, 2),
    );
    spawn_card(
        &mut commands,
        grid,
        "Main chart",
        "col 2..4, rows 2..3",
        Color::srgba(0.18, 0.68, 0.48, 0.96),
        grid_item(2, 3, 2, 2),
    );
    spawn_card(
        &mut commands,
        grid,
        "Log",
        "col 1, row 4",
        Color::srgba(0.9, 0.65, 0.18, 0.96),
        grid_item(1, 1, 4, 1),
    );
    spawn_card(
        &mut commands,
        grid,
        "Footer A",
        "col 2..3, row 4",
        Color::srgba(0.77, 0.27, 0.31, 0.96),
        grid_item(2, 2, 4, 1),
    );
    spawn_card(
        &mut commands,
        grid,
        "Footer B",
        "col 4, row 4",
        Color::srgba(0.49, 0.22, 0.67, 0.96),
        grid_item(4, 1, 4, 1),
    );
}

fn grid_item(column_start: u32, column_span: u32, row_start: u32, row_span: u32) -> USelf {
    USelf {
        item_ext: ULayoutItemExt {
            grid: ULayoutGridItem {
                column_start: Some(column_start),
                column_span,
                row_start: Some(row_start),
                row_span,
            },
            ..default()
        },
        ..default()
    }
}

fn spawn_card(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
    background_color: Color,
    uself: USelf,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            uself,
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                min_height: 80.0,
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

    commands.spawn((ChildOf(card), label_node(), text(title, 22.0, Color::WHITE)));
    commands.spawn((
        ChildOf(card),
        label_node(),
        text(subtitle, 14.0, Color::srgba(1.0, 1.0, 1.0, 0.88)),
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
