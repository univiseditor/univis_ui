//! Help Center Knowledge Base & FAQ Accordion - Spring Transitions, Expanding Panels & BSN.
//!
//! Visual Theme: Sleek modern SaaS Knowledge Base (Deep Slate Charcoal, Cyan Accents, Emerald, Clean White).
//!
//! Features demonstrated:
//! - Spring physics transitions: `UTransition::bouncy(220.0, 0.75)` and `UTransition::smooth()`
//! - Expandable accordion items with animated headers, badges, and toggle states
//! - Click observers on accordion headers toggling content visibility and chevron orientation
//! - Responsive text wrapping: `wrap_mode: UTextWrapMode::WrapToParent`
//! - Real-time SDF text outlines on header brand elements
//! - Rich embedded vector icons: `Icon::CIRCLE_HELP`, `Icon::SEARCH`, `Icon::CHEVRON_DOWN`, `Icon::CHEVRON_RIGHT`, `Icon::CODE`, `Icon::THUMBS_UP`, `Icon::THUMBS_DOWN`
//! - Category filter chips (`UInteractionColors`)
//! - Article feedback buttons ("Was this article helpful? Yes / No") with live session counters
//! - Clean, modern, accessible typography with zero visual distortion

use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Resource)]
struct FaqPortalState {
    expanded_items: [bool; 4],
    helpful_votes: u32,
    #[allow(dead_code)]
    active_category: &'static str,
    feedback_message: String,
}

impl Default for FaqPortalState {
    fn default() -> Self {
        Self {
            expanded_items: [true, false, false, false],
            helpful_votes: 142,
            active_category: "ALL QUESTIONS",
            feedback_message: "Showing 4 high-priority architecture guides".to_string(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
struct FaqFeedbackText;

#[derive(Component, Clone, Copy, Default)]
struct HelpfulCounterText;

#[derive(Component, Clone, Copy, Default)]
struct AccordionBodyMarker(usize);

#[derive(Component, Clone, Copy, Default)]
struct AccordionChevronMarker(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Univis UI - Knowledge Base & FAQ Accordion (UTransition & BSN)".into(),
                resolution: (1260, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UnivisUiPlugin)
        .init_resource::<FaqPortalState>()
        .add_systems(Startup, setup_faq_portal)
        .add_systems(Update, sync_faq_portal_ui)
        .run();
}

fn setup_faq_portal(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(faq_portal_scene(&theme));
}

/// Root scene constructing the FAQ portal layout.
fn faq_portal_scene(theme: &Theme) -> impl Scene {
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
            (faq_header_bar(icon_font.clone(), body_font.clone())),
            (faq_search_and_categories_bar(icon_font.clone(), body_font.clone())),
            (faq_accordion_container(icon_font.clone(), body_font.clone())),
            (faq_footer_bar(body_font)),
        ]
    }
}

/// Top Header Bar.
fn faq_header_bar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
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
            // Left: Title and Subtitle with Icon
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
                            width: UVal::Px(38.0),
                            height: UVal::Px(38.0),
                            background_color: Color::srgb(0.2, 0.35, 0.85),
                            border_radius: UCornerRadius::all(10.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::CIRCLE_HELP.to_string() },
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
                                text: { "DEVELOPER KNOWLEDGE BASE & RUNTIME FAQ".to_string() },
                                font_size: 14.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                                outline_width: 1.2,
                                outline_color: Color::srgba(0.25, 0.45, 0.9, 0.5),
                            }),
                            (UText {
                                text: { "Architecture guides, spring transitions, and zero-allocation engine principles".to_string() },
                                font_size: 11.0,
                                font: { body_font.clone() },
                                color: Color::srgb(0.5, 0.55, 0.65),
                                wrap_mode: UTextWrapMode::WrapToParent,
                            })
                        ]
                    )
                ]
            ),
            // Right: Portal Version Pill
            (
                UNode {
                    padding: USides::axes(12.0, 6.0),
                    background_color: Color::srgba(0.12, 0.18, 0.28, 0.7),
                    border_radius: UCornerRadius::all(6.0),
                }
                UBorder {
                    color: Color::srgba(0.25, 0.5, 0.9, 0.4),
                    width: 1.0,
                    radius: UCornerRadius::all(6.0),
                }
                Children [
                    (UText {
                        text: { "UNIVIS SPEC v0.3.1".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.4, 0.75, 1.0),
                    })
                ]
            )
        ]
    }
}

