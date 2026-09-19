//! # 2D Text Engine Showcase - Material You (M3) Style
//!
//! A comprehensive Bevy example demonstrating Univis UI's 2D text capabilities
//! styled with the Material Design 3 (Material You / M3) visual design language:
//! - M3 Tonal Surface hierarchy (Surface, Container Low, Container, Container High)
//! - M3 Shape tokens (Small 8px, Medium 12px, Large 16px, Extra Large 28px, Full Pill)
//! - M3 Pill-shaped buttons and interactive chips
//! - Intrinsic size measurement and autosize fitting
//! - Multi-line word wrapping and container height adaptation
//! - Text justification & alignment (Left, Center, Right)
//! - Overflow policies & truncation modes (Ellipsis: End, Middle, Start; Clip)
//! - BiDi & Arabic text shaping and rendering with embedded Noto / FreeSerif fonts
//! - Real-time corner radius measurement and surrounding constraint inspection

use bevy::prelude::*;
use bevy::text::LineBreak;
use univis_ui::prelude::*;

// ── Material Design 3 (Material You) Design System ──────────────────────────

#[allow(dead_code)]
struct M3;

#[allow(dead_code)]
impl M3 {
    // Surface Tonal Hierarchy (Dark Theme)
    pub const SURFACE: Color = Color::srgb(0.08, 0.07, 0.10); // #141218
    pub const SURFACE_CONTAINER_LOW: Color = Color::srgb(0.11, 0.10, 0.13); // #1D1B20
    pub const SURFACE_CONTAINER: Color = Color::srgb(0.13, 0.12, 0.15); // #211F26
    pub const SURFACE_CONTAINER_HIGH: Color = Color::srgb(0.17, 0.16, 0.19); // #2B2930
    pub const SURFACE_CONTAINER_HIGHEST: Color = Color::srgb(0.21, 0.20, 0.23); // #36343B

    // Outlines & Dividers
    pub const OUTLINE_VARIANT: Color = Color::srgba(0.58, 0.56, 0.60, 0.25); // Subtle border
    pub const OUTLINE_FOCUSED: Color = Color::srgb(0.82, 0.74, 1.0); // Focus outline

    // Primary Tonal (Lavender / Violet)
    pub const PRIMARY: Color = Color::srgb(0.82, 0.74, 1.0); // #D0BCFF
    pub const PRIMARY_CONTAINER: Color = Color::srgb(0.31, 0.22, 0.55); // #4F378B
    pub const ON_PRIMARY_CONTAINER: Color = Color::srgb(0.92, 0.87, 1.0); // #EADDFF

    // Secondary Tonal (Teal / Slate)
    pub const SECONDARY_CONTAINER: Color = Color::srgb(0.29, 0.27, 0.35); // #4A4458
    pub const ON_SECONDARY_CONTAINER: Color = Color::srgb(0.91, 0.87, 0.97); // #E8DEF8

    // Tertiary Tonal (Rose / Melon)
    pub const TERTIARY_CONTAINER: Color = Color::srgb(0.39, 0.23, 0.28); // #633B48
    pub const ON_TERTIARY_CONTAINER: Color = Color::srgb(1.0, 0.85, 0.89); // #FFD8E4

    // Success Tonal (Emerald Green)
    pub const SUCCESS_CONTAINER: Color = Color::srgb(0.12, 0.30, 0.20);
    pub const ON_SUCCESS_CONTAINER: Color = Color::srgb(0.72, 0.96, 0.80);

    // High & Medium Emphasis Typography
    pub const ON_SURFACE: Color = Color::srgb(0.90, 0.88, 0.91); // #E6E0E9
    pub const ON_SURFACE_VARIANT: Color = Color::srgb(0.79, 0.77, 0.82); // #CAC4D0
    pub const ON_SURFACE_MUTED: Color = Color::srgb(0.58, 0.56, 0.60); // #938F99
}

// ── Showcase Presets & State ────────────────────────────────────────────────

