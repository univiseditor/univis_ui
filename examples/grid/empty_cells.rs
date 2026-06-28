//! # Empty Cells Grid Example
//!
//! A grid layout showing how to place items at specific coordinates (column/row start)
//! to leave empty spaces within the grid, in a checkerboard layout.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI - Grid with Empty Cells".into(),
                    resolution: (960, 600).into(),
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
                background_color: Color::srgb(0.04, 0.05, 0.07),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 24.0,
                ..default()
            },
        ))
        .id();

    // Title & Subtitle Labels
    commands.spawn((
        ChildOf(root),
        UNode::default(),
        UTextLabel {
            text: "◈ GRID LAYOUT: EMPTY CELLS ◈".to_string(),
            font_size: 24.0,
            color: Color::srgb(0.3, 0.7, 1.0), // Cyber Cyan/Blue
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(root),
        UNode {
            margin: USides::bottom(16.0),
            ..default()
        },
        UTextLabel {
            text: "This showcase demonstrates explicit coordinate placement (1..3 columns & rows),\nleaving the remaining cells completely empty to form a checkerboard pattern.".to_string(),
            font_size: 14.0,
            color: Color::srgb(0.7, 0.8, 0.9),
            ..default()
        },
    ));

    // Grid Container
    // 3 columns x 3 rows with 12px gaps
    let grid = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(640.0),
                height: UVal::Px(420.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.03),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 3,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(12.0),
                        column_gap: Some(12.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        // 3 Columns of equal width (1fr each)
                        template_columns: vec![
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                        ],
                        // 3 Rows of equal height (120px each)
                        template_rows: vec![
                            UTrackSize::Px(120.0),
                            UTrackSize::Px(120.0),
                            UTrackSize::Px(120.0),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.2),
                width: 2.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
        ))
        .id();

    // Spawning 5 cards at specific cells, leaving the other 4 cells empty:

    // 1. Top-Left: Row 1, Col 1 (Neon Blue)
    spawn_grid_block(
        &mut commands,
        grid,
        "A",
        "Row: 1, Col: 1",
        Color::srgb(0.1, 0.4, 0.9),
        grid_item(1, 1),
    );

    // 2. Top-Right: Row 1, Col 3 (Neon Red)
    spawn_grid_block(
        &mut commands,
        grid,
        "B",
        "Row: 1, Col: 3",
        Color::srgb(0.9, 0.2, 0.3),
        grid_item(3, 1),
    );

    // 3. Center: Row 2, Col 2 (Neon Green)
    spawn_grid_block(
        &mut commands,
        grid,
        "C",
        "Row: 2, Col: 2",
        Color::srgb(0.1, 0.7, 0.4),
        grid_item(2, 2),
    );

    // 4. Bottom-Left: Row 3, Col 1 (Neon Orange)
    spawn_grid_block(
        &mut commands,
        grid,
        "D",
        "Row: 3, Col: 1",
        Color::srgb(0.9, 0.5, 0.1),
        grid_item(1, 3),
    );

    // 5. Bottom-Right: Row 3, Col 3 (Neon Purple)
    spawn_grid_block(
        &mut commands,
        grid,
        "E",
        "Row: 3, Col: 3",
        Color::srgb(0.6, 0.2, 0.9),
        grid_item(3, 3),
    );
}

// Helper to define Grid Item positions
fn grid_item(column_start: u32, row_start: u32) -> USelf {
    USelf {
        item_ext: ULayoutItemExt {
            grid: ULayoutGridItem {
                column_start: Some(column_start),
                column_span: 1,
                row_start: Some(row_start),
                row_span: 1,
            },
            ..default()
        },
        ..default()
    }
}

// Spawns a nice grid card/block
fn spawn_grid_block(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    coords: &str,
    accent_color: Color,
    grid_placement: USelf,
) {
    let block = commands
        .spawn((
            ChildOf(parent),
            grid_placement,
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(16.0),
                background_color: Color::srgba(0.08, 0.1, 0.15, 0.6),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
            UBorder {
                color: accent_color,
                width: 2.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
        ))
        .id();

    // Large letter title
    commands.spawn((
        ChildOf(block),
        UNode::default(),
        UTextLabel {
            text: format!("Block {}", label),
            font_size: 18.0,
            color: Color::WHITE,
            ..default()
        },
    ));

    // Coordinates detail text
    commands.spawn((
        ChildOf(block),
        UNode::default(),
        UTextLabel {
            text: coords.to_string(),
            font_size: 11.0,
            color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            ..default()
        },
    ));
}
