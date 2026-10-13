//! Tactical Game Settings & Inventory Menu - Focus Navigation, Focus Rings & BSN.
//!
//! Visual Theme: Premium Nordic Tactical Simulator (Deep Graphite Slate, Amber Gold, Crisp White).
//!
//! Features demonstrated:
//! - Built-in vector icons: `Icon::GAMEPAD_2`, `Icon::COMMAND`, `Icon::MONITOR`, `Icon::VOLUME_2`,
//!   `Icon::CROSSHAIR`, `Icon::EYE`, `Icon::ACTIVITY`, `Icon::CPU`, `Icon::SLIDERS_VERTICAL`,
//!   `Icon::ZAP`, `Icon::REFRESH_CW`, `Icon::CHECK`, `Icon::ROTATE_CCW`, `Icon::X`, `Icon::INFO`.
//! - `UFocusable::new()`: Spatial and sequential keyboard/gamepad navigation (`Tab`, `Shift+Tab`, arrow keys, `Enter`/`Space`)
//! - `UFocusVisual::border(...)` & `UFocusVisual::glow(...)`: Dynamic focus rings and accent borders
//! - Focusable option chips and buttons with active/selected visual states
//! - Settings controls: Graphics presets, Resolution selectors, Ray-tracing and V-Sync toggles
//! - Interactive state tracking with `UFocusState` observer updating active navigation hints
//! - Clean, modern, accessible typography with zero visual distortion

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct GameSettingsState {
    #[allow(dead_code)]
    resolution_idx: usize,
    #[allow(dead_code)]
    preset_idx: usize,
    raytracing_enabled: bool,
    vsync_enabled: bool,
    status_message: String,
}

impl Default for GameSettingsState {
    fn default() -> Self {
        Self {
            resolution_idx: 1, // 1440p
            preset_idx: 3,     // Ultra
            raytracing_enabled: true,
            vsync_enabled: true,
            status_message: "Graphics Preset: ULTRA (1440p @ 144Hz) • Raytracing: ON • V-Sync: ON"
                .to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct SettingsFeedbackText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Game Settings & Focus Navigation (UFocusable & Vector Icons)".into(),
                resolution: (1260, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<GameSettingsState>()
        .add_systems(Startup, setup_game_menu)
        .add_systems(Update, sync_game_settings_ui)
        .run();
}

fn setup_game_menu(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(game_menu_scene(&theme));
}

/// Root scene constructing the tactical game settings layout.
fn game_menu_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let text_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.05, 0.06, 0.09),
            padding: USides::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 16.0,
        }
        Children [
            (game_top_header(icon_font.clone(), text_font.clone())),
            (game_settings_split_body(icon_font.clone(), text_font.clone())),
            (game_bottom_controller_bar(icon_font, text_font)),
        ]
    }
}

/// Top Game Header Bar.
fn game_top_header(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(58.0),
            background_color: Color::srgba(0.08, 0.1, 0.15, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(20.0, 10.0),
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
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Title & Subtitle with Vector Gamepad Icon
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
                            width: UVal::Px(38.0),
                            height: UVal::Px(38.0),
                            background_color: Color::srgba(0.95, 0.75, 0.3, 0.2),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        UBorder {
                            color: Color::srgb(0.95, 0.75, 0.3),
                            width: 1.0,
                            radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::GAMEPAD_2.to_string() },
                                font_size: 20.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.95, 0.8, 0.4),
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
                                text: { "VALKYRIE // TACTICAL SIMULATION".to_string() },
                                font_size: 15.0,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.2,
                                outline_color: Color::srgba(0.95, 0.75, 0.3, 0.35),
                            }),
                            (UText {
                                text: { "SYSTEM CONFIGURATION & HARDWARE TUNING • BUILD 1.04.8".to_string() },
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.65),
                            })
                        ]
                    )
                ]
            ),
            // Right Control Modality Badge
            (
                UNode {
                    padding: USides::axes(14.0, 7.0),
                    background_color: Color::srgba(0.15, 0.2, 0.3, 0.7),
                    border_radius: UCornerRadius::all(6.0),
                }
                UBorder {
                    color: Color::srgba(0.95, 0.75, 0.3, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::COMMAND.to_string() },
                        font_size: 13.0,
                        font: { icon_font },
                        color: Color::srgb(0.95, 0.75, 0.3),
                    }),
                    (UText {
                        text: { "KEYBOARD FOCUS ACTIVE [TAB / ARROWS]".to_string() },
                        font_size: 11.0,
                        font: { text_font },
                        color: Color::srgb(0.95, 0.85, 0.6),
                    })
                ]
            )
        ]
    }
}

