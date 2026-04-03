//! Stress-tests tricky text overflow cases for `UTextLabel`.
//!
//! Run with:
//! `cargo run -p univis_ui_widgets --example text_edge_cases`

use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_engine::prelude::*;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::prelude::Theme;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::prelude::*;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

const PANEL_WIDTH: f32 = 1500.0;
const LEADING_DIGITS: &str = "1..2..3..4..5..6..7..8..9";
const MIXED_STATUS: &str =
    "status=ready | الحالة: جاهز | release=2026.04.03 | path=/srv/ui/final/build";
const END_PRIORITY: &str =
    "tenant=production-eu-west | المسار النهائي | report=release_validation_2026_04_03";
const MIDDLE_PATH: &str =
    "/srv/releases/archive/2026/final/reports/qa/session_42/build_mix_ar_latn/output.json";
const WRAP_PARAGRAPH: &str = "Editor المحرر keeps the first lines readable even when the same label mixes Arabic, Latin, numbers, and punctuation inside a narrow frame.";
const RIGHT_HEAVY: &str =
    "الحالة النهائية | candidate=v12.8.4 | service=api-gateway | checksum=ABF2391";
const AUTOSIZE_CLAMP: &str = "autosize should stop at the parent boundary and still end with ...";

#[derive(Clone, Copy)]
struct DemoConfig {
    width: f32,
    height: f32,
    justify: Justify,
    truncate_side: UTextTruncateSide,
    overflow: UTextOverflow,
    linebreak: LineBreak,
    max_lines: Option<usize>,
    autosize: bool,
}

impl DemoConfig {
    fn single_line(width: f32, truncate_side: UTextTruncateSide) -> Self {
        Self {
            width,
            height: 72.0,
            justify: Justify::Left,
            truncate_side,
            overflow: UTextOverflow::Ellipsis,
            linebreak: LineBreak::NoWrap,
            max_lines: Some(1),
            autosize: false,
        }
    }
}

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
                ..URootUi::world_2d(Vec2::new(1720.0, 1020.0))
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
                    height: UVal::Px(900.0),
                    padding: USides::all(26.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(30.0),
                    ..default()
                },
                UBorder {
                    width: 2.0,
                    color: Color::srgba(0.83, 0.92, 0.98, 0.24),
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
                    text: "UTextLabel Edge Cases".into(),
                    font_size: 38.0,
                    color: Color::srgb(0.96, 0.95, 0.90),
                    font: mixed_font.clone(),
                    ..default()
                });

                panel.spawn((
                    UNode {
                        width: UVal::Px(1100.0),
                        ..default()
                    },
                    UTextLabel {
                        text: "This scene stresses the awkward cases: numeric prefixes, Arabic + Latin mixes, start/end/middle truncation, autosize under parent limits, right alignment, and two-line clamp."
                            .into(),
                        font_size: 18.0,
                        color: Color::srgba(0.88, 0.93, 0.97, 0.84),
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
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Leading Digits",
                        "Default ellipsis should preserve the visible start, so a tight box begins with 1.. instead of the middle.",
                        LEADING_DIGITS,
                        DemoConfig::single_line(230.0, UTextTruncateSide::Auto),
                    );
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Mixed Bidi End",
                        "Default truncation keeps the start readable when Arabic and Latin live in one line.",
                        MIXED_STATUS,
                        DemoConfig::single_line(290.0, UTextTruncateSide::Auto),
                    );
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Start Truncation",
                        "Useful when the suffix matters more than the opening prefix.",
                        END_PRIORITY,
                        DemoConfig::single_line(300.0, UTextTruncateSide::Start),
                    );
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Middle Truncation",
                        "Paths and artifact ids often need both the beginning and the end.",
                        MIDDLE_PATH,
                        DemoConfig::single_line(300.0, UTextTruncateSide::Middle),
                    );
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Autosize Clamp",
                        "The label keeps autosize on, but the parent still limits how far it can grow.",
                        AUTOSIZE_CLAMP,
                        DemoConfig {
                            width: 250.0,
                            height: 74.0,
                            justify: Justify::Left,
                            truncate_side: UTextTruncateSide::Auto,
                            overflow: UTextOverflow::Ellipsis,
                            linebreak: LineBreak::NoWrap,
                            max_lines: Some(1),
                            autosize: true,
                        },
                    );
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Right Align",
                        "Arabic-heavy content should stay anchored to the end edge instead of drifting toward the center.",
                        RIGHT_HEAVY,
                        DemoConfig {
                            width: 320.0,
                            height: 72.0,
                            justify: Justify::Right,
                            truncate_side: UTextTruncateSide::Auto,
                            overflow: UTextOverflow::Ellipsis,
                            linebreak: LineBreak::NoWrap,
                            max_lines: Some(1),
                            autosize: false,
                        },
                    );
                    spawn_case_card(
                        grid,
                        mixed_font.clone(),
                        "Two-Line Clamp",
                        "Wrapping plus max-lines should keep the first lines legible and append ... when more text remains.",
                        WRAP_PARAGRAPH,
                        DemoConfig {
                            width: 320.0,
                            height: 108.0,
                            justify: Justify::Left,
                            truncate_side: UTextTruncateSide::Auto,
                            overflow: UTextOverflow::Ellipsis,
                            linebreak: LineBreak::WordBoundary,
                            max_lines: Some(2),
                            autosize: false,
                        },
                    );
                });
            });
        });
}

fn spawn_case_card(
    parent: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    title: &str,
    description: &str,
    value: &str,
    config: DemoConfig,
) {
    parent
        .spawn((
            UNode {
                width: UVal::Px(454.0),
                height: UVal::Px(222.0),
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
                        flex_basis: Some(UVal::Px(454.0)),
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

            card.spawn((
                UNode {
                    width: UVal::Percent(1.0),
                    ..default()
                },
                UTextLabel {
                    text: description.to_string(),
                    font_size: 13.0,
                    color: Color::srgba(0.84, 0.90, 0.95, 0.82),
                    autosize: false,
                    linebreak: LineBreak::WordBoundary,
                    font: font.clone(),
                    ..default()
                },
            ));

            card.spawn((
                UNode {
                    width: UVal::Px(config.width),
                    height: UVal::Px(config.height),
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
                    justify_content: match config.justify {
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
                label_box.spawn((
                    UNode {
                        width: if config.autosize {
                            UVal::Content
                        } else {
                            UVal::Percent(1.0)
                        },
                        ..default()
                    },
                    UTextLabel {
                        text: value.to_string(),
                        font_size: 20.0,
                        color: Color::srgb(0.95, 0.95, 0.93),
                        autosize: config.autosize,
                        font: font.clone(),
                        justify: config.justify,
                        linebreak: config.linebreak,
                        overflow: config.overflow,
                        truncate_side: config.truncate_side,
                        max_lines: config.max_lines,
                        ..default()
                    },
                ));
            });
        });
}
