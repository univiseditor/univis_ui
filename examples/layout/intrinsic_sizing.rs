use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Intrinsic Sizing Showcase (Upward Pass)".to_string(),
                resolution: (1240, 780).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<ShowcaseState>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Resource)]
struct ShowcaseState {
    grid_extra_items: bool,
    grid_columns: u32,
    stack_wide_layer: bool,
}

impl Default for ShowcaseState {
    fn default() -> Self {
        Self {
            grid_extra_items: false,
            grid_columns: 2,
            stack_wide_layer: false,
        }
    }
}

#[derive(Component)]
struct GridContainer;

#[derive(Component)]
struct GridExtraItem;

#[derive(Component)]
struct StackWideLayer;

#[derive(Component)]
struct InfoLabel;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Screen-space root
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.04, 0.05, 0.08),
                padding: USides::all(24.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Start,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    // Header section
    let header = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(header),
        label_node(),
        text(
            "Univis UI - Intrinsic Sizing Showcase",
            26.0,
            Color::srgb(0.95, 0.97, 1.0),
        ),
    ));

    commands.spawn((
        ChildOf(header),
        label_node(),
        text(
            "Auto-sized containers (UVal::Auto) tightly wrap Grids, Stacks, and Flex without clipping or overflow.",
            14.0,
            Color::srgb(0.65, 0.72, 0.85),
        ),
    ));

    commands.spawn((
        ChildOf(header),
        InfoLabel,
        label_node(),
        text(
            "Press [Space]: Toggle 4/6 Items | [C]: Toggle 2/3 Columns | [S]: Toggle Wide Stack Layer",
            13.0,
            Color::srgb(0.35, 0.85, 0.95),
        ),
    ));

    // Main showcase row
    let panels_row = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Auto,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Start,
                gap: 28.0,
                ..default()
            },
        ))
        .id();

    // -------------------------------------------------------------
    // Card 1: Auto-Sized Grid Container
    // -------------------------------------------------------------
    let grid_card = commands
        .spawn((
            ChildOf(panels_row),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(20.0),
                background_color: Color::srgba(0.08, 0.12, 0.18, 0.95),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.7, 0.9, 0.5),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(grid_card),
        label_node(),
        text("UDisplay::Grid", 18.0, Color::srgb(0.3, 0.85, 1.0)),
    ));
    commands.spawn((
        ChildOf(grid_card),
        label_node(),
        text(
            "Intrinsic size based on cols, rows & gaps",
            12.0,
            Color::srgb(0.6, 0.7, 0.8),
        ),
    ));

    let grid_container = commands
        .spawn((
            ChildOf(grid_card),
            GridContainer,
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                background_color: Color::srgba(0.04, 0.07, 0.11, 0.8),
                padding: USides::all(10.0),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.7, 0.9, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 2,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    // 4 base items
    for i in 1..=4 {
        spawn_grid_item(&mut commands, grid_container, &format!("Card {i}"), false);
    }
    // 2 extra items (initially hidden via UDisplay::None)
    for i in 5..=6 {
        spawn_grid_item(&mut commands, grid_container, &format!("Card {i}"), true);
    }

    // -------------------------------------------------------------
    // Card 2: Auto-Sized Stack Container
    // -------------------------------------------------------------
    let stack_card = commands
        .spawn((
            ChildOf(panels_row),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(20.0),
                background_color: Color::srgba(0.14, 0.10, 0.20, 0.95),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.65, 0.4, 0.95, 0.5),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(stack_card),
        label_node(),
        text("UDisplay::Stack", 18.0, Color::srgb(0.8, 0.55, 1.0)),
    ));
    commands.spawn((
        ChildOf(stack_card),
        label_node(),
        text(
            "Intrinsic size = max(child dimensions)",
            12.0,
            Color::srgb(0.7, 0.65, 0.8),
        ),
    ));

    let stack_container = commands
        .spawn((
            ChildOf(stack_card),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                background_color: Color::srgba(0.08, 0.05, 0.12, 0.8),
                padding: USides::all(10.0),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.65, 0.4, 0.95, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Stack,
                ..default()
            },
        ))
        .id();

    // Base Layer: 170x170
    commands.spawn((
        ChildOf(stack_container),
        UNode {
            width: UVal::Px(170.0),
            height: UVal::Px(170.0),
            background_color: Color::srgba(0.22, 0.14, 0.32, 0.9),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(8.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Start,
            ..default()
        },
    ));

    // Middle Layer: 120x120
    commands.spawn((
        ChildOf(stack_container),
        UNode {
            width: UVal::Px(120.0),
            height: UVal::Px(120.0),
            background_color: Color::srgba(0.38, 0.20, 0.55, 0.85),
            border_radius: UCornerRadius::all(10.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            ..default()
        },
    ));

    // Top Layer: 70x70
    let top_layer = commands
        .spawn((
            ChildOf(stack_container),
            UNode {
                width: UVal::Px(70.0),
                height: UVal::Px(70.0),
                background_color: Color::srgba(0.65, 0.35, 0.95, 0.9),
                border_radius: UCornerRadius::all(8.0),
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

    commands.spawn((
        ChildOf(top_layer),
        label_node(),
        text("TOP", 12.0, Color::WHITE),
    ));

    // Extra Wide Layer (toggled with [S]): 230x90
    let wide_layer = commands
        .spawn((
            ChildOf(stack_container),
            StackWideLayer,
            UNode {
                width: UVal::Px(230.0),
                height: UVal::Px(90.0),
                background_color: Color::srgba(0.85, 0.30, 0.60, 0.85),
                border_radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::None, // initially hidden
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(wide_layer),
        label_node(),
        text("Wide Layer (230x90)", 13.0, Color::WHITE),
    ));

    // -------------------------------------------------------------
    // Card 3: Auto-Sized Flex Container
    // -------------------------------------------------------------
    let flex_card = commands
        .spawn((
            ChildOf(panels_row),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                padding: USides::all(20.0),
                background_color: Color::srgba(0.08, 0.16, 0.14, 0.95),
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.25, 0.85, 0.60, 0.5),
                width: 1.5,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(flex_card),
        label_node(),
        text("UDisplay::Flex", 18.0, Color::srgb(0.4, 0.95, 0.7)),
    ));
    commands.spawn((
        ChildOf(flex_card),
        label_node(),
        text(
            "Intrinsic size = sum(main) & max(cross)",
            12.0,
            Color::srgb(0.6, 0.8, 0.7),
        ),
    ));

    let flex_container = commands
        .spawn((
            ChildOf(flex_card),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                background_color: Color::srgba(0.04, 0.09, 0.07, 0.8),
                padding: USides::all(10.0),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            UBorder {
                color: Color::srgba(0.25, 0.85, 0.60, 0.2),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    for (name, w, h) in [
        ("Flex Item Alpha", 150.0, 36.0),
        ("Flex Item Beta (Longer)", 200.0, 36.0),
        ("Flex Item Gamma", 130.0, 36.0),
    ] {
        let item = commands
            .spawn((
                ChildOf(flex_container),
                UNode {
                    width: UVal::Px(w),
                    height: UVal::Px(h),
                    background_color: Color::srgba(0.12, 0.28, 0.22, 0.9),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(6.0),
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

        commands.spawn((ChildOf(item), label_node(), text(name, 12.0, Color::WHITE)));
    }

    // Footer note
    commands.spawn((
        ChildOf(root),
        label_node(),
        text(
            "Powered by Univis UI's bottom-up measure pass with zero heap reallocations.",
            12.0,
            Color::srgb(0.45, 0.50, 0.60),
        ),
    ));
}

fn spawn_grid_item(commands: &mut Commands, parent: Entity, label: &str, is_extra: bool) {
    let display = if is_extra {
        UDisplay::None
    } else {
        UDisplay::Flex
    };

    let mut entity_cmds = commands.spawn((
        ChildOf(parent),
        UNode {
            width: UVal::Px(100.0),
            height: UVal::Px(64.0),
            background_color: Color::srgba(0.15, 0.24, 0.35, 0.9),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(6.0),
            ..default()
        },
        UBorder {
            color: Color::srgba(0.3, 0.7, 0.95, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
            offset: 0.0,
        },
        ULayout {
            display,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            ..default()
        },
    ));

    if is_extra {
        entity_cmds.insert(GridExtraItem);
    }

    let item_id = entity_cmds.id();
    commands.spawn((
        ChildOf(item_id),
        label_node(),
        text(label, 13.0, Color::WHITE),
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ShowcaseState>,
    mut q_grid: Query<
        &mut ULayout,
        (
            With<GridContainer>,
            Without<GridExtraItem>,
            Without<StackWideLayer>,
        ),
    >,
    mut q_extra_items: Query<
        &mut ULayout,
        (
            With<GridExtraItem>,
            Without<GridContainer>,
            Without<StackWideLayer>,
        ),
    >,
    mut q_wide_layer: Query<
        &mut ULayout,
        (
            With<StackWideLayer>,
            Without<GridContainer>,
            Without<GridExtraItem>,
        ),
    >,
    mut q_info: Query<&mut UTextLabel, With<InfoLabel>>,
) {
    let mut changed = false;

    if keyboard.just_pressed(KeyCode::Space) {
        state.grid_extra_items = !state.grid_extra_items;
        changed = true;
        let new_display = if state.grid_extra_items {
            UDisplay::Flex
        } else {
            UDisplay::None
        };
        for mut layout in &mut q_extra_items {
            layout.display = new_display;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyC) {
        state.grid_columns = if state.grid_columns == 2 { 3 } else { 2 };
        changed = true;
        for mut layout in &mut q_grid {
            layout.grid_columns = state.grid_columns;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        state.stack_wide_layer = !state.stack_wide_layer;
        changed = true;
        let new_display = if state.stack_wide_layer {
            UDisplay::Flex
        } else {
            UDisplay::None
        };
        for mut layout in &mut q_wide_layer {
            layout.display = new_display;
        }
    }

    if changed {
        let items_str = if state.grid_extra_items {
            "6 Items"
        } else {
            "4 Items"
        };
        let cols_str = format!("{} Cols", state.grid_columns);
        let stack_str = if state.stack_wide_layer {
            "Wide Active"
        } else {
            "Normal"
        };

        for mut label in &mut q_info {
            label.text = format!(
                "Grid: [{items_str}, {cols_str}] | Stack: [{stack_str}]  ([Space]: Items | [C]: Cols | [S]: Stack)",
            );
        }
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
