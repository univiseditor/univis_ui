//! # Complex Grid Dashboard Example
//!
//! Demonstrates a highly complex screen-space UI dashboard designed entirely
//! using Bevy's Grid Layout via Univis UI.
//!
//! It utilizes a 6-column by 5-row grid layout with mixed fractional (`Fr`) and fixed (`Px`)
//! tracks, explicit item placement, spanning columns/rows, custom borders, shapes, and inner flex widgets.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI - Complex Grid Command Dashboard".into(),
                    resolution: (1240, 840).into(),
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
    // 1. Spawns standard 2D Camera
    commands.spawn(Camera2d);

    // 2. Full-Screen Screen-Space Root UI Node
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.02, 0.03, 0.05), // Deep dark space background
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

    // 3. Shell Container
    let shell = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Px(1200.0),
                height: UVal::Px(800.0),
                padding: USides::all(20.0),
                background_color: Color::srgba(0.05, 0.07, 0.1, 0.98),
                border_radius: UCornerRadius::all(24.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.15),
                width: 1.5,
                radius: UCornerRadius::all(24.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .id();

    // 4. Grid Container (6 Columns x 5 Rows)
    let grid = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgba(0.08, 0.1, 0.15, 0.5),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.5, 1.0, 0.08),
                width: 1.0,
                radius: UCornerRadius::all(18.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 6,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(12.0),
                        column_gap: Some(12.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        // 6 Columns: Mixed stretch weights
                        template_columns: vec![
                            UTrackSize::Fr(1.0), // Column 1
                            UTrackSize::Fr(1.2), // Column 2
                            UTrackSize::Fr(1.5), // Column 3
                            UTrackSize::Fr(1.0), // Column 4
                            UTrackSize::Fr(1.2), // Column 5
                            UTrackSize::Fr(1.1), // Column 6
                        ],
                        // 5 Rows: Fixed Header/Footer, flexible center panels
                        template_rows: vec![
                            UTrackSize::Px(56.0), // Row 1 (Header)
                            UTrackSize::Fr(1.0),  // Row 2 (Top body)
                            UTrackSize::Fr(1.2),  // Row 3 (Mid body)
                            UTrackSize::Fr(0.8),  // Row 4 (Low body)
                            UTrackSize::Px(44.0), // Row 5 (Footer)
                        ],
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    // 5. Populate Grid Cards

    // Tile 1: Top Header Panel (Spans all 6 columns)
    let header_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.05, 0.1, 0.2, 0.6),
        Color::srgb(0.0, 0.8, 1.0), // Neon Cyan
        2.5,
        12.0,
        UShapeMode::Cut, // Tech bevel look
        grid_item(1, 6, 1, 1),
    );
    // Customize Header Card Inner Layout
    commands.entity(header_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Row,
        justify_content: UJustifyContent::SpaceBetween,
        align_items: UAlignItems::Center,
        ..default()
    });
    commands.spawn((
        ChildOf(header_card),
        label_node(),
        text("◈ STARSHIP COMMAND CONTROL DASHBOARD ◈", 18.0, Color::WHITE),
    ));
    // Status Badge
    let status_badge = commands
        .spawn((
            ChildOf(header_card),
            UNode {
                padding: USides::axes(14.0, 6.0),
                background_color: Color::srgba(0.0, 0.8, 0.3, 0.25),
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            UBorder {
                color: Color::srgb(0.0, 1.0, 0.4),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
                offset: 0.0,
            },
        ))
        .id();
    commands.spawn((
        ChildOf(status_badge),
        label_node(),
        text("SYSTEM STATUS: ONLINE", 12.0, Color::srgb(0.5, 1.0, 0.7)),
    ));

    // Tile 2: Primary Core Telemetry (Spans 2 columns, 2 rows)
    let core_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.05, 0.06, 0.09, 0.75),
        Color::srgb(0.9, 0.5, 0.0), // Neon Amber
        1.5,
        16.0,
        UShapeMode::Round,
        grid_item(1, 2, 2, 2),
    );
    commands.entity(core_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        justify_content: UJustifyContent::Start,
        gap: 12.0,
        ..default()
    });
    commands.spawn((
        ChildOf(core_card),
        label_node(),
        text("REACTOR CORE METRICS", 16.0, Color::srgb(1.0, 0.7, 0.3)),
    ));
    // Sub-items for reactor core status
    let mut add_core_row = |parent, label: &str, value: &str, color: Color| {
        let row = commands
            .spawn((
                ChildOf(parent),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    ..default()
                },
            ))
            .id();
        commands.spawn((
            ChildOf(row),
            label_node(),
            text(label, 13.0, Color::srgb(0.7, 0.75, 0.8)),
        ));
        commands.spawn((ChildOf(row), label_node(), text(value, 13.0, color)));
    };
    add_core_row(
        core_card,
        "Reactor Temp:",
        "4,820 °C",
        Color::srgb(1.0, 0.3, 0.3),
    );
    add_core_row(
        core_card,
        "Coolant Flow:",
        "100% (STABLE)",
        Color::srgb(0.3, 1.0, 0.5),
    );
    add_core_row(
        core_card,
        "Fuel Cell Charge:",
        "94.2% (OK)",
        Color::srgb(0.3, 0.8, 1.0),
    );
    add_core_row(
        core_card,
        "Plasma Coherence:",
        "99.8%",
        Color::srgb(0.9, 0.4, 1.0),
    );

    // Tile 3: Main Tactical Visualization Map (Spans 3 columns, 2 rows)
    let tactical_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.04, 0.06, 0.12, 0.7),
        Color::srgb(0.0, 0.6, 1.0), // Deep blue/cyan
        2.0,
        20.0,
        UShapeMode::Round,
        grid_item(3, 3, 2, 2),
    );
    commands.entity(tactical_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        justify_content: UJustifyContent::SpaceBetween,
        align_items: UAlignItems::Center,
        ..default()
    });
    commands.spawn((
        ChildOf(tactical_card),
        label_node(),
        text(
            "TACTICAL TELEMETRY SECTOR 4-B",
            16.0,
            Color::srgb(0.4, 0.8, 1.0),
        ),
    ));
    // Simulating a radar grid
    let radar_circle = commands
        .spawn((
            ChildOf(tactical_card),
            UNode {
                width: UVal::Px(150.0),
                height: UVal::Px(150.0),
                background_color: Color::srgba(0.0, 0.6, 1.0, 0.04),
                border_radius: UCornerRadius::all(75.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.7, 1.0, 0.3),
                width: 2.0,
                radius: UCornerRadius::all(75.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();
    // Radar Blip
    commands.spawn((
        ChildOf(radar_circle),
        UNode {
            width: UVal::Px(12.0),
            height: UVal::Px(12.0),
            background_color: Color::srgb(1.0, 0.2, 0.2), // Enemy radar blip
            border_radius: UCornerRadius::all(6.0),
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(tactical_card),
        label_node(),
        text(
            "TARGET RANGE: 14,000 km | THREAT: LOW",
            13.0,
            Color::srgb(0.7, 0.8, 0.9),
        ),
    ));

    // Tile 4: Command Controls Panel (Spans 1 column, 1 row)
    let control_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.06, 0.05, 0.08, 0.8),
        Color::srgb(1.0, 0.2, 0.7), // Hot Pink
        1.5,
        14.0,
        UShapeMode::Cut, // Bevel look
        grid_item(6, 1, 2, 1),
    );
    commands.entity(control_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        justify_content: UJustifyContent::Center,
        align_items: UAlignItems::Center,
        gap: 10.0,
        ..default()
    });
    commands.spawn((
        ChildOf(control_card),
        label_node(),
        text("WARP CORE", 14.0, Color::WHITE),
    ));
    commands.spawn((
        ChildOf(control_card),
        UToggle::sci_fi_style().with_checked(true),
    ));
    commands.spawn((
        ChildOf(control_card),
        label_node(),
        text("DRIVE STATUS", 11.0, Color::srgb(0.6, 0.6, 0.7)),
    ));

    // Tile 5: Auxiliary Backup Battery (Spans 1 column, 2 rows)
    let aux_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.06, 0.04, 0.09, 0.75),
        Color::srgb(0.6, 0.2, 1.0), // Glowing Purple
        1.5,
        14.0,
        UShapeMode::Round,
        grid_item(6, 1, 3, 2),
    );
    commands.entity(aux_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        justify_content: UJustifyContent::SpaceBetween,
        align_items: UAlignItems::Center,
        ..default()
    });
    commands.spawn((
        ChildOf(aux_card),
        label_node(),
        text("AUX POWER", 14.0, Color::srgb(0.8, 0.6, 1.0)),
    ));
    // Vertical battery fill simulation
    let battery_shell = commands
        .spawn((
            ChildOf(aux_card),
            UNode {
                width: UVal::Px(40.0),
                height: UVal::Px(120.0),
                padding: USides::all(3.0),
                background_color: Color::srgba(0.2, 0.0, 0.4, 0.2),
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            UBorder {
                color: Color::srgb(0.6, 0.2, 1.0),
                width: 1.5,
                radius: UCornerRadius::all(6.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::ColumnReverse, // Fill from bottom
                ..default()
            },
        ))
        .id();
    commands.spawn((
        ChildOf(battery_shell),
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(0.78), // 78% charge
            background_color: Color::srgb(0.7, 0.3, 1.0),
            border_radius: UCornerRadius::all(3.0),
            ..default()
        },
    ));
    commands.spawn((
        ChildOf(aux_card),
        label_node(),
        text("78% CAP", 13.0, Color::srgb(0.8, 0.7, 0.9)),
    ));

    // Tile 6: Log Activities Panel (Spans 3 columns, 1 row)
    let log_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.04, 0.07, 0.05, 0.75),
        Color::srgb(0.0, 0.9, 0.4), // Emerald Green
        2.0,
        15.0,
        UShapeMode::Round,
        grid_item(1, 3, 4, 1),
    );
    commands.entity(log_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        justify_content: UJustifyContent::Start,
        gap: 6.0,
        ..default()
    });
    commands.spawn((
        ChildOf(log_card),
        label_node(),
        text(
            "SYSTEM EVENT LOGS (SECTOR 4)",
            14.0,
            Color::srgb(0.4, 1.0, 0.6),
        ),
    ));
    // Add log lines
    let mut add_log_line = |parent, prefix: &str, message: &str| {
        let row = commands
            .spawn((
                ChildOf(parent),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    ..default()
                },
            ))
            .id();
        commands.spawn((
            ChildOf(row),
            label_node(),
            text(prefix, 11.0, Color::srgb(0.4, 0.8, 0.5)),
        ));
        commands.spawn((
            ChildOf(row),
            label_node(),
            text(message, 11.0, Color::srgb(0.8, 0.85, 0.8)),
        ));
    };
    add_log_line(log_card, "[12:38:02]", "Warp drive stabilizer aligned.");
    add_log_line(
        log_card,
        "[12:38:05]",
        "Auxiliary shield generator testing completed successfully.",
    );
    add_log_line(
        log_card,
        "[12:38:11]",
        "Warning: Sector 4-B threat scanning initiated.",
    );

    // Tile 7: Shield & Deflector Controls (Spans 2 columns, 1 row)
    let shield_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.08, 0.07, 0.04, 0.75),
        Color::srgb(1.0, 0.8, 0.0), // Neon Yellow/Gold
        1.5,
        16.0,
        UShapeMode::Cut, // Beveled look
        grid_item(4, 2, 4, 1),
    );
    commands.entity(shield_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Column,
        justify_content: UJustifyContent::Center,
        gap: 8.0,
        ..default()
    });
    commands.spawn((
        ChildOf(shield_card),
        label_node(),
        text("DEFLECTOR SHIELDS LEVEL", 14.0, Color::srgb(1.0, 0.9, 0.4)),
    ));
    // Shield seekbar
    commands.spawn((
        ChildOf(shield_card),
        USeekBar::sci_fi_style()
            .with_range(0.0, 100.0)
            .with_value(0.85) // 85% shield
            .show_value(),
    ));

    // Tile 8: Bottom Footer Panel (Spans all 6 columns)
    let footer_card = spawn_grid_card(
        &mut commands,
        grid,
        Color::srgba(0.03, 0.04, 0.06, 0.9),
        Color::srgba(0.2, 0.5, 1.0, 0.25), // Translucent blue
        1.0,
        10.0,
        UShapeMode::Round,
        grid_item(1, 6, 5, 1),
    );
    commands.entity(footer_card).insert(ULayout {
        display: UDisplay::Flex,
        flex_direction: UFlexDirection::Row,
        justify_content: UJustifyContent::SpaceBetween,
        align_items: UAlignItems::Center,
        ..default()
    });
    commands.spawn((
        ChildOf(footer_card),
        label_node(),
        text(
            "UNIVIS ENGINE - GRID ARCHITECTURE DEMO",
            12.0,
            Color::srgb(0.5, 0.6, 0.7),
        ),
    ));
    commands.spawn((
        ChildOf(footer_card),
        label_node(),
        text(
            "VERSION 0.3.0 | SECURITY MODE A",
            12.0,
            Color::srgb(0.5, 0.6, 0.7),
        ),
    ));
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

// Spawns a standard grid cell panel with a border and custom shape mode
fn spawn_grid_card(
    commands: &mut Commands,
    parent: Entity,
    bg_color: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    shape: UShapeMode,
    grid_placement: USelf,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            grid_placement,
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(16.0),
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                shape_mode: shape,
                ..default()
            },
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .id()
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
