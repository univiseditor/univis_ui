//! Developer Profile & Security Settings - Form Inputs, Validation & Caret with BSN.
//!
//! Visual Theme: Sleek modern developer platform settings (Charcoal, Slate, Indigo, Cyan).
//!
//! Form features demonstrated:
//! - Interactive text inputs with carets: `UTextInput` and `UTextInputCaret`
//! - Text input filter rules: `TextInputFilter::AlphaNumeric`, `TextInputFilter::NumericOnly`, `TextInputFilter::SingleLine`
//! - Focus rings: `UFocusable::new()` and `UFocusVisual::border(...)`
//! - Toggle settings switches with interactive pointer click observers
//! - Form action buttons: "Save Changes" with real-time notification toast
//! - Embedded Lucide vector icons for categories, inputs, and security status
//! - Real-time SDF text outlines on headers and titles
//! - WrapToParent responsive text layout without overflow

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct ProfileFormState {
    two_factor_enabled: bool,
    #[allow(dead_code)]
    security_alerts_enabled: bool,
    save_count: u32,
    feedback_message: String,
}

impl Default for ProfileFormState {
    fn default() -> Self {
        Self {
            two_factor_enabled: true,
            security_alerts_enabled: true,
            save_count: 0,
            feedback_message: "All profile settings up to date".to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct FeedbackToastText;

#[derive(Component, Clone, Copy, Default)]
struct TwoFactorTogglePill;

#[derive(Component, Clone, Copy, Default)]
struct TwoFactorToggleKnob;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Developer Settings & Forms (Text Input & BSN)".into(),
                resolution: (1260, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<ProfileFormState>()
        .add_systems(Startup, setup_profile_form)
        .add_systems(Update, (link_text_input_children, sync_form_feedback))
        .run();
}

fn setup_profile_form(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(profile_form_scene(&theme));
}

/// Root scene constructing the developer settings portal.
fn profile_form_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let body_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.05, 0.08),
            padding: USides::all(24.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 16.0,
        }
        Children [
            (form_header(icon_font.clone(), body_font.clone())),
            (form_split_layout(icon_font.clone(), body_font.clone())),
            (form_footer(icon_font, body_font)),
        ]
    }
}

/// Top header bar.
fn form_header(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(58.0),
            background_color: Color::srgba(0.08, 0.1, 0.16, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(20.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.4, 0.3),
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
                            background_color: Color::srgb(0.38, 0.35, 0.95),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::SHIELD.to_string() },
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
                                text: { "DEVELOPER CLOUD CONSOLE".to_string() },
                                font_size: 14.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.0,
                                outline_color: Color::srgba(0.38, 0.35, 0.95, 0.4),
                            }),
                            (UText {
                                text: { "Account Identity & Security Credentials".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.7),
                            })
                        ]
                    )
                ]
            ),
            (
                // Org Switcher Chip
                UNode {
                    background_color: Color::srgba(0.12, 0.16, 0.24, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 7.0),
                }
                UBorder {
                    color: Color::srgba(0.3, 0.38, 0.55, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { Icon::HASH.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: Color::srgb(0.5, 0.7, 1.0),
                    }),
                    (UText {
                        text: { "ORG: UNIVIS-LABS / PROD".to_string() },
                        font_size: 11.0,
                        font: { body_font },
                        color: Color::srgb(0.7, 0.8, 0.95),
                    })
                ]
            )
        ]
    }
}

/// Main layout: Left sidebar navigation & Right form body.
fn form_split_layout(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
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
            (settings_nav_sidebar(icon_font.clone(), body_font.clone())),
            (settings_form_content(icon_font, body_font)),
        ]
    }
}

/// Left navigation sidebar for settings categories.
fn settings_nav_sidebar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(240.0),
            background_color: Color::srgba(0.07, 0.09, 0.14, 0.95),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 6.0,
        }
        Children [
            (UText {
                text: { "SETTINGS".to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.45, 0.5, 0.65),
            }),
            (nav_tab_item(icon_font.clone(), body_font.clone(), Icon::USER, "General Profile", true)),
            (nav_tab_item(icon_font.clone(), body_font.clone(), Icon::KEY, "Security & Keys", false)),
            (nav_tab_item(icon_font.clone(), body_font.clone(), Icon::ZAP, "API Rate Limits", false)),
            (nav_tab_item(icon_font.clone(), body_font.clone(), Icon::CREDIT_CARD, "Billing & Quotas", false)),
            (nav_tab_item(icon_font, body_font, Icon::CLIPBOARD_LIST, "Audit Trail", false)),
        ]
    }
}

