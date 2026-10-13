//! Creative Studio Material Palette - Demonstrating Visual SDF Styling with BSN.
//!
//! Visual Theme: Professional digital vector art & UI design studio.
//!
//! Visual features demonstrated:
//! - High-definition typography via `Theme` and embedded Lucide vector icons
//!   (`Icon::LAYOUT_GRID`, `Icon::MOUSE_POINTER`, `Icon::PEN_TOOL`, `Icon::SQUARE`,
//!    `Icon::TYPE`, `Icon::PIPETTE`, `Icon::SLIDERS_VERTICAL`, `Icon::ZOOM_IN`,
//!    `Icon::SUNSET`, `Icon::SUN`, `Icon::BOX`, `Icon::APERTURE`, `Icon::CPU`).
//! - Left creative toolbar with active tool states and vector iconography.
//! - Multi-stop linear gradients: `UGradient::linear` with custom color stops.
//! - Radial lighting fills: `UGradient::radial`.
//! - Geometric corner styling: `UShapeMode::Round` vs `UShapeMode::Cut` (chamfered bevels).
//! - Soft realistic drop shadows: `UShadow { offset, blur, spread, .. }`.
//! - Real-time SDF text outlines & clean typography with zero distortion.
//! - Dynamic gradient animation system.

use bevy::prelude::*;
use core::f32::consts::FRAC_PI_4;
use univis_ui::prelude::*;

#[derive(Component, Clone, Copy, Default)]
struct StudioGradientSweep;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Creative Studio Palette (Visual SDF & BSN)".into(),
                resolution: (1280, 820).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup_creative_studio)
        .add_systems(Update, animate_studio_swatches)
        .run();
}

fn setup_creative_studio(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(studio_palette_scene(&theme));
}

/// Root scene for the Creative Studio inspector.
fn studio_palette_scene(theme: &Theme) -> impl Scene {
    let text_font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.06, 0.06, 0.08),
            padding: USides::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 14.0,
        }
        Children [
            (studio_header(icon_font.clone(), text_font.clone())),
            (studio_workspace_row(icon_font.clone(), text_font.clone())),
            (studio_footer(text_font)),
        ]
    }
}

/// Header bar with design tool branding.
fn studio_header(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(56.0),
            background_color: Color::srgba(0.09, 0.09, 0.13, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(20.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.35, 0.5, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
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
                    gap: 12.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(34.0),
                            height: UVal::Px(34.0),
                            background_color: Color::srgb(0.95, 0.45, 0.25),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::LAYOUT_GRID.to_string() },
                                font_size: 18.0,
                                font: { icon_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                    ),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 2.0,
                        }
                        Children [
                            (UText {
                                text: { "CREATIVE VECTOR STUDIO // MATERIAL & SHADER PALETTE".to_string() },
                                font_size: 14.5,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { "Real-time SDF analytical shaders, multi-stop ramps & chamfered surfaces".to_string() },
                                font_size: 10.5,
                                font: { text_font.clone() },
                                color: Color::srgb(0.55, 0.6, 0.72),
                            })
                        ]
                    )
                ]
            ),
            (
                UNode {
                    background_color: Color::srgba(0.2, 0.15, 0.35, 0.6),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::axes(12.0, 6.0),
                }
                UBorder {
                    color: Color::srgb(0.7, 0.45, 1.0),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::CPU.to_string() },
                        font_size: 13.0,
                        font: { icon_font },
                        color: Color::srgb(0.85, 0.65, 1.0),
                    }),
                    (UText {
                        text: { "SDF HARDWARE RENDERING".to_string() },
                        font_size: 10.5,
                        font: { text_font },
                        color: Color::srgb(0.85, 0.65, 1.0),
                    })
                ]
            )
        ]
    }
}

/// Main workspace area: Left toolbar + material cards grid.
fn studio_workspace_row(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
        }
        Children [
            (studio_sidebar_toolbar(icon_font.clone())),
            (material_cards_grid(icon_font, text_font)),
        ]
    }
}

