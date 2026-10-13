//! Team Workspace Collaboration Feed - Smooth Scrolling, Kinetic Drag, and Scrollbars with BSN.
//!
//! Visual Theme: Sleek modern engineering team chat & discussion feed (Charcoal, Deep Navy, Slate, Violet, Emerald).
//!
//! Features demonstrated:
//! - `UScrollContainer::vertical()` with smooth mouse wheel scrolling
//! - `UScrollKineticDrag` with momentum glide and friction deceleration
//! - Responsive text wrapping: `wrap_mode: UTextWrapMode::WrapToParent` and `word_wrap: true`
//! - Real-time SDF text outlines on header brand elements
//! - Rich embedded vector icons: `Icon::MESSAGE_SQUARE`, `Icon::HASH`, `Icon::USER`, `Icon::SEND`, `Icon::PAPERCLIP`, `Icon::CODE`, `Icon::CHECK`, `Icon::FLAME`, `Icon::ZAP`, `Icon::HEART`, `Icon::STAR`
//! - Channel sidebar with active channel indicators and direct message statuses
//! - Interactive "POST UPDATE" button with pointer click observers adding dynamic feedback
//! - Clean, modern, accessible typography with zero visual distortion

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct FeedChatState {
    message_count: u32,
    #[allow(dead_code)]
    active_channel: &'static str,
    status_notification: String,
}

impl Default for FeedChatState {
    fn default() -> Self {
        Self {
            message_count: 4,
            active_channel: "# infra-deployments",
            status_notification: "Connected to Orbit Realtime Gateway • P99 4.2ms".to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct ChannelNameText;

#[derive(Component, Clone, Copy, Default)]
struct FeedStatusText;

#[derive(Component, Clone, Copy, Default)]
struct DynamicMessageCounter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Team Collaboration Feed (UScrollContainer & BSN)".into(),
                resolution: (1260, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<FeedChatState>()
        .add_systems(Startup, setup_social_feed)
        .add_systems(Update, sync_chat_state_ui)
        .run();
}

fn setup_social_feed(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(social_feed_scene(&theme));
}

/// Root scene constructing the team chat workspace layout.
fn social_feed_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let body_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.05, 0.08),
            padding: USides::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 14.0,
        }
        Children [
            (feed_workspace_header(icon_font.clone(), body_font.clone())),
            (feed_main_split_layout(icon_font.clone(), body_font.clone())),
            (feed_workspace_footer(body_font)),
        ]
    }
}

/// Workspace top header bar.
fn feed_workspace_header(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(58.0),
            background_color: Color::srgba(0.08, 0.1, 0.16, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::axes(20.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.4, 0.35),
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
            // Left: Team Brand + Active Channel
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 14.0,
                }
                Children [
                    // Brand Badge with Lucide Icon
                    (
                        UNode {
                            width: UVal::Px(38.0),
                            height: UVal::Px(38.0),
                            background_color: Color::srgb(0.38, 0.35, 0.95),
                            border_radius: UCornerRadius::all(10.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::MESSAGE_SQUARE.to_string() },
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
                            (
                                ChannelNameText
                                UText {
                                    text: { "ORBIT LABS • # infra-deployments".to_string() },
                                    font_size: 14.0,
                                    font: { body_font.clone() },
                                    color: Color::WHITE,
                                    outline_width: 1.2,
                                    outline_color: Color::srgba(0.35, 0.45, 0.95, 0.5),
                                }
                            ),
                            (UText {
                                text: { "Production clusters, zero-downtime rollouts, and telemetry alerts".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.65),
                                wrap_mode: UTextWrapMode::WrapToParent,
                            })
                        ]
                    )
                ]
            ),
            // Right: Connectivity + Member Status
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
                            padding: USides::axes(10.0, 5.0),
                            background_color: Color::srgba(0.08, 0.25, 0.18, 0.8),
                            border_radius: UCornerRadius::all(6.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (
                                UNode {
                                    width: UVal::Px(7.0),
                                    height: UVal::Px(7.0),
                                    background_color: Color::srgb(0.2, 0.85, 0.5),
                                    border_radius: UCornerRadius::all(4.0),
                                }
                            ),
                            (UText {
                                text: { "16 ONLINE".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.3, 0.9, 0.6),
                            })
                        ]
                    ),
                    (
                        UNode {
                            padding: USides::axes(12.0, 6.0),
                            background_color: Color::srgba(0.12, 0.16, 0.24, 0.7),
                            border_radius: UCornerRadius::all(6.0),
                        }
                        Children [
                            (UText {
                                text: { "REGION: FRA-1".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.65, 0.7, 0.85),
                            })
                        ]
                    )
                ]
            )
        ]
    }
}

