//! Focused widget-level sizing semantics scene.
//!
//! Run with:
//! `cargo run -p univis_ui_widgets --example widget_sizing_semantics`

use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

const PANEL_WIDTH: f32 = 1440.0;
const WRAP_PANEL_WIDTH: f32 = 860.0;
const IMAGE_TEXTURE_PATH: &str = "background.png";
const INTRINSIC_TEXT: &str = "MinContent wraps this sentence down to its tight intrinsic width, while MaxContent keeps the measured line wider until explicit max bounds step in.";

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

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let image = assets.load(IMAGE_TEXTURE_PATH);

    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi {
                meters_per_unit: 1.0,
                ..URootUi::world_2d(Vec2::new(1600.0, 980.0))
            },
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(28.0),
                background_color: Color::srgb(0.05, 0.07, 0.10),
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
                    width: UVal::Px(PANEL_WIDTH),
                    height: UVal::Px(860.0),
                    padding: USides::all(24.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(28.0),
                    ..default()
                },
                UBorder {
                    width: 2.0,
                    color: Color::srgba(0.84, 0.92, 0.98, 0.22),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 18.0,
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn(UTextLabel {
                    text: "Widget Sizing Semantics".into(),
                    font_size: 36.0,
                    color: Color::srgb(0.95, 0.95, 0.91),
                    ..default()
                });

                panel.spawn((
                    UNode {
                        width: UVal::Px(1120.0),
                        ..default()
                    },
                    UTextLabel {
                        text: "This scene combines fixed cards, wrap breakpoints, explicit min_width, MinContent/MaxContent text containers, and UImage intrinsic sizing in one widget-focused layout."
                            .into(),
                        font_size: 17.0,
                        color: Color::srgba(0.88, 0.93, 0.97, 0.84),
                        autosize: false,
                        linebreak: LineBreak::WordBoundary,
                        ..default()
                    },
                ));

                panel.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(320.0),
                        background_color: Color::srgb(0.07, 0.09, 0.13),
                        border_radius: UCornerRadius::all(18.0),
                        padding: USides::all(16.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 12.0,
                        ..default()
                    },
                ))
                .with_children(|section| {
                    spawn_section_header(
                        section,
                        "Wrap + fixed cards + min_width",
                        "The wrap panel is intentionally narrow. Fixed cards opt out of shrink, while the flexible card still refuses to go below its min_width.",
                    );

                    section
                        .spawn((
                            UNode {
                                width: UVal::Px(WRAP_PANEL_WIDTH),
                                height: UVal::Px(214.0),
                                background_color: Color::srgb(0.10, 0.12, 0.18),
                                border_radius: UCornerRadius::all(16.0),
                                padding: USides::all(14.0),
                                ..default()
                            },
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                gap: 14.0,
                                container_ext: ULayoutContainerExt {
                                    flex: ULayoutFlexContainer {
                                        wrap: UFlexWrap::Wrap,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|wrap| {
                            spawn_fixed_wrap_card(
                                wrap,
                                "Fixed card A",
                                "width=248, flex_shrink=0",
                                Color::srgb(0.28, 0.54, 0.88),
                            );
                            spawn_fixed_wrap_card(
                                wrap,
                                "Fixed card B",
                                "width=248, flex_shrink=0",
                                Color::srgb(0.22, 0.66, 0.45),
                            );
                            spawn_fixed_wrap_card(
                                wrap,
                                "Fixed card C",
                                "width=248, flex_shrink=0",
                                Color::srgb(0.70, 0.45, 0.26),
                            );
                            spawn_min_width_wrap_card(wrap);
                        });
                });

                panel.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(360.0),
                        background_color: Color::srgb(0.07, 0.09, 0.13),
                        border_radius: UCornerRadius::all(18.0),
                        padding: USides::all(16.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 12.0,
                        ..default()
                    },
                ))
                .with_children(|section| {
                    spawn_section_header(
                        section,
                        "Intrinsic text + image sizing",
                        "MinContent and MaxContent stay explicit here, while UImage resolves intrinsic modes to its native texture size.",
                    );

                    section
                        .spawn((
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
                            spawn_intrinsic_text_panel(row, "MinContent", UVal::MinContent);
                            spawn_intrinsic_text_panel(row, "MaxContent", UVal::MaxContent);
                            spawn_image_intrinsic_panel(row, image.clone());
                        });
                });
            });
        });
}

