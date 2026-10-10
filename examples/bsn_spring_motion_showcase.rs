//! Interactive Spring Motion, Haptic Punch & Dynamic Transitions Showcase.
//!
//! Features demonstrated:
//! - `UMotionEnter`: Expressive staggered entrance animations with spring translation, scale pop & alpha fade.
//! - `UMotionPunch`: Physical haptic recoil, scale pulses, and directional impact springs.
//! - `UMotionColor`: Smooth continuous color glides for background and border styling.
//! - Physics presets: `UMotionSpringConfig::bouncy()`, `snappy()`, `wobbly()`, `gentle()`.
//!
//! Controls:
//! - [Space]: Trigger scale punch impulse on all cards
//! - [R]: Reset & replay staggered entrance animations
//! - [C]: Cycle color palette glide
//! - [1]: Apply Bouncy spring preset
//! - [2]: Apply Snappy spring preset
//! - [3]: Apply Wobbly spring preset

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct MotionDemoState {
    palette_index: usize,
    current_preset_name: &'static str,
}

impl Default for MotionDemoState {
    fn default() -> Self {
        Self {
            palette_index: 0,
            current_preset_name: "Bouncy",
        }
    }
}

const PALETTES: [Color; 4] = [
    Color::srgb(0.20, 0.65, 1.00), // Cyan / Blue
    Color::srgb(0.70, 0.35, 1.00), // Electric Purple
    Color::srgb(0.25, 0.85, 0.55), // Emerald Green
    Color::srgb(1.00, 0.55, 0.20), // Sunset Amber
];

#[derive(Component, Clone, Copy, Default)]
struct MotionCard(usize);

#[derive(Component, Clone, Copy, Default)]
struct StatusLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Spring Motion & Transitions System".into(),
                resolution: (1280, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<MotionDemoState>()
        .add_systems(Startup, setup_motion_showcase)
        .add_systems(Update, (handle_keyboard_controls, update_status_text))
        .run();
}

fn setup_motion_showcase(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(motion_showcase_scene(&theme));
}

fn motion_showcase_scene(theme: &Theme) -> impl Scene {
    let title_font = theme.text.font.free_serif.clone();
    let body_font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.06, 0.10),
            padding: USides::all(32.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        Children [
            (header_bar(title_font.clone(), body_font.clone(), icon_font)),
            (cards_container(title_font, body_font.clone())),
            (footer_status_bar(body_font)),
        ]
    }
}

fn header_bar(
    title_font: Handle<Font>,
    body_font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    bsn! {
        UMotionEnter::slide_up_with(36.0, UMotionSpringConfig::bouncy(), 0.05)
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.08, 0.11, 0.18, 0.85),
            border_radius: UCornerRadius::all(14.0),
            padding: USides::axes(24.0, 16.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.35, 0.6, 0.4),
            width: 1.0,
            radius: UCornerRadius::all(14.0),
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
                    gap: 14.0,
                }
                Children [
                    (
                        UNode {
                            width: UVal::Px(40.0),
                            height: UVal::Px(40.0),
                            background_color: Color::srgb(0.18, 0.45, 0.95),
                            border_radius: UCornerRadius::all(10.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::ZAP.to_string() },
                                font_size: 20.0,
                                font: { icon_font },
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
                                text: { "SPRING MOTION & HAPTIC SYSTEM".to_string() },
                                font_size: 18.0,
                                font: { title_font },
                                color: Color::srgb(0.95, 0.98, 1.0),
                            }),
                            (UText {
                                text: { "Second-order dynamic springs, punch recoil & continuous color glide".to_string() },
                                font_size: 12.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.60, 0.70, 0.85),
                            })
                        ]
                    )
                ]
            ),
            (
                UNode {
                    background_color: Color::srgba(0.12, 0.18, 0.30, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::axes(14.0, 8.0),
                }
                UBorder {
                    color: Color::srgba(0.3, 0.5, 0.9, 0.3),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "[Space] Punch  •  [R] Re-Enter  •  [C] Color  •  [1-3] Presets".to_string() },
                        font_size: 11.5,
                        font: { body_font },
                        color: Color::srgb(0.75, 0.85, 1.0),
                    })
                ]
            )
        ]
    }
}

fn cards_container(title_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(0.0, 16.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 28.0,
        }
        Children [
            (card_bouncy(title_font.clone(), body_font.clone())),
            (card_snappy(title_font.clone(), body_font.clone())),
            (card_wobbly(title_font, body_font)),
        ]
    }
}

fn card_bouncy(title_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    let accent = Color::srgb(0.20, 0.65, 1.00);
    bsn! {
        MotionCard(0)
        UMotionEnter::slide_up_with(50.0, UMotionSpringConfig::bouncy(), 0.15)
        UMotionPunch::default()
        UMotionColor::default()
        UNode {
            width: UVal::Px(310.0),
            height: UVal::Px(350.0),
            background_color: Color::srgb(0.08, 0.12, 0.20),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(24.0),
        }
        UBorder {
            color: accent,
            width: 1.5,
            radius: UCornerRadius::all(16.0),
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
                    gap: 12.0,
                }
                Children [
                    (UText {
                        text: { "BOUNCY SLIDE".to_string() },
                        font_size: 17.0,
                        font: { title_font },
                        color: accent,
                    }),
                    (UText {
                        text: { "Smooth translation overshoot.\nDamping: 0.75 | Stiffness: 220\nIdeal for modal popups & flyouts.".to_string() },
                        font_size: 13.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.70, 0.78, 0.88),
                    })
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(42.0),
                    background_color: accent,
                    border_radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "PUNCH IMPULSE [SPACE]".to_string() },
                        font_size: 11.5,
                        font: { body_font },
                        color: Color::srgb(0.04, 0.06, 0.10),
                    })
                ]
            )
        ]
    }
}