const M3_TEXT_PRESETS: &[(&str, &str)] = &[
    (
        "Headline & Subhead",
        "Material You delivers personalized, expressive design that adapts dynamically to users and layouts.",
    ),
    (
        "Body Medium",
        "Univis UI brings fluid SDF text measurement, automatic line reflow, and BiDi shaping to the Bevy game engine.",
    ),
    (
        "Arabic BiDi Title",
        "تصميم ماتيريال يو (Material 3) يتميز بالزوايا الانسيابية والخطوط الواضحة مع التوافق التام مع النصوص العربية.",
    ),
    ("Compact Badge", "Active Workspace"),
];

struct M3ShapePreset {
    name: &'static str,
    token: &'static str,
    radius: UCornerRadius,
}

const M3_SHAPE_PRESETS: &[M3ShapePreset] = &[
    M3ShapePreset {
        name: "Large Card (16px)",
        token: "Shape::CornerLarge",
        radius: UCornerRadius {
            top_left: 16.0,
            top_right: 16.0,
            bottom_right: 16.0,
            bottom_left: 16.0,
        },
    },
    M3ShapePreset {
        name: "Extra Large Sheet (28px)",
        token: "Shape::CornerExtraLarge",
        radius: UCornerRadius {
            top_left: 28.0,
            top_right: 28.0,
            bottom_right: 28.0,
            bottom_left: 28.0,
        },
    },
    M3ShapePreset {
        name: "Full Pill / Chip (36px)",
        token: "Shape::CornerFull",
        radius: UCornerRadius {
            top_left: 36.0,
            top_right: 36.0,
            bottom_right: 36.0,
            bottom_left: 36.0,
        },
    },
    M3ShapePreset {
        name: "Top Rounded Sheet (Top: 24px)",
        token: "Shape::TopSheet",
        radius: UCornerRadius {
            top_left: 24.0,
            top_right: 24.0,
            bottom_right: 4.0,
            bottom_left: 4.0,
        },
    },
    M3ShapePreset {
        name: "Small Component (8px)",
        token: "Shape::CornerSmall",
        radius: UCornerRadius {
            top_left: 8.0,
            top_right: 8.0,
            bottom_right: 8.0,
            bottom_left: 8.0,
        },
    },
];

#[derive(Resource)]
struct TextShowcaseState {
    container_width: f32,
    text_preset_idx: usize,
    truncate_side: UTextTruncateSide,
    corner_preset_idx: usize,
}

impl Default for TextShowcaseState {
    fn default() -> Self {
        Self {
            container_width: 380.0,
            text_preset_idx: 0,
            truncate_side: UTextTruncateSide::End,
            corner_preset_idx: 0,
        }
    }
}

// ── Components ──────────────────────────────────────────────────────────────

#[derive(Component)]
struct DynamicContainer;

#[derive(Component)]
struct DynamicText;

#[derive(Clone, Copy, PartialEq, Eq)]
enum InspectorMetric {
    ShapeToken,
    ContainerWidth,
    CornerRadii,
    MeasuredSize,
    LineCount,
    ParentBound,
    OverflowPolicy,
}

#[derive(Component)]
struct InspectorMetricText(InspectorMetric);

// ── Main Entry ──────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Univis UI - Material Design 3 (Material You) Text Showcase".into(),
                    resolution: (1360, 940).into(),
                    ..default()
                }),
                ..default()
            }),
            UnivisUiPlugin,
        ))
        .init_resource::<TextShowcaseState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_dynamic_box, update_inspector_hud))
        .run();
}

// ── Setup Scene ─────────────────────────────────────────────────────────────

