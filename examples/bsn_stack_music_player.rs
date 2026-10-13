//! Modern Music Streaming Player - Layered Stack & Aspect Ratio with BSN & Vector Icons.
//!
//! Visual Theme: Premium music streaming interface with glass overlays and 1:1 square album art.
//!
//! Stack, Icon & Advanced Layout features demonstrated:
//! - Built-in vector icons: `Icon::PLAY`, `Icon::PAUSE`, `Icon::SKIP_BACK`, `Icon::SKIP_FORWARD`,
//!   `Icon::SHUFFLE`, `Icon::REPEAT`, `Icon::HEART`, `Icon::VOLUME_2`, `Icon::HEADPHONES`, `Icon::MUSIC`.
//! - Layered stacking: `ULayout { display: UDisplay::Stack }`
//! - Aspect ratio enforcement: `UNode { aspect_ratio: { Some(1.0) } }` (perfect square album cover)
//! - Absolute pinned overlays: `USelf { position_type: UPositionType::Absolute, top: ..., left: ... }`
//! - Interactive play/pause toggle with BSN pointer observer and dynamic icon flipping

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
struct PlayerState {
    is_playing: bool,
    is_liked: bool,
    volume: f32,
    current_track: &'static str,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            is_playing: true,
            is_liked: true,
            volume: 0.84,
            current_track: "Midnight Horizon • Solar Waves",
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct PlayStateIndicator;

#[derive(Component, Clone, Copy, Default)]
struct LikeStateIndicator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Music Streaming Player (Stack Layout & Vector Icons)".into(),
                resolution: (1200, 780).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<PlayerState>()
        .add_systems(Startup, setup_player)
        .add_systems(Update, (update_play_indicator, update_like_indicator))
        .run();
}

fn setup_player(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(player_root_scene(&theme));
}

/// Root scene for the music player.
fn player_root_scene(theme: &Theme) -> impl Scene {
    let icon_font = theme.icon.font.clone();
    let text_font = theme.text.font.inter_regular.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.04, 0.07),
            padding: USides::all(28.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 20.0,
        }
        Children [
            (player_window_card(icon_font.clone(), text_font.clone())),
            (player_footer_note(icon_font, text_font)),
        ]
    }
}

/// The main music player card containing album stack (top) and playback controls (bottom).
fn player_window_card(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(640.0),
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.98),
            border_radius: UCornerRadius::all(20.0),
            padding: USides::all(24.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.35, 0.5, 0.35),
            width: 1.5,
            radius: UCornerRadius::all(20.0),
        }
        UShadow::glow(Color::srgba(0.2, 0.15, 0.45, 0.35), 24.0)
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 18.0,
        }
        Children [
            (top_player_header(icon_font.clone(), text_font.clone())),
            (square_album_stack(icon_font.clone(), text_font.clone())),
            (track_meta_info(icon_font.clone(), text_font.clone())),
            (scrubber_timeline_bar(text_font.clone())),
            (transport_controls_row(icon_font, text_font)),
        ]
    }
}

/// Header with streaming service title and audio quality badge.
fn top_player_header(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
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
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    // Vector Music Icon
                    (
                        UText {
                            text: { Icon::MUSIC.to_string() },
                            font_size: 14.0,
                            font: { icon_font },
                            color: Color::srgb(0.1, 0.85, 0.5),
                        }
                    ),
                    (
                        UText {
                            text: { "AUDIO STREAMING // LOSSLESS MASTER".to_string() },
                            font_size: 12.0,
                            font: { text_font.clone() },
                            color: Color::WHITE,
                        }
                    ),
                ]
            ),
            (
                UNode {
                    background_color: Color::srgba(0.15, 0.2, 0.3, 0.6),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::axes(10.0, 4.0),
                }
                Children [
                    (
                        UText {
                            text: { "24-BIT / 192 kHz ALAC".to_string() },
                            font_size: 10.0,
                            font: { text_font },
                            color: Color::srgb(0.6, 0.8, 1.0),
                        }
                    )
                ]
            )
        ]
    }
}

/// Square 1:1 Album Artwork container using `UDisplay::Stack` with layered overlays!
fn square_album_stack(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    let aspect_ratio_1_1 = 1.0;
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            aspect_ratio: { Some(aspect_ratio_1_1) },
            background_color: Color::srgb(0.06, 0.06, 0.1),
            border_radius: UCornerRadius::all(16.0),
        }
        UBorder {
            color: Color::srgba(0.35, 0.3, 0.6, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(16.0),
        }
        // UDisplay::Stack overlays all children in the same 1:1 square canvas!
        ULayout {
            display: UDisplay::Stack,
            align_items: UAlignItems::Stretch,
        }
        Children [
            // Layer 1: Simulated Album Cover Visual Artwork
            (layer_album_art()),

            // Layer 2: Gloss & Vignette Filter
            (layer_gloss_filter()),

            // Layer 3: Pinned Top-Left "MASTER QUALITY" Badge with Headphones Icon
            (pinned_badge_top_left(icon_font.clone(), text_font)),

            // Layer 4: Center Play/Pause Touch Orb with Vector Icon
            (center_play_button_orb(icon_font)),
        ]
    }
}