/// Main Settings Body: Left Navigation Column + Right Configuration Panel.
fn game_settings_split_body(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
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
            gap: 16.0,
        }
        Children [
            (game_category_sidebar(icon_font.clone(), text_font.clone())),
            (game_controls_panel(icon_font, text_font)),
        ]
    }
}

/// Left Navigation Sidebar listing menu sections.
fn game_category_sidebar(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(280.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.07, 0.08, 0.13, 0.9),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
        }
        Children [
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { "SETTINGS CATEGORIES".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.62),
                    }),
                    (menu_category_button(Icon::MONITOR, "DISPLAY & GRAPHICS", true, icon_font.clone(), text_font.clone())),
                    (menu_category_button(Icon::VOLUME_2, "AUDIO & ACOUSTICS", false, icon_font.clone(), text_font.clone())),
                    (menu_category_button(Icon::GAMEPAD_2, "CONTROLS & KEYBINDS", false, icon_font.clone(), text_font.clone())),
                    (menu_category_button(Icon::CROSSHAIR, "GAMEPLAY & HUD", false, icon_font.clone(), text_font.clone())),
                    (menu_category_button(Icon::EYE, "ACCESSIBILITY", false, icon_font.clone(), text_font.clone())),
                    (menu_category_button(Icon::ACTIVITY, "BENCHMARK TEST", false, icon_font.clone(), text_font.clone())),
                ]
            ),
            // Hardware Spec Summary Card
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.04, 0.06, 0.1, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(12.0),
                }
                UBorder {
                    color: Color::srgba(0.2, 0.25, 0.35, 0.25),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::CPU.to_string() },
                                font_size: 12.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.95, 0.75, 0.3),
                            }),
                            (UText {
                                text: { "DETECTED HARDWARE".to_string() },
                                font_size: 10.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.65),
                            })
                        ]
                    ),
                    (UText {
                        text: { "VULKAN 1.3 • MESA HIGH-PERF".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.95, 0.75, 0.3),
                    }),
                    (UText {
                        text: { "VRAM: 8,192 MB ALLOCATED".to_string() },
                        font_size: 10.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.65, 0.7, 0.8),
                    })
                ]
            )
        ]
    }
}

/// Helper for rendering a focusable category tab button with vector icon.
fn menu_category_button(
    icon_code: &str,
    label: &'static str,
    active: bool,
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
) -> impl Scene {
    let icon_str = icon_code.to_string();
    let bg_color = if active {
        Color::srgba(0.95, 0.75, 0.3, 0.18)
    } else {
        Color::NONE
    };
    let border_color = if active {
        Color::srgb(0.95, 0.75, 0.3)
    } else {
        Color::NONE
    };
    let text_color = if active {
        Color::srgb(0.95, 0.85, 0.4)
    } else {
        Color::srgb(0.65, 0.7, 0.8)
    };

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(38.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(12.0, 8.0),
        }
        UBorder {
            color: border_color,
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.95, 0.75, 0.3), 2.0)
        UInteraction::default()
        UInteractionColors {
            normal: bg_color,
            hovered: Color::srgba(0.95, 0.75, 0.3, 0.28),
            pressed: Color::srgba(0.95, 0.75, 0.3, 0.4),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        Children [
            (UText {
                text: icon_str,
                font_size: 14.0,
                font: { icon_font },
                color: text_color,
            }),
            (UText {
                text: { label.to_string() },
                font_size: 12.0,
                font: { text_font },
                color: text_color,
            })
        ]
    }
}

