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
                width: UVal::Px(1020.0),
                height: UVal::Px(640.0),
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
            "Grid auto-placement: `Row` vs `Column`",
            30.0,
            Color::srgb(0.96, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Both grids use the same seven items. Only `auto_flow` changes.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let row = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(972.0),
                height: UVal::Px(520.0),
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

    let row_grid = spawn_flow_panel(
        &mut commands,
        row,
        "Row flow",
        "Fills left-to-right, then creates implicit rows.",
        UGridAutoFlow::Row,
    );
    let column_grid = spawn_flow_panel(
        &mut commands,
        row,
        "Column flow",
        "Fills top-to-bottom, then creates implicit columns.",
        UGridAutoFlow::Column,
    );

    let colors = [
        Color::srgba(0.19, 0.34, 0.78, 0.96),
        Color::srgba(0.16, 0.57, 0.73, 0.96),
        Color::srgba(0.18, 0.68, 0.48, 0.96),
        Color::srgba(0.42, 0.7, 0.25, 0.96),
        Color::srgba(0.9, 0.65, 0.18, 0.96),
        Color::srgba(0.86, 0.43, 0.19, 0.96),
        Color::srgba(0.77, 0.27, 0.31, 0.96),
    ];

    for (index, color) in colors.into_iter().enumerate() {
        let label = (index + 1).to_string();
        spawn_tile(&mut commands, row_grid, &label, color);
        spawn_tile(&mut commands, column_grid, &label, color);
    }
}

fn spawn_flow_panel(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
    flow: UGridAutoFlow,
) -> Entity {
    let panel = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(477.0),
                height: UVal::Px(520.0),
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
        text(subtitle, 15.0, Color::srgb(0.78, 0.84, 0.91)),
    ));

    commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(443.0),
                height: UVal::Px(430.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
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
                        template_columns: vec![
                            UTrackSize::Px(124.0),
                            UTrackSize::Px(124.0),
                            UTrackSize::Px(124.0),
                        ],
                        template_rows: vec![UTrackSize::Px(84.0), UTrackSize::Px(84.0)],
                        auto_flow: flow,
                        auto_rows: UTrackSize::Px(84.0),
                        auto_columns: UTrackSize::Px(124.0),
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

fn spawn_tile(commands: &mut Commands, parent: Entity, title: &str, background_color: Color) {
    let tile = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Auto,
                height: UVal::Auto,
                min_height: 84.0,
                padding: USides::all(14.0),
                background_color,
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

    commands.spawn((ChildOf(tile), label_node(), text(title, 26.0, Color::WHITE)));
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