fn nav_tab_item(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon: &'static str,
    label: &'static str,
    active: bool,
) -> impl Scene {
    let label = label.to_string();
    let bg = if active {
        Color::srgba(0.38, 0.35, 0.95, 0.25)
    } else {
        Color::NONE
    };
    let text_color = if active {
        Color::WHITE
    } else {
        Color::srgb(0.6, 0.65, 0.75)
    };
    let icon_color = if active {
        Color::srgb(0.4, 0.75, 1.0)
    } else {
        Color::srgb(0.45, 0.5, 0.6)
    };

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: bg,
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(12.0, 9.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: bg,
            hovered: Color::srgba(0.2, 0.25, 0.38, 0.4),
            pressed: Color::srgba(0.15, 0.2, 0.3, 0.6),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        Children [
            (UText {
                text: { icon.to_string() },
                font_size: 13.0,
                font: { icon_font },
                color: icon_color,
            }),
            (UText {
                text: label,
                font_size: 13.0,
                font: { body_font },
                color: text_color,
            })
        ]
    }
}

/// Right main form area with input fields, toggles, and submit bar.
fn settings_form_content(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.07, 0.09, 0.14, 0.95),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::all(24.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            // Section title
            (
                UNode {
                    width: UVal::Percent(1.0),
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
                            gap: 8.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::SETTINGS.to_string() },
                                font_size: 16.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.4, 0.75, 1.0),
                            }),
                            (UText {
                                text: { "GENERAL PROFILE & AUTHENTICATION".to_string() },
                                font_size: 16.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.0,
                                outline_color: Color::srgba(0.38, 0.35, 0.95, 0.35),
                            })
                        ]
                    ),
                    (UText {
                        text: { "Configure your public profile credentials and deployment authorization keys.".to_string() },
                        font_size: 12.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.7),
                        wrap_mode: UTextWrapMode::WrapToParent,
                    })
                ]
            ),

            // Form inputs grid / column
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 12.0,
                }
                Children [
                    (form_labeled_field(icon_font.clone(), body_font.clone(), Icon::USER, "FULL NAME", "Enter your legal full name", "Alex Morgan", TextInputFilter::SingleLine)),
                    (form_labeled_field(icon_font.clone(), body_font.clone(), Icon::AT_SIGN, "DEVELOPER HANDLE (ALPHANUMERIC ONLY)", "Lowercase identifier for CLI sessions", "alex_morgan99", TextInputFilter::AlphaNumeric)),
                    (form_labeled_field(icon_font.clone(), body_font.clone(), Icon::MAIL, "WORK EMAIL ADDRESS", "Primary contact for billing and token rotation", "alex.morgan@univis.io", TextInputFilter::SingleLine)),
                    (form_labeled_field(icon_font.clone(), body_font.clone(), Icon::ZAP, "GLOBAL API RATE LIMIT (REQ / MIN - NUMERIC ONLY)", "Custom burst rate threshold for sandbox tokens", "5000", TextInputFilter::NumericOnly)),
                ]
            ),

            // Security switch toggle row
            (security_toggle_row(icon_font.clone(), body_font.clone())),

            // Bottom action & save bar
            (form_save_actions_bar(icon_font, body_font)),
        ]
    }
}

/// Helper constructing a labeled form text field.
fn form_labeled_field(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon: &'static str,
    label: &'static str,
    caption: &'static str,
    initial: &'static str,
    filter: TextInputFilter,
) -> impl Scene {
    let label = label.to_string();
    let caption = caption.to_string();
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 4.0,
        }
        Children [
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: label,
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.7, 0.75, 0.9),
                    }),
                    (UText {
                        text: caption,
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.6),
                    })
                ]
            ),
            (form_text_input(icon_font, body_font, icon, "Enter value...", initial, filter)),
        ]
    }
}

