use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowPlugin};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Responsive Tactical HUD Demo".to_string(),
                resolution: (1280, 860).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<LayoutState>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_responsive_hud)
        .run();
}

#[derive(Component)]
struct ResponsiveRoot;

#[derive(Component)]
struct BreakpointIndicator;

#[derive(Component)]
struct SimulatedScreensContainer;

#[derive(Component)]
struct SimulatedFrame;

#[derive(Component)]
struct GridExample {
    id: u32,
}

#[derive(Component, Clone, Copy)]
struct HudBlock {
    example_id: u32,
    letter: char,
}

#[derive(Resource, Default)]
struct LayoutState {
    last_width: f32,
    last_height: f32,
    is_desktop: Option<bool>,
}

fn setup(mut commands: Commands) {
    // Spawn 2D Camera
    commands.spawn(Camera2d);

    // Fullscreen dark cyber background root UI
    let root = commands
        .spawn((
            URootUi::screen(),
            ResponsiveRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(20.0),
                background_color: Color::srgb(0.02, 0.03, 0.05), // Deep space blue/black
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Start,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    // Main header block
    let header_panel = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(24.0, 16.0),
                background_color: Color::srgba(0.04, 0.08, 0.15, 0.7),
                border_radius: UCornerRadius::all(14.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.35),
                width: 1.5,
                radius: UCornerRadius::all(14.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    // Title text group
    let title_group = spawn_container(
        &mut commands,
        header_panel,
        UVal::Auto,
        UFlexDirection::Column,
        4.0,
    );
    commands.spawn((
        ChildOf(title_group),
        label_node(),
        text(
            "◈ TACTICAL RESPONSIVE HUD CONSOLE ◈",
            22.0,
            Color::srgb(0.0, 0.8, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(title_group),
        label_node(),
        text(
            "Simulating 3 screen layout guidelines from Vegas Web (Desktop vs Mobile layouts)",
            12.0,
            Color::srgb(0.5, 0.6, 0.7),
        ),
    ));

    // Breakpoint indicator pill
    let indicator_group = spawn_container(
        &mut commands,
        header_panel,
        UVal::Auto,
        UFlexDirection::Row,
        8.0,
    );
    commands.spawn((
        ChildOf(indicator_group),
        BreakpointIndicator,
        UNode {
            padding: USides::axes(16.0, 8.0),
            background_color: Color::srgba(0.0, 0.8, 1.0, 0.15),
            border_radius: UCornerRadius::all(12.0),
            ..default()
        },
        UBorder {
            color: Color::srgb(0.0, 0.8, 1.0),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
            offset: 0.0,
        },
        UTextLabel {
            text: "ACTIVE BREAKPOINT: INITIALIZING".to_string(),
            font_size: 11.0,
            color: Color::srgb(0.0, 0.8, 1.0),
            ..default()
        },
    ));

    // Main simulated screens container
    // This container switches from 3 columns (side-by-side) to 1 column (vertical stack)
    let screens_container = commands
        .spawn((
            ChildOf(root),
            SimulatedScreensContainer,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(0.90), // Take up remaining space
                padding: USides::all(10.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                gap: 20.0,
                grid_columns: 3,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(20.0),
                        column_gap: Some(20.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: vec![
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    // Spawning 3 Simulated Device Frames
    spawn_example_frame(
        &mut commands,
        screens_container,
        1,
        "EXAMPLE 1: 3-BLOCK GRID",
    );
    spawn_example_frame(
        &mut commands,
        screens_container,
        2,
        "EXAMPLE 2: SPLIT SIDEBAR",
    );
    spawn_example_frame(
        &mut commands,
        screens_container,
        3,
        "EXAMPLE 3: DATA MATRIX",
    );
}

fn spawn_example_frame(commands: &mut Commands, parent: Entity, id: u32, title: &str) {
    let frame = commands
        .spawn((
            ChildOf(parent),
            SimulatedFrame,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                min_height: 0.0,
                padding: USides::all(16.0),
                background_color: Color::srgba(0.03, 0.05, 0.09, 0.8), // Translucent dark
                border_radius: UCornerRadius::all(20.0),
                shape_mode: UShapeMode::Cut,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.3),
                width: 1.5,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Start,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    // Frame Title Bar
    let title_bar = commands
        .spawn((
            ChildOf(frame),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::bottom(6.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.0, 0.8, 1.0, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(0.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(title_bar),
        label_node(),
        text(title, 13.0, Color::srgb(0.0, 0.8, 1.0)),
    ));

    commands.spawn((
        ChildOf(title_bar),
        label_node(),
        text("TELEMETRY STATUS: OK", 10.0, Color::srgb(0.1, 0.9, 0.3)),
    ));

    // Frame Grid Area (This holds the A-H blocks)
    let grid_area = commands
        .spawn((
            ChildOf(frame),
            GridExample { id },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_grow: Some(1.0),
                        flex_shrink: Some(1.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto, // grows to fill remaining space
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                gap: 10.0,
                grid_columns: if id == 1 { 2 } else { 4 },
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(10.0),
                        column_gap: Some(10.0),
                        ..default()
                    },
                    grid: ULayoutGridContainer {
                        template_columns: if id == 1 {
                            vec![UTrackSize::Fr(2.0), UTrackSize::Fr(1.0)]
                        } else {
                            vec![
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.5),
                            ]
                        },
                        template_rows: vec![
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(2.0),
                            UTrackSize::Fr(2.0),
                        ],
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    // Spawn HUD Blocks depending on example ID
    // HSL-like curated neon colors
    let cyan = Color::srgb(0.0, 0.8, 1.0);
    let green = Color::srgb(0.1, 0.9, 0.3);
    let orange = Color::srgb(1.0, 0.6, 0.0);
    let violet = Color::srgb(0.7, 0.3, 1.0);
    let red = Color::srgb(1.0, 0.2, 0.2);

    match id {
        1 => {
            spawn_hud_block(
                commands,
                grid_area,
                1,
                'A',
                "SYS_A: MAIN_CORE",
                "ACTIVE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                1,
                'B',
                "SYS_B: POWER_SHIELD",
                "ONLINE",
                green,
            );
            spawn_hud_block(
                commands,
                grid_area,
                1,
                'C',
                "SYS_C: SECTOR_GRID",
                "ONLINE",
                orange,
            );
        }
        2 => {
            spawn_hud_block(
                commands,
                grid_area,
                2,
                'A',
                "SYS_A: TELEMETRY",
                "ACTIVE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                2,
                'B',
                "SYS_B: REACTOR_LINK",
                "ONLINE",
                green,
            );
            spawn_hud_block(
                commands,
                grid_area,
                2,
                'C',
                "SYS_C: THRUSTERS",
                "STABLE",
                orange,
            );
            spawn_hud_block(
                commands,
                grid_area,
                2,
                'D',
                "SYS_D: COOLING_SYS",
                "STABLE",
                violet,
            );
            spawn_hud_block(
                commands,
                grid_area,
                2,
                'E',
                "SYS_E: COMMS_LINK",
                "STABLE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                2,
                'F',
                "SYS_F: RADAR_MATRIX",
                "SCANNING",
                red,
            );
        }
        3 => {
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'A',
                "SYS_A: GLOBAL_GRID",
                "ACTIVE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'B',
                "SYS_B: SENSORS_L1",
                "STABLE",
                green,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'C',
                "SYS_C: SENSORS_L2",
                "STABLE",
                green,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'D',
                "SYS_D: SENSORS_L3",
                "STABLE",
                green,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'E',
                "SYS_E: AMBIENT_L1",
                "STABLE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'F',
                "SYS_F: AMBIENT_L2",
                "STABLE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'G',
                "SYS_G: AMBIENT_L3",
                "STABLE",
                cyan,
            );
            spawn_hud_block(
                commands,
                grid_area,
                3,
                'H',
                "SYS_H: LOGS_STREAM",
                "STREAMING",
                orange,
            );
        }
        _ => {}
    }
}

fn spawn_hud_block(
    commands: &mut Commands,
    parent: Entity,
    example_id: u32,
    letter: char,
    label: &str,
    status: &str,
    color: Color,
) {
    let block = commands
        .spawn((
            ChildOf(parent),
            HudBlock { example_id, letter },
            USelf::default(), // Grid position will be updated dynamically
            UNode {
                padding: USides::all(10.0),
                background_color: Color::srgba(0.04, 0.08, 0.14, 0.75), // Glass panel backing
                border_radius: UCornerRadius::all(10.0),
                shape_mode: UShapeMode::Cut, // Beautiful HUD chamfered corners
                ..default()
            },
            UBorder {
                color: Color::srgba(
                    color.to_linear().red,
                    color.to_linear().green,
                    color.to_linear().blue,
                    0.3,
                ),
                width: 1.5,
                radius: UCornerRadius::all(10.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    // Spawning header row inside the block
    let header_row = commands
        .spawn((
            ChildOf(block),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(header_row),
        label_node(),
        text(
            label,
            8.0,
            Color::srgba(
                color.to_linear().red,
                color.to_linear().green,
                color.to_linear().blue,
                0.75,
            ),
        ),
    ));

    commands.spawn((ChildOf(header_row), label_node(), text(status, 8.0, color)));

    // Big central letter
    commands.spawn((
        ChildOf(block),
        label_node(),
        text(&letter.to_string(), 28.0, color),
    ));

    // Bottom telemetry level micro bar
    let bar_container = commands
        .spawn((
            ChildOf(block),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(4.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                border_radius: UCornerRadius::all(2.0),
                ..default()
            },
        ))
        .id();

    let fill_percent = match letter {
        'A' => 0.95,
        'B' => 0.78,
        'C' => 0.45,
        'D' => 0.62,
        'E' => 0.35,
        'F' => 0.88,
        'G' => 0.52,
        'H' => 0.67,
        _ => 0.5,
    };

    commands.spawn((
        ChildOf(bar_container),
        UNode {
            width: UVal::Percent(fill_percent),
            height: UVal::Percent(1.0),
            background_color: color,
            border_radius: UCornerRadius::all(2.0),
            ..default()
        },
    ));
}

fn update_responsive_hud(
    windows: Query<&Window, With<PrimaryWindow>>,
    root_query: Query<Entity, With<ResponsiveRoot>>,
    mut indicator: Query<&mut UTextLabel, With<BreakpointIndicator>>,
    mut state: ResMut<LayoutState>,
    mut invalidate_queue: ResMut<UiInvalidateRequestQueue>,
    mut main_container: Query<
        (Entity, &mut ULayout),
        (With<SimulatedScreensContainer>, Without<GridExample>),
    >,
    mut examples_grids: Query<
        (Entity, &GridExample, &mut ULayout),
        (With<GridExample>, Without<SimulatedScreensContainer>),
    >,
    mut frames: Query<(Entity, &mut UNode), With<SimulatedFrame>>,
    mut blocks: Query<(Entity, &HudBlock, &mut USelf)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let width = window.width();
    let height = window.height();
    let is_desktop = width >= 900.0;

    let resized =
        (width - state.last_width).abs() > 0.5 || (height - state.last_height).abs() > 0.5;
    let mode_changed = state.is_desktop != Some(is_desktop);

    state.last_width = width;
    state.last_height = height;

    if resized || mode_changed {
        if let Ok(root_entity) = root_query.single() {
            invalidate_queue.request(root_entity, UiLayoutInvalidation::intrinsic_change());
        }
    }

    if !mode_changed && !resized {
        return;
    }

    state.is_desktop = Some(is_desktop);

    // Update breakpoint label display
    for mut label in &mut indicator {
        let label_text = if is_desktop {
            format!(
                "ACTIVE BREAKPOINT: DESKTOP (W: {}px >= 900px)",
                width as u32
            )
        } else {
            format!("ACTIVE BREAKPOINT: MOBILE (W: {}px < 900px)", width as u32)
        };
        if label.text != label_text {
            label.text = label_text;
        }
    }

    // Update SimulatedScreensContainer Grid
    for (entity, mut layout) in &mut main_container {
        if is_desktop {
            layout.grid_columns = 3;
            layout.container_ext.grid.template_columns = vec![
                UTrackSize::Fr(1.0),
                UTrackSize::Fr(1.0),
                UTrackSize::Fr(1.0),
            ];
        } else {
            layout.grid_columns = 1;
            layout.container_ext.grid.template_columns = vec![UTrackSize::Fr(1.0)];
        }
        invalidate_queue.request(entity, UiLayoutInvalidation::intrinsic_change());
    }

    // Update individual examples grids template columns/rows
    for (entity, example, mut layout) in &mut examples_grids {
        match example.id {
            1 => {
                if is_desktop {
                    layout.grid_columns = 2;
                    layout.container_ext.grid.template_columns =
                        vec![UTrackSize::Fr(2.0), UTrackSize::Fr(1.0)];
                    layout.container_ext.grid.template_rows =
                        vec![UTrackSize::Fr(1.0), UTrackSize::Fr(2.5)];
                } else {
                    layout.grid_columns = 1;
                    layout.container_ext.grid.template_columns = vec![UTrackSize::Fr(1.0)];
                    layout.container_ext.grid.template_rows = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(2.0),
                        UTrackSize::Fr(1.5),
                    ];
                }
            }
            2 => {
                if is_desktop {
                    layout.grid_columns = 4;
                    layout.container_ext.grid.template_columns = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.5),
                    ];
                    layout.container_ext.grid.template_rows = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.5),
                        UTrackSize::Fr(1.5),
                    ];
                } else {
                    layout.grid_columns = 3;
                    layout.container_ext.grid.template_columns = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                    ];
                    layout.container_ext.grid.template_rows = vec![
                        UTrackSize::Fr(0.8),
                        UTrackSize::Fr(1.5),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.2),
                    ];
                }
            }
            3 => {
                if is_desktop {
                    layout.grid_columns = 4;
                    layout.container_ext.grid.template_columns = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.5),
                    ];
                    layout.container_ext.grid.template_rows = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.5),
                        UTrackSize::Fr(1.5),
                    ];
                } else {
                    layout.grid_columns = 3;
                    layout.container_ext.grid.template_columns = vec![
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                        UTrackSize::Fr(1.0),
                    ];
                    layout.container_ext.grid.template_rows = vec![
                        UTrackSize::Fr(0.8),
                        UTrackSize::Fr(1.2),
                        UTrackSize::Fr(1.2),
                        UTrackSize::Fr(1.2),
                    ];
                }
            }
            _ => {}
        }
        invalidate_queue.request(entity, UiLayoutInvalidation::intrinsic_change());
    }

    // Update simulated frames height
    for (entity, mut node) in &mut frames {
        if is_desktop {
            node.height = UVal::Percent(1.0);
            node.min_height = 0.0;
        } else {
            node.height = UVal::Percent(0.3);
            node.min_height = 0.0;
        }
        invalidate_queue.request(entity, UiLayoutInvalidation::intrinsic_change());
    }

    // Update individual block positions
    for (entity, block, mut uself) in &mut blocks {
        let placement = get_placement(block.example_id, block.letter, is_desktop);
        uself.item_ext.grid = placement;
        invalidate_queue.request(entity, UiLayoutInvalidation::intrinsic_change());
    }
}

fn get_placement(example_id: u32, letter: char, is_desktop: bool) -> ULayoutGridItem {
    if is_desktop {
        match (example_id, letter) {
            // Example 1
            (1, 'A') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 2,
                row_start: Some(1),
                row_span: 1,
            },
            (1, 'B') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (1, 'C') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },

            // Example 2
            (2, 'A') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 4,
                row_start: Some(1),
                row_span: 1,
            },
            (2, 'B') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 3,
                row_start: Some(2),
                row_span: 1,
            },
            (2, 'C') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (2, 'D') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (2, 'E') => ULayoutGridItem {
                column_start: Some(3),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (2, 'F') => ULayoutGridItem {
                column_start: Some(4),
                column_span: 1,
                row_start: Some(2),
                row_span: 2,
            },

            // Example 3
            (3, 'A') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 4,
                row_start: Some(1),
                row_span: 1,
            },
            (3, 'B') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (3, 'C') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (3, 'D') => ULayoutGridItem {
                column_start: Some(3),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (3, 'E') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (3, 'F') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (3, 'G') => ULayoutGridItem {
                column_start: Some(3),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (3, 'H') => ULayoutGridItem {
                column_start: Some(4),
                column_span: 1,
                row_start: Some(2),
                row_span: 2,
            },

            _ => ULayoutGridItem::default(),
        }
    } else {
        match (example_id, letter) {
            // Example 1 (Stacked)
            (1, 'A') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(1),
                row_span: 1,
            },
            (1, 'B') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (1, 'C') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },

            // Example 2 (Stacked: A, B, [C, D, E], F)
            (2, 'A') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 3,
                row_start: Some(1),
                row_span: 1,
            },
            (2, 'B') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 3,
                row_start: Some(2),
                row_span: 1,
            },
            (2, 'C') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (2, 'D') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (2, 'E') => ULayoutGridItem {
                column_start: Some(3),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (2, 'F') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 3,
                row_start: Some(4),
                row_span: 1,
            },

            // Example 3 (Stacked: A, [B, C, D], [E, F, G], H)
            (3, 'A') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 3,
                row_start: Some(1),
                row_span: 1,
            },
            (3, 'B') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (3, 'C') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (3, 'D') => ULayoutGridItem {
                column_start: Some(3),
                column_span: 1,
                row_start: Some(2),
                row_span: 1,
            },
            (3, 'E') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (3, 'F') => ULayoutGridItem {
                column_start: Some(2),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (3, 'G') => ULayoutGridItem {
                column_start: Some(3),
                column_span: 1,
                row_start: Some(3),
                row_span: 1,
            },
            (3, 'H') => ULayoutGridItem {
                column_start: Some(1),
                column_span: 3,
                row_start: Some(4),
                row_span: 1,
            },

            _ => ULayoutGridItem::default(),
        }
    }
}

fn spawn_container(
    commands: &mut Commands,
    parent: Entity,
    width: UVal,
    direction: UFlexDirection,
    gap: f32,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode {
                width,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: direction,
                gap,
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
