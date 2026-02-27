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

    commands
        .spawn((
            UScreenRoot,
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.08, 0.09, 0.12),
                padding: USides::all(24.0),
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
                text: "Layout Case: Safe vs Unsafe Overflow Alignment".to_string(),
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            });

            root.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 16.0,
                    ..default()
                },
            ))
            .with_children(|row| {
                for (title, overflow_mode, color) in [
                    (
                        "Safe Overflow",
                        UOverflowPosition::Safe,
                        Color::srgb(0.28, 0.64, 0.88),
                    ),
                    (
                        "Unsafe Overflow",
                        UOverflowPosition::Unsafe,
                        Color::srgb(0.86, 0.42, 0.32),
                    ),
                ] {
                    row.spawn((
                        UNode {
                            width: UVal::Flex(1.0),
                            height: UVal::Percent(1.0),
                            padding: USides::all(10.0),
                            background_color: Color::srgb(0.12, 0.14, 0.18),
                            border_radius: UCornerRadius::all(12.0),
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 10.0,
                            ..default()
                        },
                    ))
                    .with_children(|panel| {
                        panel.spawn(UTextLabel {
                            text: title.to_string(),
                            font_size: 22.0,
                            color: Color::WHITE,
                            ..default()
                        });

                        panel.spawn((
                            UNode {
                                width: UVal::Percent(1.0),
                                height: UVal::Flex(1.0),
                                background_color: Color::srgb(0.09, 0.11, 0.15),
                                border_radius: UCornerRadius::all(10.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Grid,
                                grid_columns: 1,
                                align_items: UAlignItems::Center,
                                container_ext: ULayoutContainerExt {
                                    box_align: ULayoutBoxAlignContainer {
                                        justify_items: Some(UAlignItemsExt::Center),
                                        ..default()
                                    },
                                    grid: ULayoutGridContainer {
                                        template_columns: vec![UTrackSize::Px(140.0)],
                                        template_rows: vec![UTrackSize::Px(100.0)],
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|grid| {
                            grid.spawn((
                                UNode {
                                    width: UVal::Px(220.0),
                                    height: UVal::Px(60.0),
                                    background_color: color,
                                    border_radius: UCornerRadius::all(8.0),
                                    ..default()
                                },
                                USelf {
                                    item_ext: ULayoutItemExt {
                                        box_align: ULayoutBoxAlignSelf {
                                            justify_self: Some(UAlignSelfExt::Center),
                                            align_self: Some(UAlignSelfExt::Center),
                                            justify_overflow: overflow_mode,
                                            align_overflow: overflow_mode,
                                        },
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                    });
                }
            });
        });
}
