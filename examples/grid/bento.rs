use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "UnivisUI - Bento Grid Design".into(),
                    resolution: (800, 600).into(),
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
    commands.spawn(Camera2d);

    // Root UI container
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.12, 0.12, 0.12), // Dark background
                padding: USides::all(40.0),                      // Padding around the grid
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

    // The Grid Container
    let grid = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
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
                        template_columns: vec![
                            UTrackSize::Fr(1.5),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.5),
                        ],
                        template_rows: vec![
                            UTrackSize::Fr(1.0),
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

    let bg_color = Color::srgb(0.3, 0.3, 0.3); // Gray cards
    let border_color = Color::srgb(0.3, 0.75, 0.95); // Cyan lines
    let border_width = 2.0;
    let radius = 8.0;

    let mut spawn_item = |col_start: u32, col_span: u32, row_start: u32, row_span: u32| {
        commands.spawn((
            ChildOf(grid),
            USelf {
                item_ext: ULayoutItemExt {
                    grid: ULayoutGridItem {
                        column_start: Some(col_start),
                        column_span: col_span,
                        row_start: Some(row_start),
                        row_span,
                    },
                    ..default()
                },
                ..default()
            },
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                ..default()
            },
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
                offset: 0.0,
            },
        ));
    };

    // Recreate the layout from the image:
    // Box 1 (Top-Left, tall)
    spawn_item(1, 1, 1, 2);
    // Box 2 (Top-Center, wide)
    spawn_item(2, 2, 1, 1);
    // Box 3 (Top-Right, small)
    spawn_item(4, 1, 1, 1);

    // Box 4 (Mid-Left, small)
    spawn_item(1, 1, 3, 1);
    // Box 5 (Center, tall)
    spawn_item(2, 1, 2, 2);
    // Box 6 (Center-Right, small)
    spawn_item(3, 1, 2, 1);
    // Box 7 (Right, small)
    spawn_item(4, 1, 2, 1);

    // Box 8 (Bottom-Left, wide)
    spawn_item(1, 2, 4, 1);
    // Box 9 (Bottom-Right, wide & tall)
    spawn_item(3, 2, 3, 2);
}