/// Search input bar and category filter chips.
fn faq_search_and_categories_bar(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::axes(0.0, 4.0),
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
            // Left: Category Filter Pills
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                }
                Children [
                    (category_chip(body_font.clone(), "ALL QUESTIONS", true)),
                    (category_chip(body_font.clone(), "CORE ARCHITECTURE", false)),
                    (category_chip(body_font.clone(), "DECLARATIVE BSN", false)),
                    (category_chip(body_font.clone(), "SDF SHADERS", false)),
                    (category_chip(body_font.clone(), "SPRING PHYSICS", false)),
                ]
            ),
            // Right: Mock Search Input with Lucide Icon
            (
                UNode {
                    padding: USides::axes(16.0, 8.0),
                    background_color: Color::srgba(0.08, 0.1, 0.15, 0.8),
                    border_radius: UCornerRadius::all(8.0),
                }
                UBorder {
                    color: Color::srgba(0.25, 0.3, 0.45, 0.35),
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    (UText {
                        text: { Icon::SEARCH.to_string() },
                        font_size: 13.0,
                        font: { icon_font },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    }),
                    (UText {
                        text: { "Search documentation & FAQs...".to_string() },
                        font_size: 11.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.5, 0.55, 0.65),
                    })
                ]
            )
        ]
    }
}

/// Helper for rendering category filter chips.
fn category_chip(body_font: Handle<Font>, label: &'static str, active: bool) -> impl Scene {
    let bg_color = if active {
        Color::srgb(0.2, 0.35, 0.85)
    } else {
        Color::srgba(0.08, 0.1, 0.16, 0.7)
    };
    let text_color = if active {
        Color::WHITE
    } else {
        Color::srgb(0.65, 0.7, 0.8)
    };

    bsn! {
        UNode {
            padding: USides::axes(14.0, 7.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(8.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: bg_color,
            hovered: Color::srgb(0.25, 0.42, 0.95),
            pressed: Color::srgb(0.15, 0.28, 0.7),
        }
        Children [
            (UText {
                text: { label.to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: text_color,
            })
        ]
    }
}

/// Main accordion container hosting the question cards.
fn faq_accordion_container(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.06, 0.08, 0.13, 0.9),
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
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
        }
        Children [
            // Accordion Item 0 (Expanded by default)
            (faq_accordion_item_expanded(
                icon_font.clone(),
                body_font.clone(),
                0,
                "How does Univis achieve zero-allocation reactive layout passes?",
                "CORE ARCHITECTURE",
                Color::srgb(0.38, 0.35, 0.95),
                "Univis UI uses a dual-buffer topological pass (pass-up for intrinsic measurement and pass-down for bounded positioning). All tree measurements are cached in flat Vec arenas without dynamic heap allocations during runtime frames, and automatic word-wrapping expands container height smoothly.",
                "MeasurementCache::get_or_insert_entry() -> O(1) Flat Arena Indexing",
            )),

            // Accordion Item 1
            (faq_accordion_item_collapsed(
                icon_font.clone(),
                body_font.clone(),
                1,
                "Can BSN trees be nested dynamically inside Bevy ECS systems?",
                "DECLARATIVE BSN",
                Color::srgb(0.15, 0.65, 0.45),
                "Yes. The bsn! macro outputs standard Bevy Scene objects that can be spawned via commands.spawn(bsn! { ... }), composed into helper functions, or dynamically conditionally inserted into any active world or screen entity hierarchy.",
                "commands.spawn_scene(my_declarative_widget(theme));",
            )),

            // Accordion Item 2
            (faq_accordion_item_collapsed(
                icon_font.clone(),
                body_font.clone(),
                2,
                "Why use SDF rendering over traditional sprite glyph blitting?",
                "SDF SHADERS",
                Color::srgb(0.85, 0.4, 0.7),
                "Signed Distance Fields enable infinitely sharp text scaling, single-pass real-time outline strokes (outline_width), and multi-colored inline spans in a single GPU draw call without requiring multi-pass blurring or separate border meshes.",
                "let outline = text_label.with_outline(2.5, Color::srgb(0.9, 0.7, 0.2));",
            )),

            // Accordion Item 3
            (faq_accordion_item_collapsed(
                icon_font,
                body_font,
                3,
                "How do spring physics transitions integrate with UI interaction?",
                "SPRING PHYSICS",
                Color::srgb(0.2, 0.7, 0.95),
                "The UTransition component drives second-order spring dynamics (stiffness, damping, mass) that animate scale, opacity, or bounds changes smoothly upon user pointer clicks, hover states, and focus changes.",
                "UTransition::bouncy(220.0, 0.75) // Spring physics on click",
            )),
        ]
    }
}

/// Helper for constructing an expanded accordion item card.
fn faq_accordion_item_expanded(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    idx: usize,
    question: &'static str,
    tag: &'static str,
    tag_color: Color,
    answer: &'static str,
    code_block: &'static str,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.65),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(14.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.35, 0.25),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        UTransition::bouncy(220.0, 0.75)
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
        }
        Children [
            (accordion_header_row(icon_font.clone(), body_font.clone(), idx, question, tag, tag_color, Icon::CHEVRON_DOWN)),
            (accordion_body_expanded(icon_font, body_font, idx, answer, code_block))
        ]
    }
}

