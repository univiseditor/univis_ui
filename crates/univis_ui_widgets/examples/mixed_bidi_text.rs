//! Demonstrates mixed Arabic + Latin text with bidi-aware truncation.
//!
//! Run with:
//! `cargo run -p univis_ui_widgets --example mixed_bidi_text`

use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::prelude::Theme;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

const PANEL_WIDTH: f32 = 1460.0;
const MIXED_SENTENCE: &str =
    "status: جاهز | build v1.24.7 | المسار: /srv/api/releases/final_candidate";
const MIXED_PATH: &str = "logs/2026-04-03/session-42 -> تقرير_الاصدار/final_review/version_12";
const MIXED_START: &str =
    "env=prod | المنطقة: الدار البيضاء | endpoint=https://api.univis.dev/v2/metrics";

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

fn setup(mut commands: Commands, theme: Res<Theme>) {
    let mixed_font = theme.text.font.free_serif.clone();

    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi {
                meters_per_unit: 1.0,
                ..URootUi::world_2d(Vec2::new(1680.0, 980.0))
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
                    padding: USides::all(26.0),
                    background_color: Color::srgb(0.10, 0.12, 0.17),
                    border_radius: UCornerRadius::all(28.0),
                    ..default()
                },
                UBorder {
                    width: 2.0,
                    color: Color::srgba(0.68, 0.84, 0.98, 0.28),
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
                    text: "Mixed Arabic + Latin Text".into(),
                    font_size: 38.0,
                    color: Color::srgb(0.96, 0.94, 0.88),
                    font: mixed_font.clone(),
                    ..default()
                });

                panel.spawn((
                    UNode {
                        width: UVal::Px(980.0),
                        ..default()
                    },
                    UTextLabel {
                        text: "This example shows how UTextLabel handles Arabic + Latin in the same line with default ellipsis, start truncation, middle truncation, and different text alignment."
                            .into(),
                        font_size: 18.0,
                        color: Color::srgba(0.88, 0.93, 0.97, 0.86),
                        autosize: false,
                        linebreak: LineBreak::WordBoundary,
                        font: mixed_font.clone(),
                        ..default()
                    },
                ));

                panel.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Flex(1.0),
                        padding: USides::all(18.0),
                        background_color: Color::srgb(0.08, 0.10, 0.13),
                        border_radius: UCornerRadius::all(24.0),
                        ..default()
                    },
                    UBorder {
                        width: 1.0,
                        color: Color::srgba(0.58, 0.71, 0.82, 0.30),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        gap: 16.0,
                        container_ext: ULayoutContainerExt {
                            box_align: ULayoutBoxAlignContainer {
                                row_gap: Some(16.0),
                                column_gap: Some(16.0),
                                ..default()
                            },
                            flex: ULayoutFlexContainer {
                                wrap: UFlexWrap::Wrap,
                                align_content: Some(UContentAlignExt::Center),
                            },
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|grid| {
                    spawn_demo_card(
                        grid,
                        mixed_font.clone(),
                        "Default Ellipsis",
                        "The default mode should keep the beginning visible and append three dots.",
                        MIXED_SENTENCE,
                        Justify::Left,
                        UTextTruncateSide::Auto,
                    );
                    spawn_demo_card(
                        grid,
                        mixed_font.clone(),
                        "Start Truncation",
                        "Useful when the end of the mixed string matters more than the beginning.",
                        MIXED_START,
                        Justify::Left,
                        UTextTruncateSide::Start,
                    );
                    spawn_demo_card(
                        grid,
                        mixed_font.clone(),
                        "Middle Truncation",
                        "Useful for paths, ids, and release labels where both edges matter.",
                        MIXED_PATH,
                        Justify::Left,
                        UTextTruncateSide::Middle,
                    );
                    spawn_demo_card(
                        grid,
                        mixed_font.clone(),
                        "Right Justify",
                        "Arabic-heavy text can be right aligned while still truncating cleanly.",
                        "الحالة النهائية | release candidate | build=2026.04.03 | المسار=/var/data/archive/final",
                        Justify::Right,
                        UTextTruncateSide::Auto,
                    );
                    spawn_demo_card(
                        grid,
                        mixed_font.clone(),
                        "Wrap + Mixed Script",
                        "When wrapping is allowed, the label keeps both scripts readable without truncation.",
                        "المحرر Editor يعرض line breaks بشكل واضح داخل نفس البطاقة when width is fixed and wrapping is enabled.",
                        Justify::Left,
                        UTextTruncateSide::Auto,
                    );
                });
            });
        });
}

fn spawn_demo_card(
    parent: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    title: &str,
    description: &str,
    value: &str,
    justify: Justify,
    truncate_side: UTextTruncateSide,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Px(440.0),
                height: UVal::Px(210.0),
                padding: USides::all(18.0),
                background_color: Color::srgb(0.13, 0.16, 0.22),
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UBorder {
                width: 1.0,
                color: Color::srgba(0.86, 0.76, 0.47, 0.22),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
            USelf {
                item_ext: ULayoutItemExt {
                    flex: ULayoutFlexItem {
                        flex_shrink: Some(0.0),
                        flex_basis: Some(UVal::Px(440.0)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|card| {
            card.spawn(UTextLabel {
                text: title.to_string(),
                font_size: 22.0,
                color: Color::WHITE,
                font: font.clone(),
                ..default()
            });

            card.spawn(UTextLabel {
                text: description.to_string(),
                font_size: 13.0,
                color: Color::srgba(0.84, 0.90, 0.95, 0.82),
                autosize: false,
                linebreak: LineBreak::WordBoundary,
                font: font.clone(),
                ..default()
            });

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(78.0),
                    padding: USides::all(14.0),
                    background_color: Color::srgb(0.08, 0.09, 0.12),
                    border_radius: UCornerRadius::all(16.0),
                    ..default()
                },
                UBorder {
                    width: 1.0,
                    color: Color::srgba(0.42, 0.72, 0.96, 0.34),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: match justify {
                        Justify::Right => UJustifyContent::End,
                        Justify::Center | Justify::Justified => UJustifyContent::Center,
                        Justify::Left => UJustifyContent::Start,
                    },
                    align_items: UAlignItems::Center,
                    ..default()
                },
                UClip::enabled(true),
            ))
            .with_children(|label_box| {
                let linebreak = if title == "Wrap + Mixed Script" {
                    LineBreak::WordBoundary
                } else {
                    LineBreak::NoWrap
                };
                let max_lines = if title == "Wrap + Mixed Script" {
                    Some(2)
                } else {
                    Some(1)
                };

                label_box.spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        ..default()
                    },
                    UTextLabel {
                        text: value.to_string(),
                        font_size: 20.0,
                        color: Color::srgb(0.95, 0.95, 0.93),
                        autosize: false,
                        font: font.clone(),
                        justify,
                        linebreak,
                        truncate_side,
                        max_lines,
                        ..default()
                    },
                ));
            });
        });
}
