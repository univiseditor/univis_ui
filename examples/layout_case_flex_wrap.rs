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
                background_color: Color::srgb(0.08, 0.1, 0.14),
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
                text: "Layout Case: Flex Wrap + Extended Alignment".to_string(),
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            });

            root
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(320.0),
                        background_color: Color::srgb(0.12, 0.15, 0.2),
                        border_radius: UCornerRadius::all(12.0),
                        padding: USides::all(12.0),
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
                                column_gap: Some(10.0),
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
                .with_children(|container| {
                    for i in 0..12 {
                        let mut item = container.spawn(UNode {
                            width: UVal::Px(90.0 + (i % 3) as f32 * 12.0),
                            height: UVal::Px(38.0 + (i % 2) as f32 * 18.0),
                            background_color: Color::srgb(0.2 + i as f32 * 0.05, 0.38, 0.72),
                            border_radius: UCornerRadius::all(8.0),
                            ..default()
                        });

                        if i == 2 {
                            item.insert(USelf {
                                item_ext: ULayoutItemExt {
                                    flex: ULayoutFlexItem {
                                        flex_grow: Some(1.0),
                                        flex_shrink: Some(1.0),
                                        flex_basis: Some(UVal::Px(180.0)),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                        }

                        if i == 5 {
                            item.insert(USelf {
                                item_ext: ULayoutItemExt {
                                    box_align: ULayoutBoxAlignSelf {
                                        justify_self: Some(UAlignSelfExt::Center),
                                        align_self: Some(UAlignSelfExt::Center),
                                        justify_overflow: UOverflowPosition::Safe,
                                        align_overflow: UOverflowPosition::Safe,
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
