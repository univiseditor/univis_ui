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
                width: UVal::Px(1080.0),
                height: UVal::Px(720.0),
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
            "Grid tracks: `Px`, `Fr`, `Auto`",
            30.0,
            Color::srgb(0.96, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Each section below uses one of the available track definitions exposed by `UTrackSize`.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let sections = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(1030.0),
                height: UVal::Px(590.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 18.0,
                ..default()
            },
        ))
        .id();

    let px_grid = spawn_section(
        &mut commands,
        sections,
        "Fixed `Px` tracks",
        "Columns stay at exact authored sizes.",
    );
    commands.entity(px_grid).insert(ULayout {
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
                    UTrackSize::Px(90.0),
                    UTrackSize::Px(130.0),
                    UTrackSize::Px(170.0),
                    UTrackSize::Px(210.0),
                ],
                template_rows: vec![UTrackSize::Px(86.0)],
                ..default()
            },
            ..default()
        },
        ..default()
    });
    spawn_tile(
        &mut commands,
        px_grid,
        "Px 90",
        "first track",
        Color::srgba(0.2, 0.37, 0.82, 0.96),
    );
    spawn_tile(
        &mut commands,
        px_grid,
        "Px 130",
        "second track",
        Color::srgba(0.13, 0.58, 0.74, 0.96),
    );
    spawn_tile(
        &mut commands,
        px_grid,
        "Px 170",
        "third track",
        Color::srgba(0.17, 0.68, 0.49, 0.96),
    );
    spawn_tile(
        &mut commands,
        px_grid,
        "Px 210",
        "fourth track",
        Color::srgba(0.9, 0.63, 0.18, 0.96),
    );

    let fr_grid = spawn_section(
        &mut commands,
        sections,
        "Flexible `Fr` tracks",
        "Remaining width is distributed by authored fractions.",
    );
    commands.entity(fr_grid).insert(ULayout {
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
                    UTrackSize::Fr(1.0),
                    UTrackSize::Fr(2.0),
                    UTrackSize::Fr(1.0),
                ],
                template_rows: vec![UTrackSize::Px(86.0)],
                ..default()
            },
            ..default()
        },
        ..default()
    });
    spawn_tile(
        &mut commands,
        fr_grid,
        "1fr",
        "share 1",
        Color::srgba(0.24, 0.3, 0.8, 0.96),
    );
    spawn_tile(
        &mut commands,
        fr_grid,
        "2fr",
        "share 2",
        Color::srgba(0.75, 0.28, 0.38, 0.96),
    );
    spawn_tile(
        &mut commands,
        fr_grid,
        "1fr",
        "share 1",
        Color::srgba(0.18, 0.72, 0.54, 0.96),
    );

    let auto_grid = spawn_section(
        &mut commands,
        sections,
        "Implicit `Auto` tracks",
        "`Auto` is also available as a track definition and for implicit rows/columns.",
    );
    commands.entity(auto_grid).insert(ULayout {
        display: UDisplay::Grid,
        grid_columns: 3,
        container_ext: ULayoutContainerExt {
            box_align: ULayoutBoxAlignContainer {
                row_gap: Some(12.0),
                column_gap: Some(12.0),
                ..default()
            },
            grid: ULayoutGridContainer {
                template_columns: vec![UTrackSize::Auto, UTrackSize::Auto, UTrackSize::Auto],
                template_rows: vec![UTrackSize::Auto],
                auto_rows: UTrackSize::Auto,
                auto_columns: UTrackSize::Auto,
                ..default()
            },
            ..default()
        },
        ..default()
    });
    spawn_tile(
        &mut commands,
        auto_grid,
        "Auto A",
        "implicit",
        Color::srgba(0.21, 0.54, 0.88, 0.96),
    );
    spawn_tile(
        &mut commands,
        auto_grid,
        "Auto B",
        "implicit",
        Color::srgba(0.92, 0.58, 0.16, 0.96),
    );
    spawn_tile(
        &mut commands,
        auto_grid,
        "Auto C",
        "implicit",
        Color::srgba(0.52, 0.32, 0.82, 0.96),
    );
}

fn spawn_section(commands: &mut Commands, parent: Entity, title: &str, subtitle: &str) -> Entity {
    let section = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(1030.0),
                height: UVal::Px(176.0),
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
        ChildOf(section),
        label_node(),
        text(title, 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(section),
        label_node(),
        text(subtitle, 15.0, Color::srgb(0.78, 0.84, 0.91)),
    ));

    commands
        .spawn((
            ChildOf(section),
            UNode {
                width: UVal::Px(996.0),
                height: UVal::Px(96.0),
                background_color: Color::NONE,
                ..default()
            },
        ))
        .id()
}

fn spawn_tile(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
    background_color: Color,
) {
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
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((ChildOf(tile), label_node(), text(title, 22.0, Color::WHITE)));
    commands.spawn((
        ChildOf(tile),
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