/// Vertical design studio toolbar.
fn studio_sidebar_toolbar(icon_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(52.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.08, 0.09, 0.13, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(6.0, 12.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.3, 0.45, 0.35),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Start,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        Children [
            (tool_button(Icon::MOUSE_POINTER, true, icon_font.clone())),
            (tool_button(Icon::PEN_TOOL, false, icon_font.clone())),
            (tool_button(Icon::SQUARE, false, icon_font.clone())),
            (tool_button(Icon::TYPE, false, icon_font.clone())),
            (tool_button(Icon::PIPETTE, false, icon_font.clone())),
            (tool_button(Icon::SLIDERS_VERTICAL, false, icon_font.clone())),
            (tool_button(Icon::ZOOM_IN, false, icon_font)),
        ]
    }
}

fn tool_button(icon_code: &'static str, active: bool, icon_font: Handle<Font>) -> impl Scene {
    let bg_color = if active {
        Color::srgb(0.95, 0.45, 0.25)
    } else {
        Color::srgba(0.14, 0.16, 0.22, 0.7)
    };
    let icon_color = if active {
        Color::WHITE
    } else {
        Color::srgb(0.65, 0.7, 0.82)
    };

    bsn! {
        UNode {
            width: UVal::Px(40.0),
            height: UVal::Px(40.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(8.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: bg_color,
            hovered: Color::srgb(0.3, 0.35, 0.48),
            pressed: Color::srgb(0.95, 0.45, 0.25),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: { icon_code.to_string() },
                font_size: 16.0,
                font: { icon_font },
                color: icon_color,
            })
        ]
    }
}

/// Row containing 4 distinct material and visual shader cards.
fn material_cards_grid(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
        }
        Children [
            (linear_sunset_card(icon_font.clone(), text_font.clone())),
            (radial_focus_card(icon_font.clone(), text_font.clone())),
            (chamfered_card(icon_font.clone(), text_font.clone())),
            (elevation_shadow_card(icon_font, text_font)),
        ]
    }
}

/// Swatch 1: Multi-stop linear sunset gradient.
fn linear_sunset_card(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    let stops = vec![
        UGradientStop::new(0.0, Color::srgb(0.95, 0.35, 0.2)),
        UGradientStop::new(0.5, Color::srgb(0.85, 0.2, 0.65)),
        UGradientStop::new(1.0, Color::srgb(0.35, 0.2, 0.85)),
    ];
    let linear_kind = UGradientKind::Linear { angle: FRAC_PI_4 };

    bsn! {
        UNode {
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.95),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(16.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            (palette_label(Icon::SUNSET, "1. LINEAR SUNSET RAMP", Color::srgb(0.95, 0.45, 0.3), icon_font.clone(), text_font.clone())),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(240.0),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(14.0),
                }
                StudioGradientSweep
                UGradient {
                    stops: stops,
                    kind: linear_kind,
                }
                UBorder {
                    color: Color::WHITE,
                    width: 1.0,
                    radius: UCornerRadius::all(12.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "Smooth 3-Stop Blend".to_string() },
                        font_size: 13.0,
                        font: { text_font.clone() },
                        color: Color::WHITE,
                        outline_width: 1.5,
                        outline_color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                    })
                ]
            ),
            (UText {
                text: { "Smooth 3-color sunset interpolation.".to_string() },
                font_size: 11.0,
                font: { text_font.clone() },
                color: Color::srgb(0.55, 0.6, 0.7),
            })
        ]
    }
}

/// Swatch 2: Radial focus studio lighting.
fn radial_focus_card(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    let stops = vec![
        UGradientStop::new(0.0, Color::srgb(0.3, 0.75, 1.0)),
        UGradientStop::new(1.0, Color::srgb(0.05, 0.08, 0.16)),
    ];
    let radial_kind = UGradientKind::Radial {
        center: Vec2::splat(0.5),
        radius: 0.5,
    };

    bsn! {
        UNode {
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.95),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(16.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            (palette_label(Icon::SUN, "2. RADIAL SPOTLIGHT", Color::srgb(0.3, 0.8, 1.0), icon_font.clone(), text_font.clone())),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(240.0),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(14.0),
                }
                UGradient {
                    stops: stops,
                    kind: radial_kind,
                }
                UBorder {
                    color: Color::srgba(0.3, 0.75, 1.0, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(12.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "Radial Studio Light".to_string() },
                        font_size: 13.0,
                        font: { text_font.clone() },
                        color: Color::WHITE,
                        outline_width: 1.5,
                        outline_color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                    })
                ]
            ),
            (UText {
                text: { "Radial diffusion from UV center.".to_string() },
                font_size: 11.0,
                font: { text_font.clone() },
                color: Color::srgb(0.55, 0.6, 0.7),
            })
        ]
    }
}