/// Helper for constructing a collapsed accordion item card.
fn faq_accordion_item_collapsed(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    idx: usize,
    question: &'static str,
    tag: &'static str,
    tag_color: Color,
    answer: &'static str,
    code_block: &'static str,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.65),
            border_radius: UCornerRadius::all(10.0),
            padding: USides::all(14.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.35, 0.25),
            width: 1.0,
            radius: UCornerRadius::all(10.0),
        }
        USelf {
            flex_shrink: { Some(0.0) },
        }
        UTransition::bouncy(220.0, 0.75)
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
        }
        Children [
            (accordion_header_row(icon_font.clone(), body_font.clone(), idx, question, tag, tag_color, Icon::CHEVRON_RIGHT)),
            (accordion_body_collapsed(icon_font, body_font, idx, answer, code_block))
        ]
    }
}

/// Helper for the interactive header row.
fn accordion_header_row(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    idx: usize,
    question: &'static str,
    tag: &'static str,
    tag_color: Color,
    chevron: &'static str,
) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
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
                USelf {
                    flex_grow: { Some(1.0) },
                    flex_shrink: { Some(1.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    (
                        UNode {
                            padding: USides::axes(8.0, 3.0),
                            background_color: tag_color,
                            border_radius: UCornerRadius::all(4.0),
                        }
                        USelf {
                            flex_shrink: { Some(0.0) },
                        }
                        Children [
                            (UText {
                                text: { tag.to_string() },
                                font_size: 9.0,
                                font: { body_font.clone() },
                                color: Color::WHITE,
                            })
                        ]
                    ),
                    (UText {
                        text: { question.to_string() },
                        font_size: 13.0,
                        font: { body_font.clone() },
                        color: Color::WHITE,
                        wrap_mode: UTextWrapMode::WrapToParent,
                    })
                ]
            ),
            (
                AccordionChevronMarker(idx)
                UNode {
                    width: UVal::Px(28.0),
                    height: UVal::Px(28.0),
                    background_color: Color::srgba(0.15, 0.2, 0.32, 0.8),
                    border_radius: UCornerRadius::all(14.0),
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
                        text: { chevron.to_string() },
                        font_size: 12.0,
                        font: { icon_font },
                        color: Color::srgb(0.7, 0.8, 1.0),
                    })
                ]
            )
        ]
        on(move |_event: On<Pointer<Click>>, mut state: ResMut<FaqPortalState>| {
            state.expanded_items[idx] = !state.expanded_items[idx];
            state.feedback_message = format!(
                "Toggled FAQ topic #{} ({})",
                idx + 1,
                if state.expanded_items[idx] { "Expanded" } else { "Collapsed" }
            );
            info!("{}", state.feedback_message);
        })
    }
}

/// Helper for rendering code snippet box inside an accordion body.
fn faq_code_box(icon_font: Handle<Font>, body_font: Handle<Font>, code: &'static str) -> impl Scene {
    bsn! {
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.06, 0.1, 0.95),
            border_radius: UCornerRadius::all(6.0),
            padding: USides::axes(12.0, 8.0),
        }
        UBorder {
            color: Color::srgba(0.3, 0.5, 0.85, 0.35),
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

/// Body content for expanded accordion item.
fn accordion_body_expanded(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    idx: usize,
    answer: &'static str,
    code_block: &'static str,
) -> impl Scene {
    bsn! {
        AccordionBodyMarker(idx)
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.02, 0.03, 0.06, 0.85),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(12.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.4, 0.2),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        Visibility::Inherited
        UTransition::smooth()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
        }
        Children [
            (UText {
                text: { answer.to_string() },
                font_size: 12.0,
                font: { body_font.clone() },
                color: Color::srgb(0.85, 0.88, 0.94),
                wrap_mode: UTextWrapMode::WrapToParent,
            }),
            (faq_code_box(icon_font.clone(), body_font.clone(), code_block)),
            (accordion_feedback_row(icon_font, body_font))
        ]
    }
}

/// Body content for collapsed accordion item.
fn accordion_body_collapsed(
    icon_font: Handle<Font>,
    body_font: Handle<Font>,
    idx: usize,
    answer: &'static str,
    code_block: &'static str,
) -> impl Scene {
    bsn! {
        AccordionBodyMarker(idx)
        UNode {
            width: UVal::Percent(1.0),
            background_color: Color::srgba(0.02, 0.03, 0.06, 0.85),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(0.0),
        }
        UBorder {
            color: Color::srgba(0.2, 0.25, 0.4, 0.2),
            width: 0.0,
            radius: UCornerRadius::all(8.0),
        }
        Visibility::Hidden
        UTransition::smooth()
        ULayout {
            display: UDisplay::None,
            flex_direction: UFlexDirection::Column,
            gap: 10.0,
        }
        Children [
            (UText {
                text: { answer.to_string() },
                font_size: 12.0,
                font: { body_font.clone() },
                color: Color::srgb(0.85, 0.88, 0.94),
                wrap_mode: UTextWrapMode::WrapToParent,
            }),
            (faq_code_box(icon_font.clone(), body_font.clone(), code_block)),
            (accordion_feedback_row(icon_font, body_font))
        ]
    }
}

