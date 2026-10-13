//! Rich Typography, SDF Text Outlines & Inline Multi-Span Architecture Demo.
//!
//! Visual Theme: Univis Design Lab & Typography Studio.
//!
//! Features showcased:
//! 1. Real-time SDF text outline/stroke visual effects with zero extra draw calls (`outline_width`, `outline_color`).
//! 2. Inline rich text spans (`UTextSpan`) supporting mixed colors, font sizes, and fonts within a single paragraph.
//! 3. Inline vector iconography from Lucide font embedded directly inside text spans.
//! 4. Bilingual dual-script spans (Arabic + Latin) seamlessly laid out in the same label.
//! 5. Selectable and copyable text primitive (`USelectableText`, `UCopyText`) with interactive copy button and visual toast.

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum OutlineThickness {
    None,
    Subtle,
    Medium,
    Bold,
}

impl OutlineThickness {
    fn px(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Subtle => 1.5,
            Self::Medium => 3.0,
            Self::Bold => 5.0,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::None => "0.0px (Off)",
            Self::Subtle => "1.5px (Subtle)",
            Self::Medium => "3.0px (Medium)",
            Self::Bold => "5.0px (Bold)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum OutlineColorChoice {
    Gold,
    Cyan,
    Crimson,
    Emerald,
}

impl OutlineColorChoice {
    fn color(&self) -> Color {
        match self {
            Self::Gold => Color::srgb(0.96, 0.78, 0.28),
            Self::Cyan => Color::srgb(0.20, 0.85, 0.98),
            Self::Crimson => Color::srgb(0.95, 0.25, 0.40),
            Self::Emerald => Color::srgb(0.25, 0.92, 0.55),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Gold => "Amber Gold",
            Self::Cyan => "Electric Cyan",
            Self::Crimson => "Neon Crimson",
            Self::Emerald => "Emerald Green",
        }
    }
}

#[derive(Resource)]
struct TypographyLabState {
    thickness: OutlineThickness,
    color_choice: OutlineColorChoice,
    toast_timer: f32,
    toast_message: String,
}

impl Default for TypographyLabState {
    fn default() -> Self {
        Self {
            thickness: OutlineThickness::Medium,
            color_choice: OutlineColorChoice::Gold,
            toast_timer: 0.0,
            toast_message: String::new(),
        }
    }
}

#[derive(Component, Clone, Default)]
struct HeroOutlineText;

#[derive(Component, Clone, Default)]
struct OutlineThicknessLabel;

#[derive(Component, Clone, Default)]
struct OutlineColorLabel;

#[derive(Component, Clone, Default)]
struct ToastBadge;

#[derive(Component, Clone, Default)]
struct SelectableCodeSnippet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Rich Typography & SDF Outlines Studio (BSN)".into(),
                resolution: (1280, 860).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<TypographyLabState>()
        .add_systems(Startup, setup_typography_lab)
        .add_systems(Update, (sync_typography_lab, update_toast_timer))
        .run();
}

fn setup_typography_lab(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(typography_root_scene(&theme));
}

fn typography_root_scene(theme: &Theme) -> impl Scene {
    let body_font = theme.text.font.inter_regular.clone();
    let serif_font = theme.text.font.free_serif.clone();
    let arabic_font = theme.text.font.noto_sans_arabic_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Px(1280.0),
            height: UVal::Px(860.0),
            background_color: Color::srgb(0.06, 0.07, 0.09),
            padding: USides::axes(28.0, 18.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Start,
            gap: 12.0,
        }
        Children [
            (studio_header(body_font.clone(), icon_font.clone())),
            (studio_divider()),
            (studio_columns(body_font.clone(), serif_font, arabic_font, icon_font)),
            (studio_footer(body_font)),
        ]
    }
}

