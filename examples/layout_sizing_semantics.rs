use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
#[allow(unused_imports)]
use univis_ui_interaction::prelude::*;
#[allow(unused_imports)]
use univis_ui_style::prelude::*;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            UnivisUiStylePlugin,
            UnivisEnginePlugin,
            UnivisInteractionPlugin,
            UnivisWidgetPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.06, 0.08, 0.11),
                padding: USides::all(22.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel {
                text: "Layout Case: Sizing Semantics".into(),
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            });

            root.spawn(UTextLabel {
                text: "Shows how Auto, MaxContent, MinContent, and min/max constraints diverge in real layout.".into(),
                font_size: 16.0,
                color: Color::srgb(0.78, 0.83, 0.9),
                ..default()
            });

            root.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(280.0),
                    background_color: Color::srgb(0.1, 0.12, 0.18),
                    border_radius: UCornerRadius::all(16.0),
                    padding: USides::all(14.0),
                    ..default()
                },
                UBorder {
                    width: 1.0,
                    color: Color::srgba(0.95, 0.98, 1.0, 0.16),
                    radius: UCornerRadius::all(16.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 14.0,
                    align_items: UAlignItems::Stretch,
                    ..default()
                },
            ))
            .with_children(|row| {
                spawn_auto_vs_content_card(row);
                spawn_grid_semantics_card(row);
            });

            root.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(252.0),
                    background_color: Color::srgb(0.09, 0.11, 0.16),
                    border_radius: UCornerRadius::all(16.0),
                    padding: USides::all(14.0),
                    ..default()
                },
                UBorder {
                    width: 1.0,
                    color: Color::srgba(0.95, 0.98, 1.0, 0.12),
                    radius: UCornerRadius::all(16.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 12.0,
                    ..default()
                },
            ))
            .with_children(|column| {
                spawn_section_header(
                    column,
                    "Flex redistribution respects min/max",
                    "The first row shows shrink stopping at min_width. The second row shows grow stopping at max_width.",
                );
                spawn_shrink_demo(column);
                spawn_grow_demo(column);
            });
        });
}

fn spawn_auto_vs_content_card(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.13, 0.15, 0.22),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .with_children(|card| {
            spawn_section_header(
                card,
                "Auto vs MaxContent in stretch",
                "Only the Auto sample stretches to the full row height by default.",
            );

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    background_color: Color::srgb(0.08, 0.1, 0.14),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Stretch,
                    gap: 10.0,
                    ..default()
                },
            ))
            .with_children(|demo| {
                spawn_labeled_sample(
                    demo,
                    "Auto",
                    UNode {
                        width: UVal::Flex(1.0),
                        height: UVal::Auto,
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.22, 0.52, 0.88),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                );
                spawn_labeled_sample(
                    demo,
                    "MaxContent",
                    UNode {
                        width: UVal::Flex(1.0),
                        height: UVal::MaxContent,
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.55, 0.34, 0.25),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                );
            });
        });
}

fn spawn_grid_semantics_card(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.13, 0.15, 0.22),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .with_children(|card| {
            spawn_section_header(
                card,
                "Grid default stretch",
                "Auto fills the grid cell. MaxContent keeps its measured size in the top-left corner.",
            );

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    background_color: Color::srgb(0.08, 0.1, 0.14),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Grid,
                    grid_columns: 2,
                    gap: 12.0,
                    align_items: UAlignItems::Start,
                    container_ext: ULayoutContainerExt {
                        grid: ULayoutGridContainer {
                            template_columns: vec![UTrackSize::Fr(1.0), UTrackSize::Fr(1.0)],
                            template_rows: vec![UTrackSize::Px(116.0)],
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|grid| {
                spawn_labeled_sample(
                    grid,
                    "Auto cell",
                    UNode {
                        width: UVal::Auto,
                        height: UVal::Auto,
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.21, 0.63, 0.47),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                );
                spawn_labeled_sample(
                    grid,
                    "MaxContent cell",
                    UNode {
                        width: UVal::MaxContent,
                        height: UVal::MaxContent,
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.66, 0.49, 0.2),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                );
            });
        });
}

fn spawn_shrink_demo(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(78.0),
                background_color: Color::srgb(0.07, 0.09, 0.13),
                border_radius: UCornerRadius::all(12.0),
                padding: USides::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 10.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|row| {
            spawn_bar(
                row,
                "Shrink floor 180",
                Color::srgb(0.27, 0.56, 0.86),
                UNode {
                    width: UVal::Px(220.0),
                    height: UVal::Px(52.0),
                    min_width: 180.0,
                    background_color: Color::srgb(0.27, 0.56, 0.86),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
            );

            spawn_bar(
                row,
                "Absorbs overflow",
                Color::srgb(0.69, 0.38, 0.29),
                UNode {
                    width: UVal::Px(220.0),
                    height: UVal::Px(52.0),
                    background_color: Color::srgb(0.69, 0.38, 0.29),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
            );
        });
}

fn spawn_grow_demo(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(78.0),
                background_color: Color::srgb(0.07, 0.09, 0.13),
                border_radius: UCornerRadius::all(12.0),
                padding: USides::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 10.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|row| {
            spawn_grow_bar(
                row,
                "Stops at max 110",
                UNode {
                    width: UVal::Px(70.0),
                    height: UVal::Px(52.0),
                    max_width: 110.0,
                    background_color: Color::srgb(0.21, 0.63, 0.47),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
            );

            spawn_grow_bar(
                row,
                "Gets the rest",
                UNode {
                    width: UVal::Px(70.0),
                    height: UVal::Px(52.0),
                    background_color: Color::srgb(0.62, 0.44, 0.81),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
            );
        });
}

fn spawn_section_header(parent: &mut ChildSpawnerCommands, title: &str, body: &str) {
    parent.spawn(UTextLabel {
        text: title.into(),
        font_size: 18.0,
        color: Color::WHITE,
        ..default()
    });
    parent.spawn(UTextLabel {
        text: body.into(),
        font_size: 13.0,
        color: Color::srgb(0.76, 0.81, 0.9),
        ..default()
    });
}

fn spawn_labeled_sample(parent: &mut ChildSpawnerCommands, title: &str, node: UNode) {
    parent
        .spawn((
            node,
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 8.0,
                ..default()
            },
        ))
        .with_children(|sample| {
            sample.spawn(UTextLabel {
                text: title.into(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            sample.spawn(UTextLabel {
                text: "Context decides here.".into(),
                font_size: 12.0,
                color: Color::srgba(1.0, 1.0, 1.0, 0.85),
                ..default()
            });
        });
}

fn spawn_bar(parent: &mut ChildSpawnerCommands, title: &str, accent: Color, node: UNode) {
    parent
        .spawn((
            node,
            UBorder {
                width: 1.0,
                color: accent.with_alpha(0.35),
                radius: UCornerRadius::all(10.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_shrink: Some(1.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|bar| {
            bar.spawn(UTextLabel {
                text: title.into(),
                font_size: 13.0,
                color: Color::WHITE,
                ..default()
            });
        });
}

fn spawn_grow_bar(parent: &mut ChildSpawnerCommands, title: &str, node: UNode) {
    parent
        .spawn((
            node,
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_grow: Some(1.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|bar| {
            bar.spawn(UTextLabel {
                text: title.into(),
                font_size: 13.0,
                color: Color::WHITE,
                ..default()
            });
        });
}