fn card_snappy(title_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    let accent = Color::srgb(0.70, 0.35, 1.00);
    bsn! {
        MotionCard(1)
        UMotionEnter::pop_with(0.70, UMotionSpringConfig::snappy(), 0.25)
        UMotionPunch::default()
        UMotionColor::default()
        UNode {
            width: UVal::Px(310.0),
            height: UVal::Px(350.0),
            background_color: Color::srgb(0.12, 0.09, 0.22),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(24.0),
        }
        UBorder {
            color: accent,
            width: 1.5,
            radius: UCornerRadius::all(16.0),
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
                    gap: 12.0,
                }
                Children [
                    (UText {
                        text: { "SNAPPY POP".to_string() },
                        font_size: 17.0,
                        font: { title_font },
                        color: accent,
                    }),
                    (UText {
                        text: { "Critical damping without bounce.\nDamping: 1.00 | Stiffness: 380\nHigh-precision responsive action.".to_string() },
                        font_size: 13.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.70, 0.78, 0.88),
                    })
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(42.0),
                    background_color: accent,
                    border_radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "PUNCH IMPULSE [SPACE]".to_string() },
                        font_size: 11.5,
                        font: { body_font },
                        color: Color::srgb(0.04, 0.06, 0.10),
                    })
                ]
            )
        ]
    }
}

fn card_wobbly(title_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    let accent = Color::srgb(0.25, 0.85, 0.55);
    bsn! {
        MotionCard(2)
        UMotionEnter::slide_left_with(60.0, UMotionSpringConfig::wobbly(), 0.35)
        UMotionPunch::default()
        UMotionColor::default()
        UNode {
            width: UVal::Px(310.0),
            height: UVal::Px(350.0),
            background_color: Color::srgb(0.07, 0.16, 0.14),
            border_radius: UCornerRadius::all(16.0),
            padding: USides::all(24.0),
        }
        UBorder {
            color: accent,
            width: 1.5,
            radius: UCornerRadius::all(16.0),
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
                    gap: 12.0,
                }
                Children [
                    (UText {
                        text: { "WOBBLY RECOIL".to_string() },
                        font_size: 17.0,
                        font: { title_font },
                        color: accent,
                    }),
                    (UText {
                        text: { "Underdamped elastic punch.\nDamping: 0.45 | Stiffness: 160\nGreat for haptics & game feedback.".to_string() },
                        font_size: 13.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.70, 0.78, 0.88),
                    })
                ]
            ),
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(42.0),
                    background_color: accent,
                    border_radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "PUNCH IMPULSE [SPACE]".to_string() },
                        font_size: 11.5,
                        font: { body_font },
                        color: Color::srgb(0.04, 0.06, 0.10),
                    })
                ]
            )
        ]
    }
}

fn footer_status_bar(body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UMotionPunch::default()
        UNode {
            background_color: Color::srgba(0.08, 0.11, 0.18, 0.8),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(20.0, 10.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.35, 0.6, 0.3),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                StatusLabel
                UText {
                    text: { "Active Preset: Bouncy (Stiffness: 220, Damping: 0.75)  •  Theme: Palette #1".to_string() },
                    font_size: 12.0,
                    font: { body_font },
                    color: Color::srgb(0.8, 0.88, 1.0),
                }
            )
        ]
    }
}

fn handle_keyboard_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<MotionDemoState>,
    mut query: Query<(&MotionCard, &mut UMotionPunch, &mut UMotionEnter, &mut UMotionColor)>,
) {
    // 1. Space: Trigger Punch
    if keyboard.just_pressed(KeyCode::Space) {
        for (card, mut punch, _, _) in query.iter_mut() {
            let factor = 0.15 + (card.0 as f32) * 0.05;
            punch.punch_scale(Vec2::splat(factor));
            let recoil_y = if card.0 % 2 == 0 { -16.0 } else { 16.0 };
            punch.punch_offset(Vec2::new(0.0, recoil_y));
        }
    }

    // 2. R: Replay Entrance
    if keyboard.just_pressed(KeyCode::KeyR) {
        for (card, _, mut enter, _) in query.iter_mut() {
            *enter = match card.0 {
                0 => UMotionEnter::slide_up(50.0).with_delay(0.05),
                1 => UMotionEnter::pop(0.70).with_delay(0.15),
                _ => UMotionEnter::slide_left(60.0).with_delay(0.25),
            };
        }
    }

    // 3. C: Cycle Color Glide
    if keyboard.just_pressed(KeyCode::KeyC) {
        state.palette_index = (state.palette_index + 1) % PALETTES.len();
        let target_color = PALETTES[state.palette_index];

        for (_, _, _, mut motion_color) in query.iter_mut() {
            *motion_color = UMotionColor::to_border(target_color).with_speed(15.0);
        }
    }

    // 4. Presets 1, 2, 3
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.current_preset_name = "Bouncy";
        for (_, mut punch, _, _) in query.iter_mut() {
            punch.spring = UMotionSpringConfig::bouncy();
        }
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        state.current_preset_name = "Snappy";
        for (_, mut punch, _, _) in query.iter_mut() {
            punch.spring = UMotionSpringConfig::snappy();
        }
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        state.current_preset_name = "Wobbly";
        for (_, mut punch, _, _) in query.iter_mut() {
            punch.spring = UMotionSpringConfig::wobbly();
        }
    }
}

fn update_status_text(
    state: Res<MotionDemoState>,
    mut query: Query<&mut UText, With<StatusLabel>>,
) {
    if state.is_changed() {
        for mut text in query.iter_mut() {
            text.text = format!(
                "Active Preset: {}  •  Color Theme: Palette #{}",
                state.current_preset_name,
                state.palette_index + 1
            );
        }
    }
}