fn setup_scene(mut commands: Commands, state: Res<TextShowcaseState>, theme: Res<Theme>) {
    let font_body = theme.text.font.inter_regular.clone();
    let font_arabic = theme.text.font.free_serif.clone();

    // 1. 2D Camera with M3 Surface clear color
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(M3::SURFACE),
            ..default()
        },
    ));

    // 2. Screen Space Root with M3 standard padding
    let root = commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
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
        .id();

    // ── M3 Top App Bar (Header) ─────────────────────────────────────────────
    let app_bar = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(24.0, 16.0),
                background_color: M3::SURFACE_CONTAINER,
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let title_group = commands
        .spawn((
            ChildOf(app_bar),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(title_group),
        UNode::default(),
        UTextLabel {
            text: "Material 3 Typography & Layout Engine".to_string(),
            color: M3::ON_SURFACE,
            font_size: 22.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(title_group),
        UNode::default(),
        UTextLabel {
            text: "Dynamic text measurement, container reflow, corner shape tokens, and adaptive layout in Univis UI"
                .to_string(),
            color: M3::ON_SURFACE_VARIANT,
            font_size: 13.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    // M3 Tonal Pill Badge in App Bar
    commands
        .spawn((
            ChildOf(app_bar),
            UNode {
                padding: USides::axes(16.0, 8.0),
                background_color: M3::PRIMARY_CONTAINER,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((
                UNode::default(),
                UTextLabel {
                    text: "Material You | M3".to_string(),
                    color: M3::ON_PRIMARY_CONTAINER,
                    font_size: 13.0,
                    font: font_body.clone(),
                    ..default()
                },
            ));
        });

    // ── Main Body: Two Columns ──────────────────────────────────────────────
    // Total available width: 1360 - 48 (padding) = 1312px. Body gap: 20px.
    // Left Column: 660px. Right Column: 632px. Total: 660 + 20 + 632 = 1312px.
    let body = commands
        .spawn((
            ChildOf(root),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 20.0,
                ..default()
            },
        ))
        .id();

    // ────────────────────────────────────────────────────────────────────────
    // LEFT COLUMN: M3 Components & Typography Cases
    // ────────────────────────────────────────────────────────────────────────
    let left_column = commands
        .spawn((
            ChildOf(body),
            UNode {
                width: UVal::Px(660.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    // ── Case 1: M3 Chips & Badges (Autosize Fitting) ──
    let case1_card = commands
        .spawn((
            ChildOf(left_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: M3::SURFACE_CONTAINER_LOW,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(case1_card),
        UNode::default(),
        UTextLabel {
            text: "1. M3 CHIPS & BADGES (Autosize intrinsic bounds)".to_string(),
            color: M3::PRIMARY,
            font_size: 13.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    let case1_row = commands
        .spawn((
            ChildOf(case1_card),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    let chips_data = [
        (
            "Filter Chip (r=16px)",
            M3::SURFACE_CONTAINER_HIGH,
            M3::ON_SURFACE,
            16.0,
            true,
        ),
        (
            "Assist Chip (r=8px)",
            M3::PRIMARY_CONTAINER,
            M3::ON_PRIMARY_CONTAINER,
            8.0,
            false,
        ),
        (
            "Suggestion Pill (r=20px)",
            M3::SECONDARY_CONTAINER,
            M3::ON_SECONDARY_CONTAINER,
            20.0,
            false,
        ),
        (
            "Active Status",
            M3::SUCCESS_CONTAINER,
            M3::ON_SUCCESS_CONTAINER,
            16.0,
            false,
        ),
    ];

    for (chip_text, bg, text_color, radius, has_border) in chips_data {
        let chip_node = commands
            .spawn((
                ChildOf(case1_row),
                UNode {
                    padding: USides::axes(14.0, 7.0),
                    background_color: bg,
                    border_radius: UCornerRadius::all(radius),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    ..default()
                },
            ))
            .id();

        if has_border {
            commands.entity(chip_node).insert(UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(radius),
                offset: 0.0,
            });
        }

        commands.spawn((
            ChildOf(chip_node),
            UNode::default(),
            UTextLabel {
                text: chip_text.to_string(),
                color: text_color,
                font_size: 12.0,
                font: font_body.clone(),
                autosize: true,
                ..default()
            },
        ));
    }

    // ── Case 2: Multi-line Word Wrapping (Parent Height Adaptation) ──
    let case2_card = commands
        .spawn((
            ChildOf(left_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: M3::SURFACE_CONTAINER_LOW,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(case2_card),
        UNode::default(),
        UTextLabel {
            text: "2. M3 BODY & MULTI-LINE REFLOW (Parent adapts height to content)".to_string(),
            color: M3::PRIMARY,
            font_size: 13.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    let case2_body_card = commands
        .spawn((
            ChildOf(case2_card),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: M3::SURFACE_CONTAINER,
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(case2_body_card),
        UNode {
            width: UVal::Percent(1.0),
            ..default()
        },
        UTextLabel {
            text: "In Material Design 3, typography scale flows naturally within cards. LineBreak::WordOrCharacter allows text to wrap gracefully across multiple lines, expanding the card's vertical height dynamically without clipping or overflowing layout bounds.".to_string(),
            color: M3::ON_SURFACE_VARIANT,
            font_size: 14.0,
            font: font_body.clone(),
            linebreak: LineBreak::WordOrCharacter,
            autosize: true,
            ..default()
        },
    ));

    // ── Case 3: Justification & Alignment ──
    let case3_card = commands
        .spawn((
            ChildOf(left_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: M3::SURFACE_CONTAINER_LOW,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(case3_card),
        UNode::default(),
        UTextLabel {
            text: "3. TEXT ALIGNMENT & JUSTIFICATION (Positioning in M3 Cards)".to_string(),
            color: M3::PRIMARY,
            font_size: 13.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    let case3_row = commands
        .spawn((
            ChildOf(case3_card),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    let align_cards = [
        (
            Justify::Left,
            "Left Aligned",
            "Natural reading start edge anchor.",
        ),
        (
            Justify::Center,
            "Center Aligned",
            "Centered in M3 dialogue or card.",
        ),
        (
            Justify::Right,
            "Right Aligned",
            "Trailing metric or end edge anchor.",
        ),
    ];

    for (justify, title, desc) in align_cards {
        let card = commands
            .spawn((
                ChildOf(case3_row),
                UNode {
                    width: UVal::Px(202.0),
                    padding: USides::axes(12.0, 10.0),
                    background_color: M3::SURFACE_CONTAINER,
                    border_radius: UCornerRadius::all(12.0),
                    ..default()
                },
                UBorder {
                    color: M3::OUTLINE_VARIANT,
                    width: 1.0,
                    radius: UCornerRadius::all(12.0),
                    offset: 0.0,
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(card),
            UNode::default(),
            UTextLabel {
                text: title.to_string(),
                color: M3::PRIMARY,
                font_size: 12.0,
                font: font_body.clone(),
                justify,
                ..default()
            },
        ));

        commands.spawn((
            ChildOf(card),
            UNode::default(),
            UTextLabel {
                text: desc.to_string(),
                color: M3::ON_SURFACE,
                font_size: 13.0,
                font: font_body.clone(),
                justify,
                linebreak: LineBreak::WordOrCharacter,
                ..default()
            },
        ));
    }

    // ── Case 4: M3 Truncation & Ellipsis in Lists ──
    let case4_card = commands
        .spawn((
            ChildOf(left_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: M3::SURFACE_CONTAINER_LOW,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(case4_card),
        UNode::default(),
        UTextLabel {
            text: "4. M3 LIST ITEM TRUNCATION (Constrained container bounds)".to_string(),
            color: M3::PRIMARY,
            font_size: 13.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    let case4_list = commands
        .spawn((
            ChildOf(case4_card),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    let list_items = [
        (
            "Truncate::End",
            "Material Design guidelines and tokens for responsive layout architecture in mobile apps",
            UTextTruncateSide::End,
            M3::SECONDARY_CONTAINER,
        ),
        (
            "Truncate::Middle",
            "androidx.compose.material3.tokens.ColorSchemeKeyTokens_v3.1.2_final.jar",
            UTextTruncateSide::Middle,
            M3::TERTIARY_CONTAINER,
        ),
        (
            "Truncate::Start",
            "/data/user/0/com.google.android.material.demo/files/theme_cache/preferences.json",
            UTextTruncateSide::Start,
            M3::PRIMARY_CONTAINER,
        ),
    ];

    for (label, text, truncate_side, bg_color) in list_items {
        let item = commands
            .spawn((
                ChildOf(case4_list),
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::axes(12.0, 8.0),
                    background_color: M3::SURFACE_CONTAINER,
                    border_radius: UCornerRadius::all(10.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                    ..default()
                },
            ))
            .id();

        // M3 Pill Tag
        commands
            .spawn((
                ChildOf(item),
                UNode {
                    width: UVal::Px(120.0),
                    padding: USides::axes(8.0, 4.0),
                    background_color: bg_color,
                    border_radius: UCornerRadius::all(12.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|b| {
                b.spawn((
                    UNode::default(),
                    UTextLabel {
                        text: label.to_string(),
                        color: M3::ON_SURFACE,
                        font_size: 11.0,
                        font: font_body.clone(),
                        ..default()
                    },
                ));
            });

        // Truncated Text Container (explicitly bounded to prevent spilling into right column)
        let text_host = commands
            .spawn((
                ChildOf(item),
                UNode {
                    width: UVal::Px(450.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(text_host),
            UNode::default(),
            UTextLabel {
                text: text.to_string(),
                color: M3::ON_SURFACE,
                font_size: 13.0,
                font: font_body.clone(),
                overflow: UTextOverflow::Ellipsis,
                truncate_side,
                autosize: true,
                ..default()
            },
        ));
    }

    // ── Case 5: BiDi & Arabic in Material 3 ──
    let case5_card = commands
        .spawn((
            ChildOf(left_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: M3::SUCCESS_CONTAINER,
                border_radius: UCornerRadius::all(16.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(16.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(case5_card),
        UNode::default(),
        UTextLabel {
            text: "5. BIDI & ARABIC RTL TYPOGRAPHY (Material 3 localized layout)".to_string(),
            color: M3::ON_SUCCESS_CONTAINER,
            font_size: 12.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(case5_card),
        UNode {
            width: UVal::Percent(1.0),
            ..default()
        },
        UTextLabel {
            text: "تصميم Material You يدعم الخطوط العربية وتشكيل الحروف التلقائي واتجاه RTL بانسيابية تامة.".to_string(),
            color: Color::WHITE,
            font_size: 15.0,
            font: font_arabic.clone(),
            linebreak: LineBreak::WordOrCharacter,
            autosize: true,
            ..default()
        },
    ));

    // ────────────────────────────────────────────────────────────────────────
    // RIGHT COLUMN: M3 Interactive Sandbox & Live Inspector
    // ────────────────────────────────────────────────────────────────────────
    let right_column = commands
        .spawn((
            ChildOf(body),
            UNode {
                width: UVal::Px(632.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
                ..default()
            },
        ))
        .id();

    // ── M3 Sandbox Preview Card (Dynamic Container) ──
    let sandbox_outer_card = commands
        .spawn((
            ChildOf(right_column),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(230.0),
                padding: USides::all(20.0),
                background_color: M3::SURFACE_CONTAINER_LOW,
                border_radius: UCornerRadius::all(24.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(24.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    let dynamic_container = commands
        .spawn((
            ChildOf(sandbox_outer_card),
            DynamicContainer,
            UNode {
                width: UVal::Px(state.container_width),
                padding: USides::axes(20.0, 16.0),
                background_color: M3::PRIMARY_CONTAINER,
                border_radius: M3_SHAPE_PRESETS[state.corner_preset_idx].radius,
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_FOCUSED,
                width: 1.5,
                radius: M3_SHAPE_PRESETS[state.corner_preset_idx].radius,
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(dynamic_container),
        DynamicText,
        UTextLabel {
            text: M3_TEXT_PRESETS[state.text_preset_idx].1.to_string(),
            color: M3::ON_PRIMARY_CONTAINER,
            font_size: 15.0,
            font: font_body.clone(),
            linebreak: LineBreak::WordOrCharacter,
            overflow: UTextOverflow::Ellipsis,
            truncate_side: state.truncate_side,
            autosize: true,
            ..default()
        },
    ));

    // ── M3 Interactive Controls Card ──
    let controls_card = commands
        .spawn((
            ChildOf(right_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(18.0),
                background_color: M3::SURFACE_CONTAINER,
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(controls_card),
        UNode::default(),
        UTextLabel {
            text: "Interactive Controls".to_string(),
            color: M3::ON_SURFACE,
            font_size: 15.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    // Width Stepper Row
    let width_row = commands
        .spawn((
            ChildOf(controls_card),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .id();

    commands.spawn((
        ChildOf(width_row),
        UNode::default(),
        UTextLabel {
            text: "Card Width (180px - 480px):".to_string(),
            color: M3::ON_SURFACE_VARIANT,
            font_size: 13.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    let width_btn_group = commands
        .spawn((
            ChildOf(width_row),
            UNode::default(),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    // M3 Pill Stepper Minus Button
    commands
        .spawn((
            ChildOf(width_btn_group),
            UNode {
                padding: USides::axes(16.0, 8.0),
                background_color: M3::SURFACE_CONTAINER_HIGHEST,
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: M3::SURFACE_CONTAINER_HIGHEST,
                hovered: Color::srgb(0.28, 0.26, 0.32),
                pressed: Color::srgb(0.18, 0.16, 0.22),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut state: ResMut<TextShowcaseState>| {
                state.container_width = (state.container_width - 30.0).max(180.0);
            },
        )
        .with_children(|b| {
            b.spawn((
                UNode::default(),
                UTextLabel {
                    text: "- Narrow".to_string(),
                    color: M3::ON_SURFACE,
                    font_size: 13.0,
                    font: font_body.clone(),
                    ..default()
                },
            ));
        });

    // M3 Pill Stepper Plus Button
    commands
        .spawn((
            ChildOf(width_btn_group),
            UNode {
                padding: USides::axes(16.0, 8.0),
                background_color: M3::PRIMARY_CONTAINER,
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UInteraction::default(),
            UInteractionColors {
                normal: M3::PRIMARY_CONTAINER,
                hovered: Color::srgb(0.38, 0.28, 0.65),
                pressed: Color::srgb(0.24, 0.16, 0.45),
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .observe(
            |_click: On<Pointer<Click>>, mut state: ResMut<TextShowcaseState>| {
                state.container_width = (state.container_width + 30.0).min(480.0);
            },
        )
        .with_children(|b| {
            b.spawn((
                UNode::default(),
                UTextLabel {
                    text: "+ Expand".to_string(),
                    color: M3::ON_PRIMARY_CONTAINER,
                    font_size: 13.0,
                    font: font_body.clone(),
                    ..default()
                },
            ));
        });

    // M3 Tonal Action Chips Row
    let action_row = commands
        .spawn((
            ChildOf(controls_card),
            UNode {
                width: UVal::Percent(1.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    let mut add_m3_action_button =
        |parent: Entity, label: &'static str, on_click: fn(&mut TextShowcaseState)| {
            commands
                .spawn((
                    ChildOf(parent),
                    UNode {
                        padding: USides::axes(14.0, 9.0),
                        background_color: M3::SURFACE_CONTAINER_HIGH,
                        border_radius: UCornerRadius::all(20.0),
                        ..default()
                    },
                    UBorder {
                        color: M3::OUTLINE_VARIANT,
                        width: 1.0,
                        radius: UCornerRadius::all(20.0),
                        offset: 0.0,
                    },
                    UInteraction::default(),
                    UInteractionColors {
                        normal: M3::SURFACE_CONTAINER_HIGH,
                        hovered: Color::srgb(0.25, 0.23, 0.29),
                        pressed: Color::srgb(0.15, 0.14, 0.18),
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        ..default()
                    },
                ))
                .observe(
                    move |_click: On<Pointer<Click>>, mut state: ResMut<TextShowcaseState>| {
                        on_click(&mut state);
                    },
                )
                .with_children(|b| {
                    b.spawn((
                        UNode::default(),
                        UTextLabel {
                            text: label.to_string(),
                            color: M3::ON_SURFACE,
                            font_size: 12.0,
                            font: font_body.clone(),
                            ..default()
                        },
                    ));
                });
        };

    add_m3_action_button(action_row, "Next Text Preset", |s| {
        s.text_preset_idx = (s.text_preset_idx + 1) % M3_TEXT_PRESETS.len();
    });

    add_m3_action_button(action_row, "Next Truncation", |s| {
        s.truncate_side = match s.truncate_side {
            UTextTruncateSide::End => UTextTruncateSide::Middle,
            UTextTruncateSide::Middle => UTextTruncateSide::Start,
            UTextTruncateSide::Start => UTextTruncateSide::Auto,
            UTextTruncateSide::Auto => UTextTruncateSide::End,
        };
    });

    add_m3_action_button(action_row, "Next M3 Shape", |s| {
        s.corner_preset_idx = (s.corner_preset_idx + 1) % M3_SHAPE_PRESETS.len();
    });

    // ── M3 Live Inspector Sheet ──
    let inspector_card = commands
        .spawn((
            ChildOf(right_column),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(18.0),
                background_color: M3::SURFACE_CONTAINER_LOW,
                border_radius: UCornerRadius::all(20.0),
                ..default()
            },
            UBorder {
                color: M3::OUTLINE_VARIANT,
                width: 1.0,
                radius: UCornerRadius::all(20.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
                ..default()
            },
        ))
        .id();

    let inspector_header = commands
        .spawn((
            ChildOf(inspector_card),
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(0.0, 4.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 10.0,
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            ChildOf(inspector_header),
            UNode {
                padding: USides::axes(10.0, 4.0),
                background_color: M3::SECONDARY_CONTAINER,
                border_radius: UCornerRadius::all(12.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((
                UNode::default(),
                UTextLabel {
                    text: "M3 INSPECTOR".to_string(),
                    color: M3::ON_SECONDARY_CONTAINER,
                    font_size: 11.0,
                    font: font_body.clone(),
                    ..default()
                },
            ));
        });

    commands.spawn((
        ChildOf(inspector_header),
        UNode::default(),
        UTextLabel {
            text: "Real-time Metrics & Constraints".to_string(),
            color: M3::ON_SURFACE,
            font_size: 14.0,
            font: font_body.clone(),
            ..default()
        },
    ));

    // Formatted key-value metric rows in pure M3 style
    let metrics_to_spawn = [
        (InspectorMetric::ShapeToken, "M3 Shape Token"),
        (InspectorMetric::ContainerWidth, "Container Width"),
        (InspectorMetric::CornerRadii, "Corner Radii"),
        (InspectorMetric::MeasuredSize, "Measured Text Size"),
        (InspectorMetric::LineCount, "Line Count & Fit"),
        (InspectorMetric::ParentBound, "Parent Width Bound"),
        (InspectorMetric::OverflowPolicy, "Overflow Policy"),
    ];

    for (metric, label_str) in metrics_to_spawn {
        let row = commands
            .spawn((
                ChildOf(inspector_card),
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::axes(12.0, 6.0),
                    background_color: M3::SURFACE_CONTAINER,
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(row),
            UNode::default(),
            UTextLabel {
                text: label_str.to_string(),
                color: M3::ON_SURFACE_VARIANT,
                font_size: 12.0,
                font: font_body.clone(),
                ..default()
            },
        ));

        let badge = commands
            .spawn((
                ChildOf(row),
                UNode {
                    padding: USides::axes(10.0, 4.0),
                    background_color: M3::SURFACE_CONTAINER_HIGH,
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(badge),
            UNode::default(),
            InspectorMetricText(metric),
            UTextLabel {
                text: "-".to_string(),
                color: M3::PRIMARY,
                font_size: 12.0,
                font: font_body.clone(),
                ..default()
            },
        ));
    }
}

// ── Update Dynamic Box System ───────────────────────────────────────────────

fn update_dynamic_box(
    state: Res<TextShowcaseState>,
    theme: Res<Theme>,
    mut container_q: Query<(&mut UNode, &mut UBorder), With<DynamicContainer>>,
    mut text_q: Query<&mut UTextLabel, With<DynamicText>>,
) {
    if !state.is_changed() {
        return;
    }

    let current_corner = &M3_SHAPE_PRESETS[state.corner_preset_idx];

    for (mut node, mut border) in &mut container_q {
        let target_w = UVal::Px(state.container_width);
        if node.width != target_w {
            node.width = target_w;
        }
        if node.border_radius != current_corner.radius {
            node.border_radius = current_corner.radius;
        }
        if border.radius != current_corner.radius {
            border.radius = current_corner.radius;
        }
    }

    let is_arabic = state.text_preset_idx == 2;
    let selected_font = if is_arabic {
        theme.text.font.free_serif.clone()
    } else {
        theme.text.font.inter_regular.clone()
    };

    let target_text = M3_TEXT_PRESETS[state.text_preset_idx].1;
    for mut text_label in &mut text_q {
        if text_label.text != target_text {
            text_label.text = target_text.to_string();
        }
        if text_label.truncate_side != state.truncate_side {
            text_label.truncate_side = state.truncate_side;
        }
        if text_label.font != selected_font {
            text_label.font = selected_font.clone();
        }
    }
}

// ── Update Inspector HUD System ─────────────────────────────────────────────

fn update_inspector_hud(
    state: Res<TextShowcaseState>,
    cache_q: Query<Ref<UTextLabelLayoutCache>, With<DynamicText>>,
    mut metric_q: Query<(&InspectorMetricText, &mut UTextLabel)>,
) {
    let Some(cache) = cache_q.iter().next() else {
        return;
    };

    if !state.is_changed() && !cache.is_changed() {
        return;
    }

    let current_corner = &M3_SHAPE_PRESETS[state.corner_preset_idx];
    let r = current_corner.radius;

    for (metric, mut label) in &mut metric_q {
        let new_text = match metric.0 {
            InspectorMetric::ShapeToken => {
                format!("{} ({})", current_corner.name, current_corner.token)
            }
            InspectorMetric::ContainerWidth => {
                format!("{:.1} px", state.container_width)
            }
            InspectorMetric::CornerRadii => {
                format!(
                    "TL: {:.0}px | TR: {:.0}px | BR: {:.0}px | BL: {:.0}px",
                    r.top_left, r.top_right, r.bottom_right, r.bottom_left
                )
            }
            InspectorMetric::MeasuredSize => {
                format!(
                    "{:.1} x {:.1} px",
                    cache.measured_size.x, cache.measured_size.y
                )
            }
            InspectorMetric::LineCount => {
                format!(
                    "{} line(s) - {}",
                    cache.line_count,
                    if cache.overflowed {
                        "Truncating"
                    } else {
                        "Optimal Fit"
                    }
                )
            }
            InspectorMetric::ParentBound => cache
                .parent_bound_width
                .map(|w| format!("{:.1} px", w))
                .unwrap_or_else(|| "Unbounded".into()),
            InspectorMetric::OverflowPolicy => {
                format!("Ellipsis ({:?})", state.truncate_side)
            }
        };

        if label.text != new_text {
            label.text = new_text;
        }
    }
}
