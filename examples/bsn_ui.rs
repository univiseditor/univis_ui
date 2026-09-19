//! Demonstrates building a Univis UI interface using Bevy Scene Notation (BSN).
//!
//! This example shows:
//! - Declarative hierarchy authoring with `bsn!` without imperative `commands.spawn` chains.
//! - Sub-scene composition (reusable functions returning `impl Scene`).
//! - Automatic default field population without `..default()`.
//! - Attaching event observers directly within the scene using `on(...)`.

use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup_bsn_ui)
        .run();
}

fn setup_bsn_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn_scene(main_ui_scene());
}

/// The root UI scene combining the HUD canvas, layout container, and nested card.
fn main_ui_scene() -> impl Scene {
    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.04, 0.06, 0.09),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (dashboard_card())
        ]
    }
}

/// A styled dashboard card composed declaratively using pure UNode and ULayout.
fn dashboard_card() -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Px(480.0),
            background_color: Color::srgba(0.09, 0.12, 0.18, 0.95),
            border_radius: UCornerRadius::all(20.0),
            padding: USides::all(28.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.5, 0.8, 0.35),
            width: 1.5,
            radius: UCornerRadius::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 20.0,
        }
        Children [
            (header_section()),
            (content_body()),
            (actions_row()),
        ]
    }
}

/// Header section with title and status badge.
fn header_section() -> impl Scene {
    bsn! {
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            align_items: UAlignItems::Center,
        }
        UNode {
            width: UVal::Percent(1.0),
        }
        Children [
            (UText {
                text: { "Univis BSN Dashboard".to_string() },
                font_size: 24.0,
                color: Color::WHITE,
            }),
            (
                UNode {
                    background_color: Color::srgba(0.1, 0.3, 0.15, 0.8),
                    border_radius: UCornerRadius::all(6.0),
                    padding: USides::axes(8.0, 4.0),
                }
                UBorder {
                    color: Color::srgb(0.2, 0.8, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                Children [
                    (UText {
                        text: { "ACTIVE".to_string() },
                        font_size: 11.0,
                        color: Color::srgb(0.2, 0.9, 0.4),
                    })
                ]
            ),
        ]
    }
}

/// Card body showing descriptive text.
fn content_body() -> impl Scene {
    bsn! {
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
        }
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(0.0, 10.0),
        }
        Children [
            (UText {
                text: { "This interface is authored entirely with Bevy Scene Notation (BSN).".to_string() },
                font_size: 14.0,
                color: Color::srgb(0.75, 0.82, 0.9),
            }),
            (UText {
                text: { "Notice how partial field specification and direct child nesting eliminate boilerplate.".to_string() },
                font_size: 13.0,
                color: Color::srgb(0.55, 0.62, 0.72),
            }),
        ]
    }
}

/// Actions row containing interactive buttons with inline BSN observers.
fn actions_row() -> impl Scene {
    bsn! {
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::End,
            gap: 12.0,
        }
        UNode {
            width: UVal::Percent(1.0),
        }
        Children [
            (
                action_button(
                    "Cancel",
                    Color::srgb(0.2, 0.22, 0.28),
                    Color::srgb(0.26, 0.28, 0.35),
                    Color::srgb(0.14, 0.16, 0.2),
                )
                on(|_event: On<Pointer<Click>>| {
                    info!("Cancel button clicked via BSN observer!");
                })
            ),
            (
                action_button(
                    "Confirm",
                    Color::srgb(0.18, 0.52, 0.92),
                    Color::srgb(0.25, 0.6, 0.98),
                    Color::srgb(0.12, 0.4, 0.75),
                )
                on(|_event: On<Pointer<Click>>| {
                    info!("Confirm button clicked via BSN observer!");
                })
            ),
        ]
    }
}

/// Reusable interactive button generator function returning `impl Scene`.
fn action_button(label: &str, bg: Color, hover: Color, pressed: Color) -> impl Scene {
    let label = label.to_string();
    bsn! {
        UNode {
            background_color: bg,
            border_radius: UCornerRadius::all(10.0),
            padding: USides::axes(20.0, 10.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: bg,
            hovered: hover,
            pressed,
        }
        ULayout {
            display: UDisplay::Flex,
            align_items: UAlignItems::Center,
            justify_content: UJustifyContent::Center,
        }
        Children [
            (UText {
                text: label,
                font_size: 15.0,
                color: Color::WHITE,
            })
        ]
    }
}
