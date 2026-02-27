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
                background_color: Color::srgb(0.07, 0.09, 0.11),
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
                text: "Layout Case: Grid Auto Flow (Column)".to_string(),
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            });

            root
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(340.0),
                        background_color: Color::srgb(0.12, 0.14, 0.18),
                        border_radius: UCornerRadius::all(12.0),
                        padding: USides::all(10.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Grid,
                        grid_columns: 3,
                        align_items: UAlignItems::Center,
                        container_ext: ULayoutContainerExt {
                            box_align: ULayoutBoxAlignContainer {
                                justify_items: Some(UAlignItemsExt::Center),
                                row_gap: Some(8.0),
                                column_gap: Some(12.0),
                                ..default()
                            },
                            grid: ULayoutGridContainer {
                                template_columns: vec![
                                    UTrackSize::Fr(1.0),
                                    UTrackSize::Fr(1.0),
                                    UTrackSize::Fr(1.0),
                                ],
                                template_rows: vec![UTrackSize::Px(54.0), UTrackSize::Px(54.0)],
                                auto_flow: UGridAutoFlow::Column,
                                auto_rows: UTrackSize::Px(46.0),
                                auto_columns: UTrackSize::Fr(1.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|grid| {
                    for i in 0..9 {
                        let mut e = grid.spawn(UNode {
                            width: UVal::Px(40.0 + (i % 3) as f32 * 18.0),
                            height: UVal::Px(34.0),
                            background_color: Color::srgb(0.45, 0.25 + i as f32 * 0.06, 0.78),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        });

                        if i == 5 {
                            e.insert(USelf {
                                item_ext: ULayoutItemExt {
                                    grid: ULayoutGridItem {
                                        column_span: 2,
                                        row_span: 2,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    }
                });
        });
}