/// Main workspace body: sidebar channels + scrollable chat feed.
fn feed_main_split_layout(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
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
            (channels_sidebar(icon_font.clone(), body_font.clone())),
            (chat_feed_column(icon_font, body_font)),
        ]
    }
}

/// Left sidebar listing channels and direct messages.
fn channels_sidebar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(260.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.06, 0.08, 0.13, 0.9),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(16.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.25),
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
            // Channels & Direct messages list
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 12.0,
                }
                Children [
                    // Channels Section Header
                    (UText {
                        text: { "PUBLIC CHANNELS".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.65),
                    }),
                    (channel_pill_with_unread(icon_font.clone(), body_font.clone(), "infra-deployments", true, 3)),
                    (channel_pill(icon_font.clone(), body_font.clone(), "core-engine", false)),
                    (channel_pill_with_unread(icon_font.clone(), body_font.clone(), "ui-components", false, 7)),
                    (channel_pill(icon_font.clone(), body_font.clone(), "release-notes", false)),
                    (channel_pill(icon_font.clone(), body_font.clone(), "team-random", false)),

                    // Divider
                    (
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(1.0),
                            background_color: Color::srgba(0.2, 0.25, 0.35, 0.3),
                        }
                    ),

                    // Direct Messages Section Header
                    (UText {
                        text: { "DIRECT MESSAGES".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.65),
                    }),
                    (dm_item(body_font.clone(), "Sarah Chen", "Principal Architect", true)),
                    (dm_item(body_font.clone(), "Marcus Brody", "DevOps Lead", true)),
                    (dm_item(body_font.clone(), "Elena Rostova", "UI/UX Designer", false)),
                ]
            ),
            // Bottom Workspace Health Card
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.04, 0.06, 0.1, 0.7),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "LATENCY: 4.2ms".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.3, 0.85, 0.55),
                    }),
                    (UText {
                        text: { "TLS 1.3".to_string() },
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.68),
                    })
                ]
            )
        ]
    }
}

/// Helper for rendering an unread channel item pill.
fn channel_pill_with_unread(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    title: &'static str,
    active: bool,
    unread: u32,
) -> impl Scene {
    let bg_color = if active {
        Color::srgb(0.2, 0.22, 0.38)
    } else {
        Color::NONE
    };
    let text_color = if active {
        Color::WHITE
    } else {
        Color::srgb(0.6, 0.65, 0.75)
    };

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(32.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(10.0, 6.0),
        }
        UInteraction::default()
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
                        text: { Icon::HASH.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: text_color,
                    }),
                    (UText {
                        text: { title.to_string() },
                        font_size: 12.0,
                        font: { body_font.clone() },
                        color: text_color,
                    })
                ]
            ),
            (
                UNode {
                    padding: USides::axes(6.0, 2.0),
                    background_color: Color::srgb(0.38, 0.35, 0.95),
                    border_radius: UCornerRadius::all(8.0),
                }
                Children [
                    (UText {
                        text: { unread.to_string() },
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                    })
                ]
            )
        ]
    }
}

/// Helper for rendering a normal channel pill.
fn channel_pill(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    title: &'static str,
    active: bool,
) -> impl Scene {
    let bg_color = if active {
        Color::srgb(0.2, 0.22, 0.38)
    } else {
        Color::NONE
    };
    let text_color = if active {
        Color::WHITE
    } else {
        Color::srgb(0.6, 0.65, 0.75)
    };

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(32.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(10.0, 6.0),
        }
        UInteraction::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 8.0,
        }
        Children [
            (UText {
                text: { Icon::HASH.to_string() },
                font_size: 12.0,
                font: { icon_font },
                color: text_color,
            }),
            (UText {
                text: { title.to_string() },
                font_size: 12.0,
                font: { body_font.clone() },
                color: text_color,
            })
        ]
    }
}

/// Helper for direct message rows.
fn dm_item(body_font: Handle<Font>, name: &'static str, role: &'static str, online: bool) -> impl Scene {
    let dot_color = if online {
        Color::srgb(0.2, 0.85, 0.5)
    } else {
        Color::srgb(0.4, 0.45, 0.55)
    };

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(8.0, 4.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 8.0,
        }
        Children [
            (
                UNode {
                    width: UVal::Px(8.0),
                    height: UVal::Px(8.0),
                    background_color: dot_color,
                    border_radius: UCornerRadius::all(4.0),
                }
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
                        text: { name.to_string() },
                        font_size: 12.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.8, 0.85, 0.92),
                    }),
                    (UText {
                        text: { role.to_string() },
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.6),
                    })
                ]
            )
        ]
    }
}

