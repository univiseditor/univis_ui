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
                width: UVal::Px(1160.0),
                height: UVal::Px(780.0),
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
        text("Widgets: containers", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "Resizable panels and scroll containers. Hover the list on the right and use the mouse wheel to scroll.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let content = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(1112.0),
                height: UVal::Px(660.0),
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

    let left = commands
        .spawn((
            ChildOf(content),
            UPanel::glass().with_gap(12.0),
            UPanelWindow::default().with_min_size(280.0, 220.0),
            UNode {
                width: UVal::Px(520.0),
                height: UVal::Px(660.0),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(left),
        label_node(),
        text("Resizable panel window", 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(left),
        label_node(),
        text(
            "Drag any border or corner handle to resize this panel. The resize chrome is invisible until hover.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));
    commands.spawn((ChildOf(left), UDivider::horizontal().with_thickness(2.0)));

    let tool_grid = commands
        .spawn((
            ChildOf(left),
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 2,
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

    for (title, color) in [
        ("Scene", Color::srgba(0.2, 0.37, 0.82, 0.26)),
        ("Assets", Color::srgba(0.18, 0.68, 0.48, 0.26)),
        ("Logs", Color::srgba(0.92, 0.58, 0.18, 0.26)),
        ("Profiler", Color::srgba(0.77, 0.27, 0.31, 0.26)),
    ] {
        let card = commands
            .spawn((
                ChildOf(tool_grid),
                UNode {
                    min_height: 120.0,
                    padding: USides::all(14.0),
                    background_color: color,
                    border_radius: UCornerRadius::all(16.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((ChildOf(card), label_node(), text(title, 18.0, Color::WHITE)));
        commands.spawn((
            ChildOf(card),
            label_node(),
            text(
                "Panel content placeholder",
                14.0,
                Color::srgb(0.86, 0.9, 0.96),
            ),
        ));
    }

    let right = commands
        .spawn((
            ChildOf(content),
            UPanel::glass().with_gap(12.0),
            UNode {
                width: UVal::Px(574.0),
                height: UVal::Px(660.0),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(right),
        label_node(),
        text("Scroll container", 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(right),
        label_node(),
        text(
            "The viewport below owns `UClip`, `UScrollContainer`, and `UInteraction`. Its first child is the scroll content root.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));
    commands.spawn((ChildOf(right), UDivider::horizontal().with_thickness(2.0)));

    let viewport = commands
        .spawn((
            ChildOf(right),
            UClip::enabled(true),
            UScrollContainer::new(),
            UInteraction::default(),
            UNode {
                width: UVal::Px(542.0),
                height: UVal::Px(540.0),
                padding: USides::all(12.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                ..default()
            },
        ))
        .id();

    let list = commands
        .spawn((
            ChildOf(viewport),
            USelf::default(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Content,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    for index in 1..=18 {
        let card = commands
            .spawn((
                ChildOf(list),
                UNode {
                    width: UVal::Percent(1.0),
                    min_height: 70.0,
                    padding: USides::all(14.0),
                    background_color: Color::srgba(0.12, 0.15, 0.2, 0.7),
                    border_radius: UCornerRadius::all(14.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(card),
            label_node(),
            text(&format!("Entry #{index}"), 16.0, Color::WHITE),
        ));
        commands.spawn((
            ChildOf(card),
            label_node(),
            text(
                "Hover the viewport and scroll the mouse wheel to move through the list.",
                13.0,
                Color::srgb(0.82, 0.87, 0.93),
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