/// 1. Top Header & Title Bar
fn studio_header(body_font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1224.0),
            height: UVal::Px(46.0),
            padding: USides::axes(0.0, 2.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Left branding
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(36.0),
                            height: UVal::Px(36.0),
                            background_color: Color::srgb(0.12, 0.16, 0.24),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        UBorder {
                            color: Color::srgb(0.24, 0.44, 0.85),
                            width: 1.5,
                            radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (
                                UText {
                                    text: { Icon::TYPE.to_string() },
                                    font_size: 18.0,
                                    font: { icon_font },
                                    color: Color::srgb(0.38, 0.72, 0.98),
                                }
                            )
                        ]
                    ),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 1.0,
                        }
                        Children [
                            (
                                UText {
                                    text: { "UNIVIS TYPOGRAPHY STUDIO".to_string() },
                                    font_size: 16.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.96, 0.97, 0.99),
                                }
                            ),
                            (
                                UText {
                                    text: { "SDF Outlines, Multi-Span Inline Formatting & Selectable Text Architecture".to_string() },
                                    font_size: 11.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.52, 0.58, 0.68),
                                }
                            ),
                        ]
                    )
                ]
            ),

            // Right badges & Toast Area
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    (
                        ToastBadge
                        UNode {
                            height: UVal::Px(28.0),
                            background_color: Color::srgba(0.12, 0.32, 0.22, 0.0),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(10.0, 0.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (
                                UText {
                                    text: { String::new() },
                                    font_size: 11.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.35, 0.92, 0.55),
                                }
                            )
                        ]
                    ),
                    (
                        UNode {
                            height: UVal::Px(28.0),
                            background_color: Color::srgb(0.12, 0.14, 0.18),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(10.0, 0.0),
                        }
                        UBorder {
                            color: Color::srgb(0.20, 0.24, 0.32),
                            width: 1.0,
                            radius: UCornerRadius::all(6.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (
                                UText {
                                    text: { "BEVY 0.19 + PARLEY + SDF".to_string() },
                                    font_size: 10.5,
                                    font: { body_font },
                                    color: Color::srgb(0.68, 0.74, 0.84),
                                }
                            )
                        ]
                    ),
                ]
            ),
        ]
    }
}

fn studio_divider() -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1224.0),
            height: UVal::Px(1.0),
            background_color: Color::srgb(0.14, 0.17, 0.23),
        }
    }
}

/// 2. Two Main Columns (Left: SDF Outlines & Selection, Right: Multi-Span & Dual Script)
fn studio_columns(
    body_font: Handle<Font>,
    serif_font: Handle<Font>,
    arabic_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1224.0),
            height: UVal::Px(716.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 14.0,
        }
        Children [
            (studio_left_column(body_font.clone(), icon_font.clone())),
            (studio_right_column(body_font, serif_font, arabic_font, icon_font)),
        ]
    }
}

/// Left Column: Real-time SDF Outlines Laboratory & Selectable Text Block (605px)
fn studio_left_column(body_font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(605.0),
            height: UVal::Px(716.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            // Card A: Real-time Outline Laboratory
            (outline_lab_card(body_font.clone(), icon_font.clone())),
            // Card B: Selectable & Copyable Text Block
            (selectable_card(body_font, icon_font)),
        ]
    }
}

