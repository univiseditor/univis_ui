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
                width: UVal::Px(1100.0),
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
        text("Flex layout showcase", 30.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(shell),
        label_node(),
        text(
            "This example covers alignment, direction changes, wrapping, and flex item growth.",
            16.0,
            Color::srgb(0.82, 0.87, 0.93),
        ),
    ));

    let content = commands
        .spawn((
            ChildOf(shell),
            UNode {
                width: UVal::Px(1052.0),
                height: UVal::Px(600.0),
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

    spawn_direction_panel(&mut commands, content);
    spawn_wrap_panel(&mut commands, content);
}

fn spawn_direction_panel(commands: &mut Commands, parent: Entity) {
    let panel = panel(commands, parent, 516.0, 600.0);

    commands.spawn((
        ChildOf(panel),
        label_node(),
        text("Row + column alignment", 22.0, Color::srgb(0.96, 0.98, 1.0)),
    ));
    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "The first container is a horizontal row. The second flips to `ColumnReverse`.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let row_demo = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(484.0),
                height: UVal::Px(190.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_flex_card(
        commands,
        row_demo,
        "A",
        "Start",
        Color::srgba(0.19, 0.34, 0.78, 0.96),
        UNode {
            width: UVal::Px(96.0),
            height: UVal::Px(96.0),
            ..default()
        },
        None,
    );
    spawn_flex_card(
        commands,
        row_demo,
        "B",
        "Center",
        Color::srgba(0.16, 0.57, 0.73, 0.96),
        UNode {
            width: UVal::Px(124.0),
            height: UVal::Px(74.0),
            ..default()
        },
        None,
    );
    spawn_flex_card(
        commands,
        row_demo,
        "C",
        "End",
        Color::srgba(0.18, 0.68, 0.48, 0.96),
        UNode {
            width: UVal::Px(110.0),
            height: UVal::Px(122.0),
            ..default()
        },
        None,
    );

    let column_demo = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(484.0),
                height: UVal::Px(290.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::ColumnReverse,
                justify_content: UJustifyContent::SpaceAround,
                align_items: UAlignItems::End,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    spawn_flex_card(
        commands,
        column_demo,
        "D",
        "ColumnReverse",
        Color::srgba(0.9, 0.65, 0.18, 0.96),
        UNode {
            width: UVal::Px(150.0),
            height: UVal::Px(64.0),
            ..default()
        },
        None,
    );
    spawn_flex_card(
        commands,
        column_demo,
        "E",
        "align self",
        Color::srgba(0.84, 0.41, 0.2, 0.96),
        UNode {
            width: UVal::Px(180.0),
            height: UVal::Px(82.0),
            ..default()
        },
        Some(USelf {
            item_ext: ULayoutItemExt {
                box_align: ULayoutBoxAlignSelf {
                    align_self: Some(UAlignSelfExt::Center),
                    ..default()
                },
                ..default()
            },
            ..default()
        }),
    );
    spawn_flex_card(
        commands,
        column_demo,
        "F",
        "cross end",
        Color::srgba(0.49, 0.22, 0.67, 0.96),
        UNode {
            width: UVal::Px(132.0),
            height: UVal::Px(70.0),
            ..default()
        },
        None,
    );
}

fn spawn_wrap_panel(commands: &mut Commands, parent: Entity) {
    let panel = panel(commands, parent, 516.0, 600.0);

    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "Wrap + flex item growth",
            22.0,
            Color::srgb(0.96, 0.98, 1.0),
        ),
    ));
    commands.spawn((
        ChildOf(panel),
        label_node(),
        text(
            "This container wraps items and lets one card expand with `flex_grow` and `flex_basis`.",
            15.0,
            Color::srgb(0.78, 0.84, 0.91),
        ),
    ));

    let wrap_demo = commands
        .spawn((
            ChildOf(panel),
            UNode {
                width: UVal::Px(484.0),
                height: UVal::Px(500.0),
                padding: USides::all(14.0),
                background_color: Color::srgba(0.08, 0.11, 0.15, 0.96),
                border_radius: UCornerRadius::all(18.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Stretch,
                justify_content: UJustifyContent::Start,
                container_ext: ULayoutContainerExt {
                    box_align: ULayoutBoxAlignContainer {
                        row_gap: Some(14.0),
                        column_gap: Some(12.0),
                        ..default()
                    },
                    flex: ULayoutFlexContainer {
                        wrap: UFlexWrap::Wrap,
                        align_content: Some(UContentAlignExt::SpaceAround),
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let cards = [
        (
            "1",
            "fixed",
            110.0,
            74.0,
            Color::srgba(0.2, 0.37, 0.82, 0.96),
        ),
        (
            "2",
            "fixed",
            124.0,
            58.0,
            Color::srgba(0.13, 0.58, 0.74, 0.96),
        ),
        (
            "3",
            "grow",
            136.0,
            90.0,
            Color::srgba(0.17, 0.68, 0.49, 0.96),
        ),
        (
            "4",
            "fixed",
            98.0,
            72.0,
            Color::srgba(0.9, 0.63, 0.18, 0.96),
        ),
        (
            "5",
            "fixed",
            140.0,
            62.0,
            Color::srgba(0.78, 0.27, 0.31, 0.96),
        ),
        (
            "6",
            "fixed",
            116.0,
            84.0,
            Color::srgba(0.52, 0.32, 0.82, 0.96),
        ),
        (
            "7",
            "fixed",
            100.0,
            66.0,
            Color::srgba(0.16, 0.7, 0.66, 0.96),
        ),
        (
            "8",
            "fixed",
            126.0,
            76.0,
            Color::srgba(0.86, 0.43, 0.19, 0.96),
        ),
    ];

    for (index, subtitle, width, height, color) in cards {
        let uself = if index == "3" {
            Some(USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_grow: Some(1.0),
                        flex_shrink: Some(1.0),
                        flex_basis: Some(UVal::Px(190.0)),
                    },
                    ..default()
                },
                ..default()
            })
        } else {
            None
        };

        spawn_flex_card(
            commands,
            wrap_demo,
            index,
            subtitle,
            color,
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(height),
                ..default()
            },
            uself,
        );
    }
}

fn panel(commands: &mut Commands, parent: Entity, width: f32, height: f32) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(height),
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
        .id()
}

fn spawn_flex_card(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    subtitle: &str,
    background_color: Color,
    node: UNode,
    uself: Option<USelf>,
) {
    let card = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: node.width,
                height: node.height,
                min_height: node.min_height,
                padding: USides::all(12.0),
                background_color,
                border_radius: UCornerRadius::all(16.0),
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

    if let Some(uself) = uself {
        commands.entity(card).insert(uself);
    }

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