/// Main Settings Form Panel: Graphics options and action buttons.
fn game_controls_panel(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.07, 0.08, 0.13, 0.92),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(20.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
        }
        Children [
            // Settings Controls Form
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 14.0,
                }
                Children [
                    // Section Title
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
                                UNode {
                                    width: UVal::Px(32.0),
                                    height: UVal::Px(32.0),
                                    background_color: Color::srgba(0.95, 0.75, 0.3, 0.15),
                                    border_radius: UCornerRadius::all(6.0),
                                }
                                ULayout {
                                    display: UDisplay::Flex,
                                    justify_content: UJustifyContent::Center,
                                    align_items: UAlignItems::Center,
                                }
                                Children [
                                    (UText {
                                        text: { Icon::SLIDERS_VERTICAL.to_string() },
                                        font_size: 16.0,
                                        font: { icon_font.clone() },
                                        color: Color::srgb(0.95, 0.8, 0.4),
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
                                        text: { "DISPLAY RESOLUTION & RENDER ENGINE".to_string() },
                                        font_size: 14.0,
                                        font: { text_font.clone() },
                                        color: Color::WHITE,
                                    }),
                                    (UText {
                                        text: { "Navigate options with TAB or Arrow Keys. Press ENTER to select or toggle.".to_string() },
                                        font_size: 11.0,
                                        font: { text_font.clone() },
                                        color: Color::srgb(0.5, 0.55, 0.65),
                                    })
                                ]
                            )
                        ]
                    ),

                    // Control Row 1: Resolution
                    (control_row(
                        "RENDER RESOLUTION",
                        "Target display buffer pixel dimensions",
                        bsn! {
                            UNode::default()
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                gap: 8.0,
                            }
                            Children [
                                (option_chip("1080p (FHD)", false, icon_font.clone(), text_font.clone())),
                                (option_chip("1440p (QHD)", true, icon_font.clone(), text_font.clone())),
                                (option_chip("2160p (4K)", false, icon_font.clone(), text_font.clone())),
                            ]
                        },
                        text_font.clone(),
                    )),

                    // Control Row 2: Quality Preset
                    (control_row(
                        "GRAPHICS PRESET",
                        "Overall asset fidelity and shader complexity profile",
                        bsn! {
                            UNode::default()
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                gap: 8.0,
                            }
                            Children [
                                (option_chip("LOW", false, icon_font.clone(), text_font.clone())),
                                (option_chip("MEDIUM", false, icon_font.clone(), text_font.clone())),
                                (option_chip("HIGH", false, icon_font.clone(), text_font.clone())),
                                (option_chip("ULTRA", true, icon_font.clone(), text_font.clone())),
                            ]
                        },
                        text_font.clone(),
                    )),

                    // Control Row 3: Frame Rate
                    (control_row(
                        "FRAME RATE LIMIT",
                        "Maximum swapchain presentation frequency",
                        bsn! {
                            UNode::default()
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                gap: 8.0,
                            }
                            Children [
                                (option_chip("60 FPS", false, icon_font.clone(), text_font.clone())),
                                (option_chip("144 FPS", true, icon_font.clone(), text_font.clone())),
                                (option_chip("UNLIMITED", false, icon_font.clone(), text_font.clone())),
                            ]
                        },
                        text_font.clone(),
                    )),

                    // Control Row 4: Ray-Tracing Toggle
                    (toggle_control_row(
                        Icon::ZAP,
                        "HARDWARE RAY TRACING",
                        "Real-time BVH reflections and global ambient occlusion",
                        true,
                        true,
                        icon_font.clone(),
                        text_font.clone(),
                    )),

                    // Control Row 5: V-Sync Toggle
                    (toggle_control_row(
                        Icon::REFRESH_CW,
                        "VERTICAL SYNC (V-SYNC)",
                        "Synchronize frame swaps with display refresh to prevent tearing",
                        true,
                        false,
                        icon_font.clone(),
                        text_font.clone(),
                    )),
                ]
            ),

            // Bottom Action Buttons
            (game_action_buttons_bar(icon_font, text_font))
        ]
    }
}