/// Standalone interactive text input matching Univis UI's `UTextInput` architecture.
fn form_text_input(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon: &'static str,
    placeholder: &'static str,
    initial: &'static str,
    filter: TextInputFilter,
) -> impl Scene {
    let font_size = 13.0f32;
    let initial_text = if initial.is_empty() {
        placeholder.to_string()
    } else {
        initial.to_string()
    };
    let initial_color = if initial.is_empty() {
        Color::srgba(0.4, 0.45, 0.6, 0.6)
    } else {
        Color::srgb(0.9, 0.93, 0.98)
    };

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(38.0),
            padding: USides::axes(12.0, 6.0),
            background_color: Color::srgba(0.04, 0.06, 0.09, 0.8),
            border_radius: UCornerRadius::all(8.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.3, 0.45, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.38, 0.55, 1.0), 1.5)
        UInteraction::default()
        UTextInput {
            placeholder: { placeholder.to_string() },
            value: { initial.to_string() },
            filter: { filter },
        }
        UTextInputCaret {
            color: Color::srgb(0.38, 0.55, 1.0),
            width: 2.0,
        }
        Children [
            (UText {
                text: { icon.to_string() },
                font_size: 13.0,
                font: { icon_font },
                color: Color::srgb(0.4, 0.5, 0.7),
            }),
            (
                UNode {
                    width: UVal::Px(0.0),
                    height: { UVal::Px(font_size * 1.2) },
                    background_color: Color::srgba(0.2, 0.45, 0.9, 0.35),
                    border_radius: UCornerRadius::all(2.0),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(36.0),
                }
                Visibility::Hidden
                UTextInputSelectionMarker
            ),
            (
                UTextInputTextMarker
                UText {
                    text: initial_text,
                    font_size: 13.0,
                    font: { body_font },
                    color: initial_color,
                }
            ),
            (
                UNode {
                    width: UVal::Px(2.0),
                    height: { UVal::Px(font_size * 1.2) },
                    background_color: Color::srgb(0.38, 0.55, 1.0),
                    border_radius: UCornerRadius::all(1.0),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(36.0),
                }
                Visibility::Hidden
                UTextInputCaretMarker
            )
        ]
    }
}

/// Security 2FA toggle row with interactive switch.
fn security_toggle_row(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.6),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(14.0),
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
                            width: UVal::Px(32.0),
                            height: UVal::Px(32.0),
                            background_color: Color::srgba(0.1, 0.7, 0.45, 0.18),
                            border_radius: UCornerRadius::all(8.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::KEY.to_string() },
                                font_size: 15.0,
                                font: { icon_font },
                                color: Color::srgb(0.2, 0.85, 0.5),
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
                                text: { "MANDATORY TWO-FACTOR AUTHENTICATION (2FA)".to_string() },
                                font_size: 12.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { "Enforce hardware FIDO2 key or TOTP code before pushing git commits to main.".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.65),
                                wrap_mode: UTextWrapMode::WrapToParent,
                            })
                        ]
                    )
                ]
            ),
            (
                // Toggle Switch Pill
                TwoFactorTogglePill
                UNode {
                    width: UVal::Px(46.0),
                    height: UVal::Px(24.0),
                    background_color: Color::srgb(0.1, 0.7, 0.45),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(3.0),
                }
                USelf {
                    flex_shrink: { Some(0.0) },
                }
                UInteraction::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::End,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        TwoFactorToggleKnob
                        UNode {
                            width: UVal::Px(18.0),
                            height: UVal::Px(18.0),
                            background_color: Color::WHITE,
                            border_radius: UCornerRadius::all(9.0),
                        }
                    )
                ]
                on(|_event: On<Pointer<Click>>, mut state: ResMut<ProfileFormState>| {
                    state.two_factor_enabled = !state.two_factor_enabled;
                    state.feedback_message = if state.two_factor_enabled {
                        "2FA enforced for all workspace deployment channels".to_string()
                    } else {
                        "Warning: 2FA disabled! Workspace credentials less secure".to_string()
                    };
                    info!("{}", state.feedback_message);
                })
            )
        ]
    }
}