/// Right main chat feed column: Scroll container + Bottom message bar.
fn chat_feed_column(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            height: UVal::Percent(1.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
        }
        Children [
            (feed_scroll_viewport(icon_font.clone(), body_font.clone())),
            (feed_message_input_bar(icon_font, body_font)),
        ]
    }
}

/// Scrollable viewport containing chat messages.
fn feed_scroll_viewport(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.95),
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(18.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.38, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        USelf {
            flex_grow: { Some(1.0) },
        }
        UScrollContainer::vertical()
        UScrollKineticDrag::default()
        Children [
            (feed_scroll_content(icon_font, body_font))
        ]
    }
}

/// Inner scroll content child that moves when scrolled.
fn feed_scroll_content(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 18.0,
        }
        Children [
            // Welcome Header Card
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.1, 0.13, 0.22, 0.5),
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::all(16.0),
                }
                UBorder {
                    color: Color::srgba(0.25, 0.3, 0.5, 0.3),
                    width: 1.0,
                    radius: UCornerRadius::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 14.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(36.0),
                            height: UVal::Px(36.0),
                            background_color: Color::srgba(0.38, 0.35, 0.95, 0.2),
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
                                color: Color::srgb(0.55, 0.5, 1.0),
                            })
                        ]
                    ),
                    (
                        UNode::default()
                        USelf {
                            flex_grow: { Some(1.0) },
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 4.0,
                        }
                        Children [
                            (UText {
                                text: { "Welcome to the start of the #infra-deployments channel history.".to_string() },
                                font_size: 14.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                wrap_mode: UTextWrapMode::WrapToParent,
                            }),
                            (UText {
                                text: { "Scroll with mouse wheel or drag to navigate message logs. Automatic word-wrapping adapts to parent container width dynamically.".to_string() },
                                font_size: 12.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.55, 0.6, 0.72),
                                wrap_mode: UTextWrapMode::WrapToParent,
                            })
                        ]
                    )
                ]
            ),

            // Message 1: Sarah Chen
            (chat_message_with_snippet(
                icon_font.clone(),
                body_font.clone(),
                "Sarah Chen",
                "STAFF",
                Color::srgb(0.38, 0.35, 0.95),
                "10:14 AM",
                "The Canary release of Univis Engine v0.3.1 passed all integration health checks across both EU and US-East regions. All multi-span font glyphs and real-time SDF text outlines settled in a single unified draw call with zero frame drops.",
                "kubectl rollout status deployment/univis-gateway -n production",
                Icon::CHECK,
                "7",
                Icon::ZAP,
                "4",
            )),

            // Message 2: Marcus Brody
            (chat_message_with_snippet(
                icon_font.clone(),
                body_font.clone(),
                "Marcus Brody",
                "DEVOPS",
                Color::srgb(0.15, 0.65, 0.45),
                "10:18 AM",
                "Affirmative. P99 latency dropped from 18ms down to 4.2ms after the new layout cache optimization and Parley multi-section measure pass.",
                "CLUSTER HEALTH: CPU 14.2% • MEM 184MB • ERRORS 0.00%",
                Icon::FLAME,
                "5",
                Icon::STAR,
                "3",
            )),

            // Message 3: Elena Rostova
            (chat_message(
                icon_font.clone(),
                body_font.clone(),
                "Elena Rostova",
                "DESIGN",
                Color::srgb(0.85, 0.4, 0.7),
                "10:25 AM",
                "Verified the new SDF text renderer on 4K HiDPI and mobile viewports. Glyph edges remain super sharp even under scale transitions, and WrapToParent wraps cleanly without any horizontal clipping.",
                Icon::HEART,
                "8",
                Icon::STAR,
                "6",
            )),

            // Message 4: Alex Morgan
            (chat_message_with_snippet(
                icon_font.clone(),
                body_font.clone(),
                "Alex Morgan",
                "FOUNDER",
                Color::srgb(0.2, 0.7, 0.95),
                "10:31 AM",
                "Incredible milestone team! We are opening public access for the BSN declarative UI suite today with full CSS Grid and Flexbox support.",
                "git push origin main --tags release-v0.3.1",
                Icon::CHECK,
                "12",
                Icon::FLAME,
                "9",
            )),

            // Dynamic Counter Pill
            (
                UNode {
                    width: UVal::Percent(1.0),
                    background_color: Color::srgba(0.12, 0.16, 0.25, 0.6),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (
                        DynamicMessageCounter
                        UText {
                            text: { "Channel History: 4 total messages logged in session".to_string() },
                            font_size: 11.0,
                            font: { body_font.clone() },
                            color: Color::srgb(0.4, 0.85, 0.6),
                        }
                    ),
                    (UText {
                        text: { "End of synchronized history".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.62),
                    })
                ]
            )
        ]
    }
}