/// Layer 1: Simulated album art color field.
fn layer_album_art() -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.12, 0.08, 0.22),
            border_radius: UCornerRadius::all(14.0),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UNode {
                    width: UVal::Px(240.0),
                    height: UVal::Px(240.0),
                    background_color: Color::srgba(0.65, 0.3, 0.95, 0.3),
                    border_radius: UCornerRadius::all(120.0),
                }
                UBorder {
                    color: Color::srgba(0.8, 0.45, 1.0, 0.5),
                    width: 2.0,
                    radius: UCornerRadius::all(120.0),
                }
            )
        ]
    }
}

/// Layer 2: Gloss overlay.
fn layer_gloss_filter() -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.02, 0.02, 0.05, 0.25),
            border_radius: UCornerRadius::all(14.0),
        }
    }
}

/// Layer 3: Pinned top-left badge using absolute positioning with Headphones Icon.
fn pinned_badge_top_left(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgba(0.08, 0.08, 0.14, 0.85),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(12.0, 6.0),
        }
        UBorder {
            color: Color::srgba(0.7, 0.5, 1.0, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        USelf {
            position_type: UPositionType::Absolute,
            left: UVal::Px(16.0),
            top: UVal::Px(16.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        Children [
            (
                UText {
                    text: { Icon::HEADPHONES.to_string() },
                    font_size: 12.0,
                    font: { icon_font },
                    color: Color::srgb(0.85, 0.7, 1.0),
                }
            ),
            (
                UText {
                    text: { "STUDIO MASTER AUDIO".to_string() },
                    font_size: 10.0,
                    font: { text_font },
                    color: Color::srgb(0.85, 0.7, 1.0),
                }
            ),
        ]
    }
}

/// Layer 4: Center Play/Pause button orb with dynamic vector icon.
fn center_play_button_orb(icon_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(68.0),
            height: UVal::Px(68.0),
            background_color: Color::srgba(0.1, 0.85, 0.5, 0.9),
            border_radius: UCornerRadius::all(34.0),
        }
        UBorder {
            color: Color::WHITE,
            width: 2.0,
            radius: UCornerRadius::all(34.0),
        }
        UShadow::glow(Color::srgba(0.1, 0.85, 0.5, 0.5), 20.0)
        USelf {
            align_self: UAlignSelf::Center,
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::srgba(0.1, 0.85, 0.5, 0.9),
            hovered: Color::srgb(0.2, 0.95, 0.6),
            pressed: Color::srgb(0.05, 0.65, 0.35),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                PlayStateIndicator
                UText {
                    text: { Icon::PAUSE.to_string() },
                    font_size: 26.0,
                    font: { icon_font },
                    color: Color::srgb(0.04, 0.05, 0.08),
                }
            )
        ]
        on(|_event: On<Pointer<Click>>, mut state: ResMut<PlayerState>| {
            state.is_playing = !state.is_playing;
            info!("Playback state changed: is_playing = {}", state.is_playing);
        })
    }
}

/// Track title, artist, and interactive like button with Heart Icon.
fn track_meta_info(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
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
            (
                UNode::default()
                USelf {
                    flex_grow: { Some(1.0) },
                    flex_shrink: { Some(1.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 3.0,
                }
                Children [
                    (
                        UText {
                            text: { "Midnight Horizon".to_string() },
                            font_size: 18.0,
                            font: { text_font.clone() },
                            color: Color::WHITE,
                            outline_width: 1.2,
                            outline_color: Color::srgba(0.65, 0.3, 0.95, 0.4),
                            wrap_mode: UTextWrapMode::WrapToParent,
                        }
                    ),
                    (
                        UText {
                            text: { "Solar Waves • Synthwave Symphony (2026)".to_string() },
                            font_size: 13.0,
                            font: { text_font },
                            color: Color::srgb(0.6, 0.6, 0.75),
                            wrap_mode: UTextWrapMode::WrapToParent,
                        }
                    ),
                ]
            ),
            (
                // Interactive Heart / Like Button
                UNode {
                    width: UVal::Px(42.0),
                    height: UVal::Px(42.0),
                    background_color: Color::srgba(0.2, 0.2, 0.3, 0.5),
                    border_radius: UCornerRadius::all(21.0),
                }
                UBorder {
                    color: Color::srgba(0.95, 0.45, 0.65, 0.3),
                    width: 1.0,
                    radius: UCornerRadius::all(21.0),
                }
                USelf {
                    flex_shrink: { Some(0.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                UInteraction::default()
                UInteractionColors {
                    normal: Color::srgba(0.2, 0.2, 0.3, 0.5),
                    hovered: Color::srgba(0.3, 0.25, 0.4, 0.8),
                    pressed: Color::srgba(0.4, 0.2, 0.45, 0.9),
                }
                Children [
                    (
                        LikeStateIndicator
                        UText {
                            text: { Icon::HEART.to_string() },
                            font_size: 18.0,
                            font: { icon_font },
                            color: Color::srgb(0.95, 0.35, 0.55),
                        }
                    )
                ]
                on(|_event: On<Pointer<Click>>, mut state: ResMut<PlayerState>| {
                    state.is_liked = !state.is_liked;
                })
            )
        ]
    }
}

/// Timeline scrubber bar.
fn scrubber_timeline_bar(text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 6.0,
        }
        Children [
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(6.0),
                    background_color: Color::srgb(0.14, 0.15, 0.22),
                    border_radius: UCornerRadius::all(3.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Percent(0.58),
                            height: UVal::Percent(1.0),
                            background_color: Color::srgb(0.1, 0.85, 0.5),
                            border_radius: UCornerRadius::all(3.0),
                        }
                    )
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    (
                        UText {
                            text: { "02:34".to_string() },
                            font_size: 11.0,
                            font: { text_font.clone() },
                            color: Color::srgb(0.55, 0.6, 0.7),
                        }
                    ),
                    (
                        UText {
                            text: { "04:18".to_string() },
                            font_size: 11.0,
                            font: { text_font },
                            color: Color::srgb(0.55, 0.6, 0.7),
                        }
                    ),
                ]
            )
        ]
    }
}

