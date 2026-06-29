//! # Simple Blocks Grid Example
//!
//! A basic grid layout consisting of simple colored squares/blocks to clearly demonstrate
//! row and column spanning.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI - Simple Blocks Grid".into(),
                    resolution: (960, 540).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Screen-space UI root
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                background_color: Color::srgb(0.05, 0.06, 0.08),
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

    // Grid Container
    // 4 columns x 3 rows with 10px gaps
    let grid = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(800.0),
                height: UVal::Px(400.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 4,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(10.0),
                        column_gap: Some(10.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        // 4 Columns of equal width (1fr each)
                        template_columns: vec![
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                        ],
                        // 3 Rows with custom fixed pixel heights
                        template_rows: vec![
                            UTrackSize::Px(60.0),
                            UTrackSize::Px(180.0),
                            UTrackSize::Px(100.0),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    // Spawning the simple colored blocks (squares) with custom text showing spans:

    // 1. Block A (Header): Spans columns 1 to 4, Row 1
    spawn_block(
        &mut commands,
        grid,
        "Block A",
        "Col: 1..=4 | Row: 1",
        Color::srgb(0.2, 0.4, 0.8), // Blue
        grid_item(1, 4, 1, 1),
    );

    // 2. Block B (Sidebar): Column 1, Spans rows 2 to 3
    spawn_block(
        &mut commands,
        grid,
        "Block B",
        "Col: 1 | Row: 2..=3",
        Color::srgb(0.8, 0.3, 0.3), // Red
        grid_item(1, 1, 2, 2),
    );

    // 3. Block C (Main Content): Spans columns 2 to 3, Row 2
    spawn_block(
        &mut commands,
        grid,
        "Block C",
        "Col: 2..=3 | Row: 2",
        Color::srgb(0.2, 0.7, 0.4), // Green
        grid_item(2, 2, 2, 1),
    );

    // 4. Block D (Right Sidebar): Column 4, Row 2
    spawn_block(
        &mut commands,
        grid,
        "Block D",
        "Col: 4 | Row: 2",
        Color::srgb(0.8, 0.6, 0.2), // Yellow/Orange
        grid_item(4, 1, 2, 1),
    );

    // 5. Block E (Bottom Left): Column 2, Row 3
    spawn_block(
        &mut commands,
        grid,
        "Block E",
        "Col: 2 | Row: 3",
        Color::srgb(0.5, 0.3, 0.8), // Purple
        grid_item(2, 1, 3, 1),
    );

    // 6. Block F (Bottom Right): Spans columns 3 to 4, Row 3
    spawn_block(
        &mut commands,
        grid,
        "Block F",
        "Col: 3..=4 | Row: 3",
        Color::srgb(0.3, 0.7, 0.8), // Cyan
        grid_item(3, 2, 3, 1),
    );
}

// Helper to define Grid Item positions
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

// Spawns a simple colored square/rectangle block
fn spawn_block(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    description: &str,
    bg_color: Color,
    grid_placement: USelf,
) {
    let block = commands
        .spawn((
            ChildOf(parent),
            grid_placement,
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(12.0),
                background_color: bg_color,
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(block),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: title.to_string(),
            font_size: 18.0,
            color: Color::WHITE,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(block),
        UNode {
            background_color: Color::NONE,
            ..default()
        },
        UTextLabel {
            text: description.to_string(),
            font_size: 12.0,
            color: Color::srgba(1.0, 1.0, 1.0, 0.8),
            ..default()
        },
    ));
}