/// Helper for the helpfulness feedback row with Lucide icons.
fn accordion_feedback_row(icon_font: Handle<Font>, body_font: Handle<Font>) -> impl Scene {
    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        Children [
            (UText {
                text: { "Was this article helpful?".to_string() },
                font_size: 11.0,
                font: { body_font.clone() },
                color: Color::srgb(0.5, 0.55, 0.65),
            }),
            (
                UNode {
                    padding: USides::axes(10.0, 4.0),
                    background_color: Color::srgba(0.1, 0.25, 0.18, 0.8),
                    border_radius: UCornerRadius::all(6.0),
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
                        text: { Icon::THUMBS_UP.to_string() },
                        font_size: 11.0,
                        font: { icon_font.clone() },
                        color: Color::srgb(0.3, 0.9, 0.6),
                    }),
                    (UText {
                        text: { "YES".to_string() },
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.3, 0.9, 0.6),
                    })
                ]
                on(|_event: On<Pointer<Click>>, mut state: ResMut<FaqPortalState>| {
                    state.helpful_votes += 1;
                    state.feedback_message = format!("Thank you! Helpful score updated: {}", state.helpful_votes);
                    info!("{}", state.feedback_message);
                })
            ),
            (
                UNode {
                    padding: USides::axes(10.0, 4.0),
                    background_color: Color::srgba(0.2, 0.12, 0.15, 0.8),
                    border_radius: UCornerRadius::all(6.0),
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
                        text: { Icon::THUMBS_DOWN.to_string() },
                        font_size: 11.0,
                        font: { icon_font },
                        color: Color::srgb(0.9, 0.45, 0.45),
                    }),
                    (UText {
                        text: { "NO".to_string() },
                        font_size: 10.0,
                        font: { body_font.clone() },
                        color: Color::srgb(0.9, 0.45, 0.45),
                    })
                ]
            )
        ]
    }
}

/// Bottom footer bar.
fn faq_footer_bar(body_font: Handle<Font>) -> impl Scene {
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
                FaqFeedbackText
                UText {
                    text: { "Showing 4 high-priority architecture guides".to_string() },
                    font_size: 11.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.4, 0.85, 0.6),
                }
            ),
            (
                HelpfulCounterText
                UText {
                    text: { "Community Rating: 142 helpful votes".to_string() },
                    font_size: 11.0,
                    font: { body_font.clone() },
                    color: Color::srgb(0.5, 0.55, 0.65),
                }
            )
        ]
    }
}

/// Synchronizes state changes to dynamic text entities and updates accordion visibility.
fn sync_faq_portal_ui(
    state: Res<FaqPortalState>,
    theme: Res<Theme>,
    mut feedback_q: Query<&mut UText, (With<FaqFeedbackText>, Without<HelpfulCounterText>)>,
    mut counter_q: Query<&mut UText, (With<HelpfulCounterText>, Without<FaqFeedbackText>)>,
    mut body_q: Query<(&AccordionBodyMarker, &mut Visibility, &mut ULayout, &mut UNode, &mut UBorder)>,
    mut chevron_q: Query<(&AccordionChevronMarker, &mut UText), (Without<FaqFeedbackText>, Without<HelpfulCounterText>)>,
) {
    if state.is_changed() {
        for mut text in &mut feedback_q {
            text.text = state.feedback_message.clone();
        }
        for mut text in &mut counter_q {
            text.text = format!("Community Rating: {} helpful votes", state.helpful_votes);
        }
        for (marker, mut vis, mut layout, mut node, mut border) in &mut body_q {
            let is_expanded = state.expanded_items[marker.0];
            if is_expanded {
                *vis = Visibility::Inherited;
                layout.display = UDisplay::Flex;
                node.padding = USides::all(12.0);
                border.width = 1.0;
            } else {
                *vis = Visibility::Hidden;
                layout.display = UDisplay::None;
                node.padding = USides::all(0.0);
                border.width = 0.0;
            }
        }
        for (marker, mut text) in &mut chevron_q {
            let is_expanded = state.expanded_items[marker.0];
            text.text = if is_expanded {
                Icon::CHEVRON_DOWN.to_string()
            } else {
                Icon::CHEVRON_RIGHT.to_string()
            };
            text.font = theme.icon.font.clone();
        }
    }
}