/// Helper for rendering a settings row with a custom widget scene on the right.
fn control_row(title: &'static str, desc: &'static str, widget: impl Scene, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.55),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(12.0),
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
                    flex_direction: UFlexDirection::Column,
                    gap: 2.0,
                }
                Children [
                    (UText {
                        text: { title.to_string() },
                        font_size: 12.0,
                        font: { text_font.clone() },
                        color: Color::WHITE,
                    }),
                    (UText {
                        text: { desc.to_string() },
                        font_size: 10.0,
                        font: { text_font },
                        color: Color::srgb(0.5, 0.55, 0.65),
                        wrap_mode: UTextWrapMode::WrapToParent,
                    })
                ]
            ),
            (widget)
        ]
    }
}

/// Helper for rendering a focusable option chip with checkmark when selected.
fn option_chip(label: &'static str, selected: bool, icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    let bg_color = if selected {
        Color::srgb(0.95, 0.75, 0.3)
    } else {
        Color::srgba(0.12, 0.16, 0.24, 0.8)
    };
    let text_color = if selected {
        Color::srgb(0.08, 0.08, 0.12)
    } else {
        Color::srgb(0.7, 0.75, 0.85)
    };

    bsn! {
        UNode {
            padding: USides::axes(12.0, 6.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(6.0),
        }
        UFocusable::new()
        UFocusVisual::border(Color::srgb(1.0, 0.9, 0.5), 2.0)
        UInteraction::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: { if selected { 4.0 } else { 0.0 } },
        }
        Children [
            (UText {
                text: { if selected { Icon::CHECK.to_string() } else { "".to_string() } },
                font_size: 11.0,
                font: { icon_font },
                color: text_color,
            }),
            (UText {
                text: { label.to_string() },
                font_size: 11.0,
                font: { text_font },
                color: text_color,
            })
        ]
    }
}

/// Helper for rendering a focusable toggle row with vector icon.
fn toggle_control_row(
    icon_code: &str,
    title: &'static str,
    desc: &'static str,
    _initial_on: bool,
    is_raytracing: bool,
    icon_font: Handle<Font>,
    text_font: Handle<Font>,
) -> impl Scene {
    let icon_str = icon_code.to_string();
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.55),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(12.0),
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
                    gap: 10.0,
                }
                Children [
                    (UText {
                        text: icon_str,
                        font_size: 14.0,
                        font: { icon_font },
                        color: Color::srgb(0.95, 0.75, 0.3),
                    }),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 2.0,
                        }
                        Children [
                            (UText {
                                text: { title.to_string() },
                                font_size: 12.0,
                                font: { text_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { desc.to_string() },
                                font_size: 10.0,
                                font: { text_font },
                                color: Color::srgb(0.5, 0.55, 0.65),
                                wrap_mode: UTextWrapMode::WrapToParent,
                            })
                        ]
                    )
                ]
            ),
            (
                // Focusable Toggle Switch Pill
                UNode {
                    width: UVal::Px(46.0),
                    height: UVal::Px(24.0),
                    background_color: Color::srgb(0.12, 0.72, 0.45),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(3.0),
                }
                UFocusable::new()
                UFocusVisual::border(Color::srgb(1.0, 0.9, 0.5), 2.0)
                UInteraction::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::End,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(18.0),
                            height: UVal::Px(18.0),
                            background_color: Color::WHITE,
                            border_radius: UCornerRadius::all(9.0),
                        }
                    )
                ]
                on(move |_event: On<Pointer<Click>>, mut state: ResMut<GameSettingsState>| {
                    if is_raytracing {
                        state.raytracing_enabled = !state.raytracing_enabled;
                    } else {
                        state.vsync_enabled = !state.vsync_enabled;
                    }
                    state.status_message = format!(
                        "Configuration updated: Raytracing {} • V-Sync {}",
                        if state.raytracing_enabled { "ENABLED" } else { "DISABLED" },
                        if state.vsync_enabled { "ENABLED" } else { "DISABLED" },
                    );
                    info!("{}", state.status_message);
                })
            )
        ]
    }
}