/// Card A: Real-Time Outline Playground Card
fn outline_lab_card(body_font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(605.0),
            height: UVal::Px(450.0),
            background_color: Color::srgb(0.09, 0.11, 0.15),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgb(0.18, 0.22, 0.29),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            // Card Header
            (
                UNode {
                    width: UVal::Px(573.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 8.0,
                        }
                        Children [
                            (
                                UText {
                                    text: { Icon::PEN_TOOL.to_string() },
                                    font_size: 15.0,
                                    font: { icon_font.clone() },
                                    color: Color::srgb(0.96, 0.78, 0.28),
                                }
                            ),
                            (
                                UText {
                                    text: { "REAL-TIME SDF TEXT OUTLINES".to_string() },
                                    font_size: 13.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.92, 0.94, 0.98),
                                }
                            ),
                        ]
                    ),
                    (
                        OutlineThicknessLabel
                        UText {
                            text: { "Stroke: 3.0px (Medium)".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.65, 0.72, 0.82),
                        }
                    )
                ]
            ),

            // Hero Outline Display Area
            (
                UNode {
                    width: UVal::Px(573.0),
                    height: UVal::Px(190.0),
                    background_color: Color::srgb(0.05, 0.06, 0.08),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: Color::srgb(0.14, 0.17, 0.22),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (
                        HeroOutlineText
                        UText {
                            text: { "UNIVIS 2026".to_string() },
                            font_size: 44.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.08, 0.09, 0.12),
                            outline_width: 3.0,
                            outline_color: Color::srgb(0.96, 0.78, 0.28),
                        }
                    ),
                    (
                        UText {
                            text: { "Zero Extra Draw Calls * Single-Pass SDF Edge Distance in WGSL".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.55, 0.60, 0.70),
                        }
                    ),
                ]
            ),

            // Thickness Interactive Controls
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    (
                        UText {
                            text: { "OUTLINE STROKE THICKNESS:".to_string() },
                            font_size: 10.5,
                            font: { body_font.clone() },
                            color: Color::srgb(0.55, 0.60, 0.70),
                        }
                    ),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            gap: 8.0,
                        }
                        Children [
                            (outline_thickness_btn("None (0px)", OutlineThickness::None, body_font.clone())),
                            (outline_thickness_btn("Subtle (1.5px)", OutlineThickness::Subtle, body_font.clone())),
                            (outline_thickness_btn("Medium (3.0px)", OutlineThickness::Medium, body_font.clone())),
                            (outline_thickness_btn("Bold (5.0px)", OutlineThickness::Bold, body_font.clone())),
                        ]
                    )
                ]
            ),

            // Color Interactive Controls
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::SpaceBetween,
                        }
                        Children [
                            (
                                UText {
                                    text: { "OUTLINE STROKE COLOR:".to_string() },
                                    font_size: 10.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.55, 0.60, 0.70),
                                }
                            ),
                            (
                                OutlineColorLabel
                                UText {
                                    text: { "Amber Gold".to_string() },
                                    font_size: 10.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.96, 0.78, 0.28),
                                }
                            )
                        ]
                    ),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            gap: 8.0,
                        }
                        Children [
                            (outline_color_btn("Gold", OutlineColorChoice::Gold, Color::srgb(0.96, 0.78, 0.28), body_font.clone())),
                            (outline_color_btn("Cyan", OutlineColorChoice::Cyan, Color::srgb(0.20, 0.85, 0.98), body_font.clone())),
                            (outline_color_btn("Crimson", OutlineColorChoice::Crimson, Color::srgb(0.95, 0.25, 0.40), body_font.clone())),
                            (outline_color_btn("Emerald", OutlineColorChoice::Emerald, Color::srgb(0.25, 0.92, 0.55), body_font.clone())),
                        ]
                    )
                ]
            ),

            // Tech specs callout
            (
                UNode {
                    width: UVal::Px(573.0),
                    background_color: Color::srgb(0.07, 0.08, 0.11),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::axes(10.0, 6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        UText {
                            text: { "Uniform: vec4 outline_color + f32 outline_width | 16-byte aligned".to_string() },
                            font_size: 10.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.46, 0.50, 0.60),
                        }
                    ),
                    (
                        UText {
                            text: { "Pipeline: text_label_sdf.wgsl".to_string() },
                            font_size: 10.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.38, 0.65, 0.85),
                        }
                    )
                ]
            )
        ]
    }
}

fn outline_thickness_btn(label: &'static str, thickness: OutlineThickness, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(136.0),
            height: UVal::Px(28.0),
            background_color: Color::srgb(0.13, 0.16, 0.22),
            border_radius: UCornerRadius::all(5.0),
            padding: USides::axes(4.0, 0.0),
        }
        UBorder {
            color: Color::srgb(0.22, 0.27, 0.36),
            width: 1.0,
            radius: UCornerRadius::all(5.0),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        UInteraction
        UInteractionColors {
            normal: Color::srgb(0.13, 0.16, 0.22),
            hovered: Color::srgb(0.20, 0.26, 0.38),
            pressed: Color::srgb(0.28, 0.36, 0.52),
        }
        Children [
            (
                UText {
                    text: { label.to_string() },
                    font_size: 11.0,
                    font: { body_font },
                    color: Color::srgb(0.85, 0.88, 0.94),
                }
            )
        ]
        on(move |_event: On<Pointer<Click>>, mut state: ResMut<TypographyLabState>| {
            state.thickness = thickness;
        })
    }
}