fn spawn_section_header(parent: &mut ChildSpawnerCommands, title: &str, body: &str) {
    parent.spawn(UTextLabel {
        text: title.into(),
        font_size: 22.0,
        color: Color::srgb(0.95, 0.95, 0.91),
        ..default()
    });

    parent.spawn((
        UNode {
            width: UVal::Px(1120.0),
            ..default()
        },
        UTextLabel {
            text: body.into(),
            font_size: 15.0,
            color: Color::srgba(0.86, 0.91, 0.96, 0.82),
            autosize: false,
            linebreak: LineBreak::WordBoundary,
            ..default()
        },
    ));
}

fn spawn_fixed_wrap_card(parent: &mut ChildSpawnerCommands, title: &str, body: &str, color: Color) {
    parent
        .spawn((
            UNode {
                width: UVal::Px(248.0),
                height: UVal::Px(80.0),
                padding: USides::all(12.0),
                background_color: color,
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_shrink: Some(0.0),
                        flex_basis: Some(UVal::Px(248.0)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: title.into(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            card.spawn(UTextLabel {
                text: body.into(),
                font_size: 13.0,
                color: Color::srgba(1.0, 1.0, 1.0, 0.88),
                ..default()
            });
        });
}

fn spawn_min_width_wrap_card(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                min_width: 220.0,
                height: UVal::Px(80.0),
                padding: USides::all(12.0),
                background_color: Color::srgb(0.42, 0.30, 0.70),
                border_radius: UCornerRadius::all(14.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_grow: Some(1.0),
                        flex_shrink: Some(1.0),
                        flex_basis: Some(UVal::Px(280.0)),
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: "Flexible card".into(),
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            });
            card.spawn(UTextLabel {
                text: "flex_grow=1, flex_basis=280, min_width=220".into(),
                font_size: 13.0,
                color: Color::srgba(1.0, 1.0, 1.0, 0.88),
                ..default()
            });
        });
}

fn spawn_intrinsic_text_panel(parent: &mut ChildSpawnerCommands, title: &str, width_mode: UVal) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgb(0.10, 0.12, 0.18),
                border_radius: UCornerRadius::all(16.0),
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
                text: title.into(),
                font_size: 18.0,
                color: Color::srgb(0.95, 0.95, 0.91),
                ..default()
            });

            panel
                .spawn((
                    UNode {
                        width: width_mode,
                        max_width: 360.0,
                        padding: USides::all(12.0),
                        background_color: Color::srgb(0.18, 0.22, 0.30),
                        border_radius: UCornerRadius::all(12.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        ..default()
                    },
                ))
                .with_children(|sample| {
                    sample.spawn(UTextLabel {
                        text: INTRINSIC_TEXT.into(),
                        font_size: 14.0,
                        linebreak: LineBreak::WordBoundary,
                        color: Color::srgba(0.92, 0.96, 1.0, 0.92),
                        ..default()
                    });
                });

            panel.spawn(UTextLabel {
                text: match width_mode {
                    UVal::MinContent => "Tight intrinsic width with early wrapping.",
                    _ => "Wide intrinsic width until max_width clamps it.",
                }
                .into(),
                font_size: 13.0,
                color: Color::srgba(0.82, 0.88, 0.95, 0.82),
                ..default()
            });
        });
}

fn spawn_image_intrinsic_panel(parent: &mut ChildSpawnerCommands, texture: Handle<Image>) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(12.0),
                background_color: Color::srgb(0.10, 0.12, 0.18),
                border_radius: UCornerRadius::all(16.0),
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
                text: "UImage intrinsic sizing".into(),
                font_size: 18.0,
                color: Color::srgb(0.95, 0.95, 0.91),
                ..default()
            });

            panel
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        gap: 10.0,
                        ..default()
                    },
                ))
                .with_children(|row| {
                    spawn_image_demo(row, texture.clone(), "Auto", UVal::Auto);
                    spawn_image_demo(row, texture.clone(), "MaxContent", UVal::MaxContent);
                    spawn_image_demo(row, texture, "MinContent", UVal::MinContent);
                });

            panel.spawn(UTextLabel {
                text: "For images, Auto and intrinsic content modes all resolve to the native texture size once the asset is ready."
                    .into(),
                font_size: 13.0,
                color: Color::srgba(0.82, 0.88, 0.95, 0.82),
                linebreak: LineBreak::WordBoundary,
                ..default()
            });
        });
}

fn spawn_image_demo(
    parent: &mut ChildSpawnerCommands,
    texture: Handle<Image>,
    title: &str,
    size_mode: UVal,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Flex(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(10.0),
                background_color: Color::srgb(0.15, 0.18, 0.25),
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: title.into(),
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            });

            card.spawn((
                UImage {
                    texture,
                    width: size_mode,
                    height: size_mode,
                    radius: Some(UCornerRadius::all(10.0)),
                    ..default()
                },
                UNode {
                    max_width: 120.0,
                    max_height: 82.0,
                    ..default()
                },
            ));
        });
}