/// Swatch 3: Chamfered cut corners (`UShapeMode::Cut`).
fn chamfered_card(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            shape_mode: UShapeMode::Cut,
            border_radius: UCornerRadius::all(20.0),
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.95),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgb(0.85, 0.65, 0.35),
            width: 1.5,
            radius: UCornerRadius::all(20.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            (palette_label(Icon::BOX, "3. ARCHITECTURAL CUT", Color::srgb(0.9, 0.7, 0.35), icon_font.clone(), text_font.clone())),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(240.0),
                    shape_mode: UShapeMode::Cut,
                    border_radius: UCornerRadius::all(16.0),
                    background_color: Color::srgba(0.14, 0.12, 0.18, 0.9),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: Color::srgb(0.9, 0.7, 0.35),
                    width: 1.0,
                    radius: UCornerRadius::all(16.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "UShapeMode::Cut\nBeveled Corners".to_string() },
                        font_size: 13.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.95, 0.85, 0.6),
                        outline_width: 1.5,
                        outline_color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                    })
                ]
            ),
            (UText {
                text: { "Chamfered polygonal cut geometry.".to_string() },
                font_size: 11.0,
                font: { text_font.clone() },
                color: Color::srgb(0.55, 0.6, 0.7),
            })
        ]
    }
}

/// Swatch 4: Elevation drop shadow & inner softness.
fn elevation_shadow_card(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.95),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(16.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 12.0,
        }
        Children [
            (palette_label(Icon::APERTURE, "4. REALISTIC ELEVATION", Color::srgb(0.4, 0.85, 0.6), icon_font, text_font.clone())),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(240.0),
                    background_color: Color::srgb(0.14, 0.15, 0.22),
                    border_radius: UCornerRadius::all(14.0),
                    padding: USides::all(14.0),
                }
                UShadow {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.65),
                    offset: Vec2::new(0.0, 10.0),
                    blur: 24.0,
                    spread: 2.0,
                    is_inner: false,
                }
                UBorder {
                    color: Color::srgba(0.4, 0.45, 0.6, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "Soft Elevation (10px Drop)".to_string() },
                        font_size: 13.0,
                        font: { text_font.clone() },
                        color: Color::WHITE,
                        outline_width: 1.5,
                        outline_color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                    })
                ]
            ),
            (UText {
                text: { "Soft analytical GPU shadow elevation.".to_string() },
                font_size: 11.0,
                font: { text_font.clone() },
                color: Color::srgb(0.55, 0.6, 0.7),
            })
        ]
    }
}

/// Footer summary.
fn studio_footer(text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(38.0),
            background_color: Color::srgba(0.06, 0.06, 0.08, 0.9),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(16.0, 8.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: { "SDF Shape primitives render crisp borders, soft elevation, and gradients at any display DPI.".to_string() },
                font_size: 11.0,
                font: { text_font.clone() },
                color: Color::srgb(0.5, 0.55, 0.65),
            }),
            (UText {
                text: { "Univis Visual SDF Pipeline".to_string() },
                font_size: 11.0,
                font: { text_font },
                color: Color::srgb(0.95, 0.45, 0.25),
            })
        ]
    }
}

fn palette_label(
    icon_code: &'static str,
    title: &str,
    accent: Color,
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
) -> impl Scene {
    let title = title.to_string();
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        Children [
            (UText {
                text: { icon_code.to_string() },
                font_size: 13.0,
                font: { icon_font },
                color: accent,
            }),
            (UText {
                text: title,
                font_size: 12.0,
                font: { text_font },
                color: accent,
            })
        ]
    }
}

/// Animate the linear gradient sweep phase over time.
fn animate_studio_swatches(
    time: Res<Time>,
    mut sweeps: Query<&mut UGradient, With<StudioGradientSweep>>,
) {
    let t = time.elapsed_secs();
    for mut grad in &mut sweeps {
        grad.offset = (t * 0.3) % 1.0;
    }
}