fn outline_color_btn(
    label: &'static str,
    choice: OutlineColorChoice,
    swatch: Color,
    body_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(136.0),
            height: UVal::Px(28.0),
            background_color: Color::srgb(0.13, 0.16, 0.22),
            border_radius: UCornerRadius::all(5.0),
            padding: USides::axes(4.0, 0.0),
        }
        UBorder {
            color: swatch,
            width: 1.0,
            radius: UCornerRadius::all(5.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        UInteraction
        UInteractionColors {
            normal: Color::srgb(0.13, 0.16, 0.22),
            hovered: Color::srgb(0.20, 0.26, 0.38),
            pressed: Color::srgb(0.28, 0.36, 0.52),
        }
        Children [
            (
                UNode {
                    width: UVal::Px(8.0),
                    height: UVal::Px(8.0),
                    background_color: swatch,
                    border_radius: UCornerRadius::all(4.0),
                }
            ),
            (
                UText {
                    text: { label.to_string() },
                    font_size: 11.0,
                    font: { body_font },
                    color: Color::srgb(0.88, 0.90, 0.95),
                }
            )
        ]
        on(move |_event: On<Pointer<Click>>, mut state: ResMut<TypographyLabState>| {
            state.color_choice = choice;
        })
    }
}

/// Card B: Selectable & Copyable Text Block (254px)
fn selectable_card(body_font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(605.0),
            height: UVal::Px(254.0),
            background_color: Color::srgb(0.09, 0.11, 0.15),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgb(0.18, 0.22, 0.29),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 10.0,
        }
        Children [
            // Header with Copy Button
            (
                UNode {
                    width: UVal::Px(573.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 8.0,
                        }
                        Children [
                            (
                                UText {
                                    text: { Icon::COPY.to_string() },
                                    font_size: 14.0,
                                    font: { icon_font.clone() },
                                    color: Color::srgb(0.38, 0.72, 0.98),
                                }
                            ),
                            (
                                UText {
                                    text: { "SELECTABLE & COPYABLE TEXT (USelectableText)".to_string() },
                                    font_size: 12.0,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.92, 0.94, 0.98),
                                }
                            ),
                        ]
                    ),
                    (
                        UNode {
                            height: UVal::Px(26.0),
                            background_color: Color::srgb(0.14, 0.18, 0.26),
                            border_radius: UCornerRadius::all(5.0),
                            padding: USides::axes(10.0, 0.0),
                        }
                        UBorder {
                            color: Color::srgb(0.25, 0.35, 0.52),
                            width: 1.0,
                            radius: UCornerRadius::all(5.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        UInteraction
                        UInteractionColors {
                            normal: Color::srgb(0.14, 0.18, 0.26),
                            hovered: Color::srgb(0.22, 0.28, 0.42),
                            pressed: Color::srgb(0.30, 0.38, 0.58),
                        }
                        Children [
                            (
                                UText {
                                    text: { Icon::COPY.to_string() },
                                    font_size: 11.0,
                                    font: { icon_font },
                                    color: Color::srgb(0.85, 0.90, 0.98),
                                }
                            ),
                            (
                                UText {
                                    text: { "Copy Code Snippet".to_string() },
                                    font_size: 10.5,
                                    font: { body_font.clone() },
                                    color: Color::srgb(0.85, 0.90, 0.98),
                                }
                            ),
                        ]
                        on(|_event: On<Pointer<Click>>,
                            mut state: ResMut<TypographyLabState>,
                            query: Query<&UText, With<SelectableCodeSnippet>>,
                            mut commands: Commands| {
                            if let Ok(text_label) = query.single() {
                                commands.trigger(UCopyText {
                                    text: text_label.text.clone(),
                                });
                                state.toast_timer = 2.0;
                                state.toast_message = "Snippet copied to clipboard!".to_string();
                            }
                        })
                    )
                ]
            ),

            // Selectable Code Snippet Box
            (
                UNode {
                    width: UVal::Px(573.0),
                    height: UVal::Px(136.0),
                    background_color: Color::srgb(0.05, 0.06, 0.08),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::all(12.0),
                }
                UBorder {
                    color: Color::srgb(0.15, 0.19, 0.26),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                }
                Children [
                    (
                        SelectableCodeSnippet
                        USelectableText::default()
                        UText {
                            text: {
                                "// Univis Rich Text & Real-Time Outline Setup\n\
                                let label = UTextLabel::rich([\n\
                                    UTextSpan::new(\"Hello \").with_color(Color::WHITE),\n\
                                    UTextSpan::new(\"Univis!\").with_color(Color::GOLD),\n\
                                ])\n\
                                .with_outline(2.5, Color::srgb(0.9, 0.7, 0.2));".to_string()
                            },
                            font_size: 12.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.80, 0.85, 0.92),
                        }
                    )
                ]
            ),

            // Information note
            (
                UText {
                    text: { "Tip: Click and drag or trigger programmatic selection. USelectableText tracks active ranges with custom highlight colors.".to_string() },
                    font_size: 10.5,
                    font: { body_font },
                    color: Color::srgb(0.48, 0.54, 0.64),
                }
            )
        ]
    }
}