/// Bottom transport buttons row with vector icons.
fn transport_controls_row(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
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
            (icon_control_pill(Icon::SHUFFLE, icon_font.clone())),
            (icon_control_pill(Icon::SKIP_BACK, icon_font.clone())),
            (icon_control_pill(Icon::SKIP_FORWARD, icon_font.clone())),
            (icon_control_pill(Icon::REPEAT, icon_font.clone())),
            (
                // Volume pill with Speaker Icon
                UNode {
                    background_color: Color::srgba(0.12, 0.15, 0.22, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(12.0, 8.0),
                }
                UBorder {
                    color: Color::srgba(0.3, 0.35, 0.5, 0.3),
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
                    (
                        UText {
                            text: { Icon::VOLUME_2.to_string() },
                            font_size: 14.0,
                            font: { icon_font.clone() },
                            color: Color::srgb(0.7, 0.8, 0.95),
                        }
                    ),
                    (
                        UText {
                            text: { "84%".to_string() },
                            font_size: 11.0,
                            font: { text_font },
                            color: Color::srgb(0.7, 0.8, 0.95),
                        }
                    ),
                ]
            ),
        ]
    }
}

/// Reusable icon button pill.
fn icon_control_pill(icon_code: &str, icon_font: Handle<Font>) -> impl Scene {
    let icon_str = icon_code.to_string();
    bsn! {
        UNode {
            width: UVal::Px(44.0),
            height: UVal::Px(36.0),
            background_color: Color::srgba(0.14, 0.15, 0.22, 0.8),
            border_radius: UCornerRadius::all(8.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.35, 0.5, 0.25),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::srgba(0.14, 0.15, 0.22, 0.8),
            hovered: Color::srgb(0.25, 0.28, 0.38),
            pressed: Color::srgb(0.08, 0.1, 0.15),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UText {
                    text: icon_str,
                    font_size: 16.0,
                    font: { icon_font },
                    color: Color::srgb(0.85, 0.88, 0.95),
                }
            )
        ]
    }
}

/// Updates the play/pause icon in real-time.
fn update_play_indicator(
    state: Res<PlayerState>,
    mut text_q: Query<&mut UText, With<PlayStateIndicator>>,
) {
    if state.is_changed() {
        for mut text in &mut text_q {
            text.text = if state.is_playing {
                Icon::PAUSE.to_string()
            } else {
                Icon::PLAY.to_string()
            };
        }
    }
}

/// Updates the heart / favorite icon in real-time.
fn update_like_indicator(
    state: Res<PlayerState>,
    mut text_q: Query<&mut UText, With<LikeStateIndicator>>,
) {
    if state.is_changed() {
        for mut text in &mut text_q {
            text.color = if state.is_liked {
                Color::srgb(0.95, 0.25, 0.5) // Vibrant active pink
            } else {
                Color::srgb(0.45, 0.48, 0.55) // Dim inactive gray
            };
        }
    }
}

/// Footer status bar for music player.
fn player_footer_note(icon_font: Handle<Font>, text_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(640.0),
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
                        font: { icon_font },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    }),
                    (UText {
                        text: { "UDisplay::Stack overlays 1:1 square canvas, gloss filters, and floating controls.".to_string() },
                        font_size: 11.0,
                        font: { text_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    })
                ]
            ),
            (UText {
                text: { "Univis Audio UI".to_string() },
                font_size: 11.0,
                font: { text_font },
                color: Color::srgb(0.1, 0.85, 0.5),
            })
        ]
    }
}