/// Reusable chat message bubble scene without code snippet.
fn chat_message(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    author: &'static str,
    badge: &'static str,
    badge_color: Color,
    timestamp: &'static str,
    body: &'static str,
    r1_icon: &'static str,
    r1_count: &'static str,
    r2_icon: &'static str,
    r2_count: &'static str,
) -> impl Scene {
    let initial = author.chars().next().unwrap_or('?').to_string();

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.55),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(14.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.35, 0.2),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
            align_items: UAlignItems::Start,
        }
        Children [
            (
                UNode {
                    width: UVal::Px(38.0),
                    height: UVal::Px(38.0),
                    background_color: badge_color,
                    border_radius: UCornerRadius::all(19.0),
                }
                USelf {
                    flex_shrink: { Some(0.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: initial,
                        font_size: 16.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                    })
                ]
            ),
            (
                UNode::default()
                USelf {
                    flex_grow: { Some(1.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    (author_header_line(body_font.clone(), author, badge, badge_color, timestamp)),
                    (UText {
                        text: { body.to_string() },
                        font_size: 12.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.85, 0.88, 0.94),
                        wrap_mode: UTextWrapMode::WrapToParent,
                    }),
                    (reactions_row(icon_font, body_font, r1_icon, r1_count, r2_icon, r2_count))
                ]
            )
        ]
    }
}

/// Reusable chat message bubble scene with an embedded code snippet.
fn chat_message_with_snippet(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    author: &'static str,
    badge: &'static str,
    badge_color: Color,
    timestamp: &'static str,
    body: &'static str,
    code: &'static str,
    r1_icon: &'static str,
    r1_count: &'static str,
    r2_icon: &'static str,
    r2_count: &'static str,
) -> impl Scene {
    let initial = author.chars().next().unwrap_or('?').to_string();

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.55),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(14.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.35, 0.2),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
            align_items: UAlignItems::Start,
        }
        Children [
            (
                UNode {
                    width: UVal::Px(38.0),
                    height: UVal::Px(38.0),
                    background_color: badge_color,
                    border_radius: UCornerRadius::all(19.0),
                }
                USelf {
                    flex_shrink: { Some(0.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: initial,
                        font_size: 16.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                    })
                ]
            ),
            (
                UNode::default()
                USelf {
                    flex_grow: { Some(1.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    (author_header_line(body_font.clone(), author, badge, badge_color, timestamp)),
                    (UText {
                        text: { body.to_string() },
                        font_size: 12.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.85, 0.88, 0.94),
                        wrap_mode: UTextWrapMode::WrapToParent,
                    }),
                    (code_snippet_box(icon_font.clone(), body_font.clone(), code)),
                    (reactions_row(icon_font, body_font, r1_icon, r1_count, r2_icon, r2_count))
                ]
            )
        ]
    }
}

/// Helper for the author title line.
fn author_header_line(
    body_font: Handle<Font>,
    author: &'static str,
    badge: &'static str,
    badge_color: Color,
    timestamp: &'static str,
) -> impl Scene {
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 8.0,
        }
        Children [
            (UText {
                text: { author.to_string() },
                font_size: 13.0,
                font: { body_font.clone() },
                color: Color::WHITE,
            }),
            (
                UNode {
                    padding: USides::axes(6.0, 2.0),
                    background_color: badge_color,
                    border_radius: UCornerRadius::all(4.0),
                }
                Children [
                    (UText {
                        text: { badge.to_string() },
                        font_size: 9.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                    })
                ]
            ),
            (UText {
                text: { timestamp.to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.45, 0.5, 0.6),
            })
        ]
    }
}

/// Helper for code snippet boxes.
fn code_snippet_box(icon_font: Handle<Font>, body_font: Handle<Font>, code: &'static str) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.02, 0.03, 0.05, 0.9),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(12.0, 8.0),
        }
        UBorder {
            color: Color::srgba(0.25, 0.35, 0.5, 0.3),
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
                text: { Icon::CODE.to_string() },
                font_size: 12.0,
                font: { icon_font },
                color: Color::srgb(0.35, 0.7, 0.6),
            }),
            (UText {
                text: { code.to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.4, 0.85, 0.7),
                wrap_mode: UTextWrapMode::WrapToParent,
            })
        ]
    }
}