/// Right Column: Inline Multi-Span Formatting & Bilingual Typography (605px)
fn studio_right_column(
    body_font: Handle<Font>,
    serif_font: Handle<Font>,
    arabic_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(605.0),
            height: UVal::Px(716.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            // Card C: Rich Multi-Span Paragraph
            (rich_spans_card(body_font.clone(), serif_font, icon_font.clone())),
            // Card D: Bilingual Dual-Script Typography
            (bilingual_card(body_font, arabic_font, icon_font)),
        ]
    }
}

/// Card C: Rich Text Multi-Span Demonstration (410px)
fn rich_spans_card(
    body_font: Handle<Font>,
    serif_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(605.0),
            height: UVal::Px(410.0),
            background_color: Color::srgb(0.09, 0.11, 0.15),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgb(0.18, 0.22, 0.29),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            // Header
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (
                        UText {
                            text: { Icon::FEATHER.to_string() },
                            font_size: 15.0,
                            font: { icon_font.clone() },
                            color: Color::srgb(0.38, 0.82, 0.98),
                        }
                    ),
                    (
                        UText {
                            text: { "INLINE MULTI-SPAN FORMATTING (UTextSpan)".to_string() },
                            font_size: 13.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.92, 0.94, 0.98),
                        }
                    ),
                ]
            ),

            // Main Rich Text Paragraph
            (
                UNode {
                    width: UVal::Px(573.0),
                    background_color: Color::srgb(0.05, 0.06, 0.08),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: Color::srgb(0.14, 0.17, 0.22),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                Children [
                    (
                        UText {
                            wrap_mode: UTextWrapMode::WrapToParent,
                            overflow: UTextOverflow::Visible,
                            font: { body_font.clone() },
                            font_size: 13.5,
                            spans: {
                                vec![
                                    UTextSpan::new("Univis UI introduces ").with_color(Color::srgb(0.84, 0.88, 0.94)),
                                    UTextSpan::new("Rich Text Inline Spans").with_color(Color::srgb(0.96, 0.78, 0.28)),
                                    UTextSpan::new(", enabling seamless composition of ").with_color(Color::srgb(0.84, 0.88, 0.94)),
                                    UTextSpan::new("vibrant vertex colors").with_color(Color::srgb(0.25, 0.92, 0.55)),
                                    UTextSpan::new(", contrasting ").with_color(Color::srgb(0.84, 0.88, 0.94)),
                                    UTextSpan::new("FreeSerif font styles").with_font(serif_font).with_color(Color::srgb(0.95, 0.70, 0.50)),
                                    UTextSpan::new(", and ").with_color(Color::srgb(0.84, 0.88, 0.94)),
                                    UTextSpan::new(format!("{} Vector Icons ", Icon::FEATHER)).with_font(icon_font.clone()).with_color(Color::srgb(0.38, 0.82, 0.98)),
                                    UTextSpan::new("right inside the flow of running prose—all packaged into a ").with_color(Color::srgb(0.84, 0.88, 0.94)),
                                    UTextSpan::new("single unified draw call").with_color(Color::srgb(0.98, 0.42, 0.55)),
                                    UTextSpan::new("! When resizing or reflowing, Bevy's Parley layout measures all sections with pixel-perfect accuracy.").with_color(Color::srgb(0.70, 0.75, 0.84)),
                                ]
                            },
                        }
                    )
                ]
            ),

            // Feature Highlights Pills
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    (rich_feature_pill(Icon::CHECK, "Mesh::ATTRIBUTE_COLOR: Per-vertex tinting without breaking batching", Color::srgb(0.25, 0.92, 0.55), icon_font.clone(), body_font.clone())),
                    (rich_feature_pill(Icon::GRID_3X3, "Multi-section Parley text layout with font texture atlas caching", Color::srgb(0.38, 0.82, 0.98), icon_font.clone(), body_font.clone())),
                    (rich_feature_pill(Icon::HEART, "Zero runtime overhead for plain labels, instant scaling for rich text", Color::srgb(0.96, 0.78, 0.28), icon_font, body_font.clone())),
                ]
            )
        ]
    }
}

