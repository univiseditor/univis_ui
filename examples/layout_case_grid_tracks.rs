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
                background_color: Color::srgb(0.09, 0.1, 0.13),
                padding: USides::all(28.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel {
                text: "Layout Case: Grid Tracks + Span".to_string(),
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            });

            root
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(360.0),
                        background_color: Color::srgb(0.14, 0.16, 0.2),
                        border_radius: UCornerRadius::all(12.0),
                        padding: USides::all(10.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Grid,
                        grid_columns: 4,
                        align_items: UAlignItems::Stretch,
                        container_ext: ULayoutContainerExt {
                            box_align: ULayoutBoxAlignContainer {
                                justify_items: Some(UAlignItemsExt::Center),
                                row_gap: Some(10.0),
                                column_gap: Some(10.0),
                                ..default()
                            },
                            grid: ULayoutGridContainer {
                                template_columns: vec![
                                    UTrackSize::Fr(1.0),
                                    UTrackSize::Px(120.0),
                                    UTrackSize::Fr(1.0),
                                    UTrackSize::Fr(2.0),
                                ],
                                template_rows: vec![UTrackSize::Px(76.0), UTrackSize::Px(76.0)],
                                auto_rows: UTrackSize::Px(62.0),
                                auto_columns: UTrackSize::Fr(1.0),
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
                            width: UVal::Px(140.0),
                            height: UVal::Px(46.0),
                            background_color: Color::srgb(0.74, 0.33, 0.28),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        USelf {
                            item_ext: ULayoutItemExt {
                                box_align: ULayoutBoxAlignSelf {
                                    justify_self: Some(UAlignSelfExt::End),
                                    align_self: Some(UAlignSelfExt::Center),
                                    justify_overflow: UOverflowPosition::Safe,
                                    align_overflow: UOverflowPosition::Safe,
                                },
                                grid: ULayoutGridItem {
                                    column_span: 2,
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    grid.spawn((
                        UNode {
                            width: UVal::Px(60.0),
                            height: UVal::Px(46.0),
                            background_color: Color::srgb(0.27, 0.76, 0.4),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        USelf {
                            item_ext: ULayoutItemExt {
                                grid: ULayoutGridItem {
                                    column_start: Some(4),
                                    row_start: Some(1),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    grid.spawn((
                        UNode {
                            width: UVal::Px(180.0),
                            height: UVal::Px(46.0),
                            background_color: Color::srgb(0.3, 0.55, 0.86),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        },
                        USelf {
                            item_ext: ULayoutItemExt {
                                grid: ULayoutGridItem {
                                    column_start: Some(2),
                                    column_span: 3,
                                    row_start: Some(2),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    for i in 0..8 {
                        grid.spawn(UNode {
                            width: UVal::Px(66.0 + i as f32 * 4.0),
                            height: UVal::Px(42.0),
                            background_color: Color::srgb(0.2, 0.42 + i as f32 * 0.05, 0.7),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        });
                    }
                });
        });
}