/// Helper for reactions row.
fn reactions_row(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon1: &'static str,
    count1: &'static str,
    icon2: &'static str,
    count2: &'static str,
) -> impl Scene {
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 8.0,
        }
        Children [
            (reaction_pill(icon_font.clone(), body_font.clone(), icon1, count1)),
            (reaction_pill(icon_font, body_font, icon2, count2)),
        ]
    }
}

/// Helper for reaction pills with Lucide icons.
fn reaction_pill(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    icon: &'static str,
    count: &'static str,
) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.1, 0.13, 0.22, 0.6),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(8.0, 4.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.35, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(6.0),
        }
        UInteraction::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        Children [
            (UText {
                text: { icon.to_string() },
                font_size: 12.0,
                font: { icon_font },
                color: Color::srgb(0.4, 0.8, 1.0),
            }),
            (UText {
                text: { count.to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.75, 0.8, 0.9),
            })
        ]
    }
}

/// Bottom message composer bar.
fn feed_message_input_bar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(52.0),
            background_color: Color::srgba(0.06, 0.08, 0.13, 0.95),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::axes(16.0, 8.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.4, 0.35),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
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
            // Left Input Placeholder Box
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
                        text: { Icon::MESSAGE_CIRCLE.to_string() },
                        font_size: 14.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.4, 0.45, 0.6),
                    }),
                    (UText {
                        text: { "Message #infra-deployments... (Type an update or code snippet)".to_string() },
                        font_size: 12.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.45, 0.5, 0.62),
                    })
                ]
            ),
            // Right Action Buttons
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                }
                Children [
                    (
                        UNode {
                            background_color: Color::srgba(0.14, 0.18, 0.28, 0.8),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(14.0, 8.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::srgba(0.14, 0.18, 0.28, 0.8),
                            hovered: Color::srgb(0.2, 0.25, 0.38),
                            pressed: Color::srgb(0.1, 0.12, 0.2),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::PAPERCLIP.to_string() },
                                font_size: 12.0,
                                font: { icon_font.clone() },
                                color: Color::srgb(0.7, 0.75, 0.85),
                            }),
                            (UText {
                                text: { "ATTACH LOG".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.7, 0.75, 0.85),
                            })
                        ]
                    ),
                    (
                        UNode {
                            background_color: Color::srgb(0.38, 0.35, 0.95),
                            border_radius: UCornerRadius::all(6.0),
                            padding: USides::axes(16.0, 8.0),
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
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                        }
                        Children [
                            (UText {
                                text: { Icon::SEND.to_string() },
                                font_size: 12.0,
                                font: { icon_font },
                                color: Color::WHITE,
                            }),
                            (UText {
                                text: { "POST UPDATE".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                        on(|_event: On<Pointer<Click>>, mut state: ResMut<FeedChatState>| {
                            state.message_count += 1;
                            state.status_notification = format!("Published cluster alert #{} to #infra-deployments", state.message_count);
                            info!("{}", state.status_notification);
                        })
                    )
                ]
            )
        ]
    }
}

/// Workspace bottom footer metadata bar.
fn feed_workspace_footer(body_font: Handle<Font>) -> impl Scene {
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
                FeedStatusText
                UText {
                    text: { "Connected to Orbit Realtime Gateway • P99 4.2ms".to_string() },
                    font_size: 11.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.4, 0.85, 0.6),
                }
            ),
            (UText {
                text: { "Univis Scroll Engine • UScrollContainer • Kinetic Momentum Drag • WrapToParent".to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.45, 0.5, 0.65),
            })
        ]
    }
}

/// Synchronizes state changes to dynamic text entities.
fn sync_chat_state_ui(
    state: Res<FeedChatState>,
    mut status_q: Query<&mut UText, (With<FeedStatusText>, Without<DynamicMessageCounter>)>,
    mut counter_q: Query<&mut UText, (With<DynamicMessageCounter>, Without<FeedStatusText>)>,
) {
    if state.is_changed() {
        for mut text in &mut status_q {
            text.text = state.status_notification.clone();
        }
        for mut text in &mut counter_q {
            text.text = format!(
                "Channel History: {} total messages logged in session",
                state.message_count
            );
        }
    }
}