/// Action bar at the bottom of the settings card.
fn game_action_buttons_bar(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(0.0, 4.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            // Left: Restore Defaults Button with Rotate Icon
            (
                UNode {
                    background_color: Color::srgba(0.12, 0.16, 0.24, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 10.0),
                }
                UFocusable::new()
                UFocusVisual::border(Color::srgb(0.95, 0.75, 0.3), 2.0)
                UInteraction::default()
                UInteractionColors {
                    normal: Color::srgba(0.12, 0.16, 0.24, 0.8),
                    hovered: Color::srgb(0.18, 0.22, 0.32),
                    pressed: Color::srgb(0.08, 0.1, 0.16),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::ROTATE_CCW.to_string() },
                        font_size: 12.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.7, 0.75, 0.85),
                    }),
                    (UText {
                        text: { "RESTORE DEFAULTS".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.7, 0.75, 0.85),
                    })
                ]
            ),
            // Right: Discard + Apply Changes
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 12.0,
                }
                Children [
                    (
                        UNode {
                            background_color: Color::srgba(0.15, 0.18, 0.26, 0.8),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::axes(16.0, 10.0),
                        }
                        UFocusable::new()
                        UFocusVisual::border(Color::srgb(0.95, 0.75, 0.3), 2.0)
                        UInteraction::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::X.to_string() },
                                font_size: 12.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.65, 0.7, 0.8),
                            }),
                            (UText {
                                text: { "DISCARD".to_string() },
                                font_size: 11.0,
                                font: { text_font.clone() },
                                color: Color::srgb(0.65, 0.7, 0.8),
                            })
                        ]
                    ),
                    (
                        UNode {
                            background_color: Color::srgb(0.95, 0.75, 0.3),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::axes(18.0, 10.0),
                        }
                        UFocusable::new()
                        UFocusVisual::glow(Color::srgb(1.0, 0.9, 0.5))
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgb(0.95, 0.75, 0.3),
                            hovered: Color::srgb(1.0, 0.85, 0.4),
                            pressed: Color::srgb(0.8, 0.6, 0.2),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::CHECK.to_string() },
                                font_size: 13.0,
                                font: { icon_font },
                                color: Color::srgb(0.08, 0.08, 0.12),
                            }),
                            (UText {
                                text: { "APPLY CONFIGURATION".to_string() },
                                font_size: 11.0,
                                font: { text_font },
                                color: Color::srgb(0.08, 0.08, 0.12),
                            })
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<GameSettingsState>| {
                            state.status_message = "All graphical pipeline changes successfully applied to engine!".to_string();
                            info!("{}", state.status_message);
                        })
                    )
                ]
            )
        ]
    }
}

/// Bottom controller navigation hints bar.
fn game_bottom_controller_bar(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(38.0),
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.85),
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
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::INFO.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: Color::srgb(0.95, 0.8, 0.4),
                    }),
                    (
                        SettingsFeedbackText
                        UText {
                            text: { "Graphics Preset: ULTRA (1440p @ 144Hz) • Raytracing: ON • V-Sync: ON".to_string() },
                            font_size: 11.0,
                            font: { text_font.clone() },
                            color: Color::srgb(0.95, 0.8, 0.4),
                        }
                    )
                ]
            ),
            (UText {
                text: { "Univis Focus Engine • UFocusable Spatial & Tab Navigation • UFocusVisual".to_string() },
                font_size: 11.0,
                font: { text_font },
                color: Color::srgb(0.45, 0.5, 0.65),
            })
        ]
    }
}

/// Synchronizes state feedback to UI.
fn sync_game_settings_ui(
    state: Res<GameSettingsState>,
    mut text_q: Query<&mut UText, With<SettingsFeedbackText>>,
) {
    if state.is_changed() {
        for mut text in &mut text_q {
            text.text = state.status_message.clone();
        }
    }
}
