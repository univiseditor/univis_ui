use bevy::prelude::*;
use univis_ui_engine::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisEnginePlugin)
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
                padding: USides::all(22.0),
                background_color: Color::srgb(0.03, 0.05, 0.09),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                UNode {
                    width: UVal::Px(1560.0),
                    height: UVal::Px(920.0),
                    padding: USides::all(14.0),
                    background_color: Color::srgb(0.08, 0.11, 0.17),
                    border_radius: UCornerRadius::all(16.0),
                    ..default()
                },
                UBorder {
                    color: Color::srgba(0.9, 0.95, 1.0, 0.35),
                    width: 1.0,
                    offset: 0.0,
                    radius: UCornerRadius::all(16.0),
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 14.0,
                    ..default()
                },
            ))
            .with_children(|frame| {
                spawn_top_stage(frame);
                spawn_bottom_grid_strip(frame);
            });
        });
}

fn spawn_top_stage(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Flex(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.11, 0.14, 0.21),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 12.0,
                align_items: UAlignItems::Stretch,
                ..default()
            },
        ))
        .with_children(|top| {
            spawn_left_column(top);
            spawn_center_column(top);
            spawn_right_column(top);
        });
}

fn spawn_left_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.35),
                height: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.1, 0.12, 0.19),
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
        .with_children(|left| {
            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(230.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgb(0.13, 0.16, 0.25),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Grid,
                    grid_columns: 6,
                    gap: 8.0,
                    container_ext: ULayoutContainerExt {
                        grid: ULayoutGridContainer {
                            template_columns: vec![
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                                UTrackSize::Fr(1.0),
                            ],
                            template_rows: vec![
                                UTrackSize::Px(56.0),
                                UTrackSize::Px(56.0),
                                UTrackSize::Px(56.0),
                            ],
                            auto_rows: UTrackSize::Px(54.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|grid| {
                spawn_grid_tile(grid, Color::srgb(0.22, 0.56, 0.9), None, 3, None, 2);
                spawn_grid_tile(grid, Color::srgb(0.78, 0.35, 0.32), None, 2, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.25, 0.75, 0.44), None, 1, None, 2);
                spawn_grid_tile(grid, Color::srgb(0.9, 0.68, 0.26), Some(1), 2, Some(3), 1);
                spawn_grid_tile(grid, Color::srgb(0.46, 0.52, 0.9), None, 2, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.26, 0.77, 0.8), None, 2, None, 1);
                spawn_grid_tile(grid, Color::srgb(0.56, 0.37, 0.76), Some(5), 2, Some(3), 1);
                spawn_grid_tile(grid, Color::srgb(0.34, 0.69, 0.89), None, 1, None, 1);
            });

            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Flex(1.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgb(0.12, 0.16, 0.24),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Masonry,
                    grid_columns: 4,
                    container_ext: ULayoutContainerExt {
                        box_align: ULayoutBoxAlignContainer {
                            row_gap: Some(8.0),
                            column_gap: Some(8.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|masonry| {
                let heights = [
                    58.0, 90.0, 66.0, 108.0, 52.0, 82.0, 64.0, 96.0, 72.0, 60.0, 88.0, 54.0,
                    102.0, 70.0, 80.0, 62.0,
                ];
                for (i, h) in heights.iter().enumerate() {
                    masonry.spawn(UNode {
                        width: UVal::Auto,
                        height: UVal::Px(*h),
                        background_color: Color::srgb(
                            0.26 + (i % 4) as f32 * 0.08,
                            0.36 + (i % 3) as f32 * 0.05,
                            0.5 + (i % 2) as f32 * 0.05,
                        ),
                        border_radius: UCornerRadius::all(8.0),
                        ..default()
                    });
                }
            });

            left.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(140.0),
                    padding: USides::all(10.0),
                    background_color: Color::srgb(0.15, 0.18, 0.27),
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                    ..default()
                },
            ))
            .with_children(|row| {
                for i in 0..5 {
                    row.spawn((
                        UNode {
                            width: UVal::Flex(1.0),
                            height: UVal::Px(86.0),
                            background_color: Color::srgb(
                                0.3 + i as f32 * 0.04,
                                0.37 + i as f32 * 0.03,
                                0.53 + i as f32 * 0.02,
                            ),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        flex_grow_item(),
                    ));
                }
            });
        });
}

fn spawn_center_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(0.95),
                height: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.13, 0.15, 0.23),
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
        .with_children(|center| {
            center
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(268.0),
                        padding: USides::all(12.0),
                        background_color: Color::srgb(0.19, 0.22, 0.36),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Stack,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|stack| {
                    stack.spawn(UNode {
                        width: UVal::Px(290.0),
                        height: UVal::Px(180.0),
                        background_color: Color::srgba(0.2, 0.52, 0.9, 0.82),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    });
                    stack.spawn(UNode {
                        width: UVal::Px(248.0),
                        height: UVal::Px(154.0),
                        background_color: Color::srgba(0.86, 0.63, 0.24, 0.84),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    });
                    stack.spawn(UNode {
                        width: UVal::Px(210.0),
                        height: UVal::Px(126.0),
                        background_color: Color::srgba(0.24, 0.78, 0.46, 0.84),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    });
                    stack.spawn(UNode {
                        width: UVal::Px(156.0),
                        height: UVal::Px(94.0),
                        background_color: Color::srgba(0.92, 0.35, 0.42, 0.86),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    });
                    stack.spawn(UNode {
                        width: UVal::Px(114.0),
                        height: UVal::Px(68.0),
                        background_color: Color::srgba(0.76, 0.85, 0.98, 0.9),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    });
                });

            center
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        padding: USides::all(8.0),
                        background_color: Color::srgb(0.14, 0.18, 0.27),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Radial,
                        ..default()
                    },
                ))
                .with_children(|radial| {
                    for i in 0..15 {
                        radial.spawn(UNode {
                            width: UVal::Px(34.0 + (i % 3) as f32 * 10.0),
                            height: UVal::Px(34.0 + (i % 4) as f32 * 8.0),
                            background_color: Color::srgb(
                                0.24 + i as f32 * 0.03,
                                0.7 - i as f32 * 0.02,
                                0.9 - i as f32 * 0.015,
                            ),
                            border_radius: UCornerRadius::all(10.0),
                            ..default()
                        });
                    }
                });

            center
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(190.0),
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.17, 0.2, 0.3),
                        border_radius: UCornerRadius::all(10.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 8.0,
                        ..default()
                    },
                ))
                .with_children(|timeline| {
                    for i in 0..5 {
                        timeline
                            .spawn((
                                UNode {
                                    width: UVal::Percent(1.0),
                                    height: UVal::Px(28.0),
                                    padding: USides::axes(8.0, 6.0),
                                    background_color: Color::srgba(0.11, 0.13, 0.2, 0.65),
                                    border_radius: UCornerRadius::all(7.0),
                                    ..default()
                                },
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 8.0,
                                    ..default()
                                },
                            ))
                            .with_children(|row| {
                                row.spawn(UNode {
                                    width: UVal::Px(36.0),
                                    height: UVal::Px(14.0),
                                    background_color: Color::srgb(0.38 + i as f32 * 0.06, 0.7, 0.95),
                                    border_radius: UCornerRadius::all(6.0),
                                    ..default()
                                });
                                row.spawn((
                                    UNode {
                                        width: UVal::Px(180.0),
                                        height: UVal::Px(12.0),
                                        background_color: Color::srgba(0.85, 0.9, 1.0, 0.66),
                                        border_radius: UCornerRadius::all(6.0),
                                        ..default()
                                    },
                                ));
                                row.spawn(UNode {
                                    width: UVal::Px(24.0),
                                    height: UVal::Px(12.0),
                                    background_color: Color::srgb(0.94, 0.58, 0.3),
                                    border_radius: UCornerRadius::all(6.0),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

fn spawn_right_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(0.95),
                height: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.12, 0.14, 0.22),
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
        .with_children(|right| {
            right
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(290.0),
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.16, 0.18, 0.3),
                        border_radius: UCornerRadius::all(10.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Grid,
                        grid_columns: 3,
                        gap: 8.0,
                        container_ext: ULayoutContainerExt {
                            box_align: ULayoutBoxAlignContainer {
                                justify_items: Some(UAlignItemsExt::Center),
                                row_gap: Some(8.0),
                                column_gap: Some(8.0),
                                ..default()
                            },
                            grid: ULayoutGridContainer {
                                template_columns: vec![
                                    UTrackSize::Fr(1.0),
                                    UTrackSize::Fr(1.0),
                                    UTrackSize::Fr(1.0),
                                ],
                                template_rows: vec![
                                    UTrackSize::Px(64.0),
                                    UTrackSize::Px(64.0),
                                    UTrackSize::Px(64.0),
                                ],
                                auto_flow: UGridAutoFlow::Column,
                                auto_rows: UTrackSize::Px(56.0),
                                auto_columns: UTrackSize::Fr(1.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|grid| {
                    spawn_grid_tile(grid, Color::srgb(0.22, 0.62, 0.98), None, 2, None, 1);
                    spawn_grid_tile(grid, Color::srgb(0.85, 0.4, 0.34), None, 1, None, 2);
                    spawn_grid_tile(grid, Color::srgb(0.34, 0.76, 0.42), None, 1, None, 1);
                    spawn_grid_tile(grid, Color::srgb(0.86, 0.72, 0.26), None, 2, None, 1);
                    spawn_grid_tile(grid, Color::srgb(0.47, 0.54, 0.9), None, 1, None, 1);
                    spawn_grid_tile(grid, Color::srgb(0.25, 0.78, 0.82), None, 1, None, 2);
                    spawn_grid_tile(grid, Color::srgb(0.64, 0.44, 0.86), None, 2, None, 1);
                    spawn_grid_tile(grid, Color::srgb(0.92, 0.52, 0.58), None, 1, None, 1);
                });

            right
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.18, 0.2, 0.31),
                        border_radius: UCornerRadius::all(10.0),
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
                    panel
                        .spawn((
                            UNode {
                                width: UVal::Percent(1.0),
                                height: UVal::Px(130.0),
                                padding: USides::all(8.0),
                                background_color: Color::srgb(0.1, 0.12, 0.2),
                                border_radius: UCornerRadius::all(8.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                gap: 8.0,
                                ..default()
                            },
                        ))
                        .with_children(|row| {
                            row.spawn((
                                UNode {
                                    width: UVal::Flex(1.0),
                                    height: UVal::Percent(1.0),
                                    background_color: Color::srgb(0.31, 0.4, 0.67),
                                    border_radius: UCornerRadius::all(8.0),
                                    ..default()
                                },
                                flex_grow_item(),
                            ));
                            row.spawn((
                                UNode {
                                    width: UVal::Flex(1.0),
                                    height: UVal::Percent(1.0),
                                    background_color: Color::srgb(0.38, 0.49, 0.78),
                                    border_radius: UCornerRadius::all(8.0),
                                    ..default()
                                },
                                flex_grow_item(),
                            ));
                            row.spawn((
                                UNode {
                                    width: UVal::Flex(1.0),
                                    height: UVal::Percent(1.0),
                                    background_color: Color::srgb(0.45, 0.58, 0.84),
                                    border_radius: UCornerRadius::all(8.0),
                                    ..default()
                                },
                                flex_grow_item(),
                            ));
                        });

                    panel
                        .spawn((
                            UNode {
                                width: UVal::Percent(1.0),
                                height: UVal::Flex(1.0),
                                padding: USides::all(8.0),
                                background_color: Color::srgb(0.12, 0.16, 0.25),
                                border_radius: UCornerRadius::all(8.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Grid,
                                grid_columns: 4,
                                gap: 8.0,
                                container_ext: ULayoutContainerExt {
                                    grid: ULayoutGridContainer {
                                        template_columns: vec![
                                            UTrackSize::Fr(1.0),
                                            UTrackSize::Fr(1.0),
                                            UTrackSize::Fr(1.0),
                                            UTrackSize::Fr(1.0),
                                        ],
                                        auto_rows: UTrackSize::Px(44.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|grid| {
                            spawn_grid_tile(grid, Color::srgb(0.24, 0.64, 0.97), None, 2, None, 2);
                            spawn_grid_tile(grid, Color::srgb(0.84, 0.36, 0.32), None, 2, None, 1);
                            spawn_grid_tile(grid, Color::srgb(0.35, 0.77, 0.44), None, 1, None, 2);
                            spawn_grid_tile(grid, Color::srgb(0.9, 0.66, 0.23), None, 1, None, 1);
                            spawn_grid_tile(grid, Color::srgb(0.57, 0.42, 0.82), None, 2, None, 1);
                            spawn_grid_tile(grid, Color::srgb(0.24, 0.79, 0.8), None, 1, None, 1);
                        });

                    panel.spawn((
                        UNode {
                            width: UVal::Px(16.0),
                            height: UVal::Px(16.0),
                            background_color: Color::srgb(0.96, 0.35, 0.36),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        USelf {
                            position_type: UPositionType::Absolute,
                            right: UVal::Px(10.0),
                            top: UVal::Px(10.0),
                            ..default()
                        },
                    ));

                    panel.spawn((
                        UNode {
                            width: UVal::Px(16.0),
                            height: UVal::Px(16.0),
                            background_color: Color::srgb(0.34, 0.95, 0.65),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        USelf {
                            position_type: UPositionType::Absolute,
                            left: UVal::Px(10.0),
                            bottom: UVal::Px(10.0),
                            ..default()
                        },
                    ));
                });

            right
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(132.0),
                        padding: USides::all(10.0),
                        background_color: Color::srgb(0.14, 0.17, 0.26),
                        border_radius: UCornerRadius::all(10.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        gap: 8.0,
                        ..default()
                    },
                ))
                .with_children(|actions| {
                    for i in 0..3 {
                        actions.spawn((
                            UNode {
                                width: UVal::Flex(1.0),
                                height: UVal::Percent(1.0),
                                background_color: Color::srgb(
                                    0.25 + i as f32 * 0.12,
                                    0.5 - i as f32 * 0.06,
                                    0.9 - i as f32 * 0.1,
                                ),
                                border_radius: UCornerRadius::all(8.0),
                                ..default()
                            },
                            flex_grow_item(),
                        ));
                    }
                });
        });
}

fn spawn_bottom_grid_strip(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(178.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.12, 0.15, 0.22),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Grid,
                grid_columns: 12,
                gap: 8.0,
                container_ext: ULayoutContainerExt {
                    grid: ULayoutGridContainer {
                        template_columns: vec![
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                            UTrackSize::Fr(1.0),
                        ],
                        template_rows: vec![UTrackSize::Px(72.0), UTrackSize::Px(72.0)],
                        auto_rows: UTrackSize::Px(68.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|grid| {
            spawn_grid_tile(grid, Color::srgb(0.22, 0.6, 0.95), Some(1), 3, Some(1), 1);
            spawn_grid_tile(grid, Color::srgb(0.35, 0.74, 0.44), Some(4), 2, Some(1), 2);
            spawn_grid_tile(grid, Color::srgb(0.84, 0.38, 0.35), Some(6), 3, Some(1), 1);
            spawn_grid_tile(grid, Color::srgb(0.88, 0.68, 0.24), Some(9), 2, Some(1), 1);
            spawn_grid_tile(grid, Color::srgb(0.49, 0.53, 0.88), Some(11), 2, Some(1), 2);
            spawn_grid_tile(grid, Color::srgb(0.26, 0.78, 0.8), Some(1), 3, Some(2), 1);
            spawn_grid_tile(grid, Color::srgb(0.57, 0.39, 0.79), Some(6), 2, Some(2), 1);
            spawn_grid_tile(grid, Color::srgb(0.32, 0.66, 0.93), Some(8), 3, Some(2), 1);
        });
}

fn spawn_grid_tile(
    parent: &mut ChildSpawnerCommands,
    color: Color,
    col_start: Option<u32>,
    col_span: u32,
    row_start: Option<u32>,
    row_span: u32,
) {
    parent.spawn((
        UNode {
            width: UVal::Auto,
            height: UVal::Auto,
            background_color: color,
            border_radius: UCornerRadius::all(8.0),
            ..default()
        },
        USelf {
            item_ext: ULayoutItemExt {
                grid: ULayoutGridItem {
                    column_start: col_start,
                    column_span: col_span.max(1),
                    row_start,
                    row_span: row_span.max(1),
                },
                ..default()
            },
            ..default()
        },
    ));
}

fn flex_grow_item() -> USelf {
    USelf {
        item_ext: ULayoutItemExt {
            flex: ULayoutFlexItem {
                flex_grow: Some(1.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}
