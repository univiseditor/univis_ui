use crate::internal_prelude::*;
use bevy::prelude::*;

/// Component defining interaction colors.
///
/// Attaching this component to a `UNode` enables automatic hover/press color changes.
#[derive(Component, Clone, Reflect)]
#[require(UInteraction)]
pub struct UInteractionColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// High-level interaction state tracked for interactive UI entities.
#[derive(Component, Clone, Reflect, PartialEq, Default)]
// #[require(Pickable)]
pub enum UInteraction {
    /// The pointer is not hovering or pressing this entity.
    #[default]
    Normal,
    /// A click was completed on this entity.
    Clicked,
    /// The pointer is currently hovering this entity.
    Hovered,
    /// The pointer is currently pressing this entity.
    Pressed,
    /// The pointer was just released over this entity.
    Released,
}

impl Default for UInteractionColors {
    fn default() -> Self {
        UInteractionColors {
            normal: Color::NONE,
            hovered: Color::srgb(0.1, 0.1, 0.1),
            pressed: Color::BLACK,
        }
    }
}
/// Event Handler: Pointer Over (Hover Enter)
#[doc(hidden)]
pub fn on_pointer_over(
    trigger: On<Pointer<Over>>,
    mut query: Query<
        (&mut UInteraction, &mut UNode, Option<&UInteractionColors>),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors)) = query.get_mut(entity) {
        *interaction = UInteraction::Hovered;
        if let Some(color) = colors {
            node.background_color = color.hovered;
        }
    }
}

/// Event Handler: Pointer Click (Click Enter)
#[doc(hidden)]
pub fn on_pointer_click(
    trigger: On<Pointer<Click>>,
    mut query: Query<
        (&mut UInteraction, &mut UNode, Option<&UInteractionColors>),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors)) = query.get_mut(entity) {
        *interaction = UInteraction::Clicked;
        if let Some(color) = colors {
            node.background_color = color.pressed;
        }
    }
}

/// Event Handler: Pointer Out (Hover Exit)
#[doc(hidden)]
pub fn on_pointer_out(
    trigger: On<Pointer<Out>>,
    mut query: Query<
        (&mut UInteraction, &mut UNode, Option<&UInteractionColors>),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors)) = query.get_mut(entity) {
        *interaction = UInteraction::Normal;
        if let Some(color) = colors {
            node.background_color = color.normal;
        }
    }
}

/// Event Handler: Pointer Press (Click Down)
#[doc(hidden)]
pub fn on_pointer_press(
    trigger: On<Pointer<Press>>,
    mut query: Query<
        (&mut UInteraction, &mut UNode, Option<&UInteractionColors>),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors)) = query.get_mut(entity) {
        *interaction = UInteraction::Pressed;
        if let Some(color) = colors {
            node.background_color = color.pressed;
        }
    }
}

/// Event Handler: Pointer Release (Click Up)
#[doc(hidden)]
pub fn on_pointer_release(
    trigger: On<Pointer<Release>>,
    mut query: Query<
        (&mut UInteraction, &mut UNode, Option<&UInteractionColors>),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors)) = query.get_mut(entity) {
        *interaction = UInteraction::Released;
        if let Some(color) = colors {
            node.background_color = color.hovered;
        }
    }
}