/// Action footer bar with save and cancel buttons.
fn form_save_actions_bar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(0.0, 6.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                FeedbackToastText
                UText {
                    text: { "All profile settings up to date".to_string() },
                    font_size: 12.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.4, 0.8, 0.6),
                }
            ),
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
                            background_color: Color::srgba(0.12, 0.16, 0.22, 0.8),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::axes(18.0, 10.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgba(0.12, 0.16, 0.22, 0.8),
                            hovered: Color::srgb(0.18, 0.22, 0.3),
                            pressed: Color::srgb(0.08, 0.1, 0.15),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::X.to_string() },
                                font_size: 11.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.65, 0.7, 0.8),
                            }),
                            (UText {
                                text: { "DISCARD".to_string() },
                                font_size: 12.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.65, 0.7, 0.8),
                            })
                        ]
                    ),
                    (
                        UNode {
                            background_color: Color::srgb(0.38, 0.35, 0.95),
                            border_radius: UCornerRadius::all(8.0),
                            padding: USides::axes(24.0, 10.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgb(0.38, 0.35, 0.95),
                            hovered: Color::srgb(0.48, 0.45, 1.0),
                            pressed: Color::srgb(0.3, 0.25, 0.8),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                            gap: 8.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::SAVE.to_string() },
                                font_size: 12.0,
                                font: { icon_font },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { "SAVE CHANGES".to_string() },
                                font_size: 12.0,
                                font: { body_font },
                                color: Color::WHITE,
                            })
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<ProfileFormState>| {
                            state.save_count += 1;
                            state.feedback_message = format!("Success: Profile changes saved at revision #{}", state.save_count);
                            info!("{}", state.feedback_message);
                        })
                    )
                ]
            )
        ]
    }
}

/// Footer summary bar.
fn form_footer(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(36.0),
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
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::INFO.to_string() },
                        font_size: 12.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    }),
                    (UText {
                        text: { "Univis Form Engine • UTextInput, Caret Animation, and Filter Validation".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    })
                ]
            ),
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
                        text: { Icon::CHECK.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: Color::srgb(0.38, 0.55, 1.0),
                    }),
                    (UText {
                        text: { "BSN Architecture • Hardware-accelerated UI".to_string() },
                        font_size: 11.0,
                        font: { body_font },
                        color: Color::srgb(0.38, 0.55, 1.0),
                    })
                ]
            )
        ]
    }
}

/// Updates feedback toast text and 2FA pill visuals.
fn sync_form_feedback(
    state: Res<ProfileFormState>,
    mut toast_q: Query<&mut UText, With<FeedbackToastText>>,
    mut toggle_pill_q: Query<
        (&mut UNode, &mut ULayout),
        (With<TwoFactorTogglePill>, Without<TwoFactorToggleKnob>),
    >,
) {
    if state.is_changed() {
        for mut text in &mut toast_q {
            text.text = state.feedback_message.clone();
            text.color = if state.two_factor_enabled {
                Color::srgb(0.4, 0.85, 0.6)
            } else {
                Color::srgb(0.95, 0.55, 0.3)
            };
        }
        for (mut node, mut layout) in &mut toggle_pill_q {
            if state.two_factor_enabled {
                node.background_color = Color::srgb(0.1, 0.7, 0.45);
                layout.justify_content = UJustifyContent::End;
            } else {
                node.background_color = Color::srgb(0.3, 0.35, 0.45);
                layout.justify_content = UJustifyContent::Start;
            }
        }
    }
}

/// Automatically links child text, caret, and selection marker entities spawned via BSN to the parent `UTextInputCaret`.
fn link_text_input_children(
    mut inputs: Query<&mut UTextInputCaret>,
    children_q: Query<(Entity, &Children)>,
    text_markers: Query<Entity, With<UTextInputTextMarker>>,
    caret_markers: Query<Entity, With<UTextInputCaretMarker>>,
    selection_markers: Query<Entity, With<UTextInputSelectionMarker>>,
) {
    for (entity, children) in children_q.iter() {
        if let Ok(mut caret) = inputs.get_mut(entity) {
            if caret.text_entity.is_none()
                || caret.caret_entity.is_none()
                || caret.selection_entity.is_none()
            {
                for child in children.iter() {
                    if text_markers.get(child).is_ok() {
                        caret.text_entity = Some(child);
                    }
                    if caret_markers.get(child).is_ok() {
                        caret.caret_entity = Some(child);
                    }
                    if selection_markers.get(child).is_ok() {
                        caret.selection_entity = Some(child);
                    }
                }
            }
        }
    }
}