fn rich_feature_pill(
    icon: &'static str,
    text: &'static str,
    accent: Color,
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(573.0),
            height: UVal::Px(28.0),
            background_color: Color::srgb(0.07, 0.08, 0.11),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(10.0, 0.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 8.0,
        }
        Children [
            (
                UText {
                    text: { icon.to_string() },
                    font_size: 12.0,
                    font: { icon_font },
                    color: accent,
                }
            ),
            (
                UText {
                    text: { text.to_string() },
                    font_size: 10.5,
                    font: { body_font },
                    color: Color::srgb(0.72, 0.76, 0.85),
                }
            )
        ]
    }
}

/// Card D: Bilingual Dual-Script Typography (294px)
fn bilingual_card(
    body_font: Handle<Font>,
    arabic_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(605.0),
            height: UVal::Px(294.0),
            background_color: Color::srgb(0.09, 0.11, 0.15),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgb(0.18, 0.22, 0.29),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 10.0,
        }
        Children [
            // Header
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (
                        UText {
                            text: { Icon::GLOBE.to_string() },
                            font_size: 15.0,
                            font: { icon_font },
                            color: Color::srgb(0.40, 0.85, 0.65),
                        }
                    ),
                    (
                        UText {
                            text: { "BILINGUAL DUAL-SCRIPT MULTI-SPAN (ARABIC + LATIN)".to_string() },
                            font_size: 12.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.92, 0.94, 0.98),
                        }
                    ),
                ]
            ),

            // Bilingual paragraph box
            (
                UNode {
                    width: UVal::Px(573.0),
                    height: UVal::Px(160.0),
                    background_color: Color::srgb(0.05, 0.06, 0.08),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: Color::srgb(0.14, 0.17, 0.22),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    justify_content: UJustifyContent::Center,
                    gap: 8.0,
                }
                Children [
                    (
                        UText {
                            wrap_mode: UTextWrapMode::WrapToParent,
                            overflow: UTextOverflow::Visible,
                            font: { arabic_font.clone() },
                            font_size: 14.5,
                            spans: {
                                vec![
                                    UTextSpan::new("نظام النصوص المتطور في ").with_font(arabic_font.clone()).with_color(Color::srgb(0.96, 0.80, 0.35)),
                                    UTextSpan::new("Univis UI").with_font(body_font.clone()).with_color(Color::srgb(0.38, 0.82, 0.98)),
                                    UTextSpan::new(" — Next-Gen Typography Architecture for Bevy.").with_font(body_font.clone()).with_color(Color::srgb(0.70, 0.78, 0.90)),
                                ]
                            },
                        }
                    ),
                    (
                        UText {
                            wrap_mode: UTextWrapMode::WrapToParent,
                            overflow: UTextOverflow::Visible,
                            font: { arabic_font.clone() },
                            font_size: 13.0,
                            spans: {
                                vec![
                                    UTextSpan::new("يدعم الالتفاف التلقائي، والحدود الخارجية ").with_font(arabic_font.clone()).with_color(Color::srgb(0.82, 0.85, 0.90)),
                                    UTextSpan::new("(Outlines)").with_font(body_font.clone()).with_color(Color::srgb(0.96, 0.78, 0.28)),
                                    UTextSpan::new("، والأيقونات المدمجة ").with_font(arabic_font.clone()).with_color(Color::srgb(0.35, 0.90, 0.60)),
                                    UTextSpan::new("مع الحفاظ على أعلى درجات الأداء والدقة.").with_font(arabic_font).with_color(Color::srgb(0.82, 0.85, 0.90)),
                                ]
                            },
                        }
                    ),
                ]
            ),

            // Colophon annotation
            (
                UText {
                    text: { "Rendered with Noto Sans Arabic Regular + Inter Regular embedded fonts".to_string() },
                    font_size: 10.5,
                    font: { body_font },
                    color: Color::srgb(0.48, 0.54, 0.64),
                }
            )
        ]
    }
}

