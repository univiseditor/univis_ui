//! Builder and spawning helpers for assembling composite text input widgets.

use bevy::prelude::*;
use bevy::text::LineBreak;
use univis_ui_engine::layout::{
    geometry::{UCornerRadius, USides, UVal},
    text::UTextLabel,
    univis_node::{
        UAlignItems, UBorder, UDisplay, UFlexDirection, ULayout, UNode, UPositionType, USelf,
    },
};

use super::types::{UTextInput, UTextInputCaret, UTextInputCaretMarker, UTextInputTextMarker};
use crate::interaction::{
    feedback::UInteraction,
    focus::{UFocusVisual, UFocusable},
};

/// Spawns a complete text input widget with interactive styling, focus visual rings,
/// text label, and animated caret cursor.
pub fn spawn_text_input(commands: &mut Commands, input: UTextInput) -> Entity {
    let font_size = input.font_size;
    let initial_text = if input.value.is_empty() {
        input.placeholder.clone()
    } else {
        super::actions::display_text(&input)
    };
    let initial_color = if input.value.is_empty() {
        input.placeholder_color
    } else {
        input.text_color
    };

    // 1. Spawn parent container
    let parent = commands
        .spawn((
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(42.0),
                padding: USides::axes(14.0, 6.0),
                background_color: Color::srgba(0.08, 0.1, 0.14, 0.95),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                ..default()
            },
            UBorder {
                color: Color::srgba(0.2, 0.25, 0.35, 0.8),
                width: 1.5,
                radius: UCornerRadius::all(6.0),
                offset: 0.0,
            },
            UFocusable::new(),
            UFocusVisual::border(Color::srgb(0.0, 0.9, 1.0), 2.0),
            UInteraction::default(),
            input,
        ))
        .id();

    // 2. Spawn text child
    let text_child = commands
        .spawn((
            ChildOf(parent),
            UNode::default(),
            UTextLabel {
                text: initial_text,
                font_size,
                color: initial_color,
                linebreak: LineBreak::NoWrap,
                autosize: true,
                overflow: univis_ui_engine::layout::text::UTextOverflow::Clip,
                ..default()
            },
            UTextInputTextMarker,
        ))
        .id();

    // 3. Spawn caret child
    let caret_child = commands
        .spawn((
            ChildOf(parent),
            UNode {
                width: UVal::Px(2.0),
                height: UVal::Px(font_size * 1.2),
                background_color: Color::srgb(0.0, 0.9, 1.0),
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(14.0),
                top: UVal::Px(11.0),
                ..default()
            },
            Visibility::Hidden,
            UTextInputCaretMarker,
        ))
        .id();

    // 4. Attach caret state linking children
    let caret = UTextInputCaret {
        text_entity: Some(text_child),
        caret_entity: Some(caret_child),
        ..default()
    };

    commands.entity(parent).insert(caret);

    parent
}
