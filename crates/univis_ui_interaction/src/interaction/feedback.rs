use bevy::prelude::*;
use univis_ui_engine::layout::univis_node::UNode;

/// Component defining interaction colors.
///
/// Attaching this component to a `UNode` enables automatic hover/press color changes.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_engine::prelude::*;
/// use univis_ui_interaction::prelude::*;
///
/// fn spawn_interactive(commands: &mut Commands) {
///     commands.spawn((
///         UNode {
///             width: UVal::Px(160.0),
///             height: UVal::Px(48.0),
///             background_color: Color::srgb(0.18, 0.24, 0.34),
///             ..default()
///         },
///         UInteractionColors {
///             normal: Color::srgb(0.18, 0.24, 0.34),
///             hovered: Color::srgb(0.24, 0.3, 0.42),
///             pressed: Color::srgb(0.12, 0.18, 0.28),
///         },
///     ));
/// }
/// ```
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