/// 3. Footer Bar
fn studio_footer(body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(1224.0),
            height: UVal::Px(24.0),
            padding: USides::axes(2.0, 0.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UText {
                    text: { "Univis UI v0.3.1 * High-Performance SDF Hybrid World-Space UI Framework".to_string() },
                    font_size: 10.5,
                    font: { body_font.clone() },
                    color: Color::srgb(0.44, 0.48, 0.56),
                }
            ),
            (
                UText {
                    text: { "Architecture Path 1: Advanced Text Pipeline Complete".to_string() },
                    font_size: 10.5,
                    font: { body_font },
                    color: Color::srgb(0.35, 0.75, 0.55),
                }
            ),
        ]
    }
}

fn sync_typography_lab(
    state: Res<TypographyLabState>,
    mut hero_query: Query<&mut UText, With<HeroOutlineText>>,
    mut thickness_query: Query<&mut UText, (With<OutlineThicknessLabel>, Without<HeroOutlineText>, Without<OutlineColorLabel>)>,
    mut color_label_query: Query<&mut UText, (With<OutlineColorLabel>, Without<HeroOutlineText>, Without<OutlineThicknessLabel>)>,
    mut toast_query: Query<(&mut UNode, &Children), With<ToastBadge>>,
    mut toast_text_query: Query<&mut UText, (Without<HeroOutlineText>, Without<OutlineThicknessLabel>, Without<OutlineColorLabel>)>,
) {
    if state.is_changed() {
        let width = state.thickness.px();
        let color = state.color_choice.color();

        for mut hero in hero_query.iter_mut() {
            hero.outline_width = width;
            hero.outline_color = color;
        }

        for mut label in thickness_query.iter_mut() {
            label.text = format!("Stroke: {}", state.thickness.label());
        }

        for mut label in color_label_query.iter_mut() {
            label.text = state.color_choice.name().to_string();
            label.color = color;
        }

        for (mut toast_node, children) in toast_query.iter_mut() {
            if state.toast_timer > 0.0 {
                toast_node.background_color = Color::srgba(0.12, 0.32, 0.22, 0.85);
                for child in children.iter() {
                    if let Ok(mut text) = toast_text_query.get_mut(child) {
                        text.text = state.toast_message.clone();
                    }
                }
            } else {
                toast_node.background_color = Color::srgba(0.12, 0.32, 0.22, 0.0);
                for child in children.iter() {
                    if let Ok(mut text) = toast_text_query.get_mut(child) {
                        text.text.clear();
                    }
                }
            }
        }
    }
}

fn update_toast_timer(time: Res<Time>, mut state: ResMut<TypographyLabState>) {
    if state.toast_timer > 0.0 {
        state.toast_timer = (state.toast_timer - time.delta_secs()).max(0.0);
    }
}
