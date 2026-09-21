use bevy::prelude::*;
use univis_ui_engine::layout::{
    transition::{UTransition, UTransitionState},
    univis_node::UNode,
};

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

/// Component defining interaction scale (zoom) feedback.
///
/// Attaching this component enables automatic physics-driven hover and press
/// scale transitions via [`UTransition`].
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_engine::prelude::*;
/// use univis_ui_interaction::prelude::*;
///
/// fn spawn_card(commands: &mut Commands) {
///     commands.spawn((
///         UNode {
///             width: UVal::Px(200.0),
///             height: UVal::Px(120.0),
///             ..default()
///         },
///         UInteractionScale::bouncy(),
///         UTransition::bouncy(260.0, 0.65),
///     ));
/// }
/// ```
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[require(UInteraction, UTransition)]
pub struct UInteractionScale {
    /// Normal resting scale factor (default: 1.0).
    pub normal: f32,
    /// Scale factor when hovered (default: 1.05).
    pub hovered: f32,
    /// Scale factor when pressed (default: 0.95).
    pub pressed: f32,
}

impl Default for UInteractionScale {
    fn default() -> Self {
        Self {
            normal: 1.0,
            hovered: 1.05,
            pressed: 0.95,
        }
    }
}

impl UInteractionScale {
    /// Standard subtle zoom (1.05 on hover, 0.95 on press).
    pub fn subtle() -> Self {
        Self::default()
    }

    /// Playful bouncy zoom (1.10 on hover, 0.92 on press).
    pub fn bouncy() -> Self {
        Self {
            normal: 1.0,
            hovered: 1.10,
            pressed: 0.92,
        }
    }

    /// Creates custom zoom factors.
    pub fn new(normal: f32, hovered: f32, pressed: f32) -> Self {
        Self {
            normal,
            hovered,
            pressed,
        }
    }
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
        (
            &mut UInteraction,
            Option<&mut UNode>,
            Option<&UInteractionColors>,
            Option<&UInteractionScale>,
            Option<&mut UTransitionState>,
        ),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors, scale, mut transition_state)) =
        query.get_mut(entity)
    {
        *interaction = UInteraction::Hovered;
        if let (Some(ref mut node), Some(color)) = (node.as_deref_mut(), colors) {
            node.background_color = color.hovered;
        }
        if let Some(scale) = scale {
            if let Some(ref mut state) = transition_state {
                state.target_scale = Vec2::splat(scale.hovered);
            }
        }
    }
}

/// Event Handler: Pointer Click (Click Enter)
#[doc(hidden)]
pub fn on_pointer_click(
    trigger: On<Pointer<Click>>,
    mut query: Query<
        (
            &mut UInteraction,
            Option<&mut UNode>,
            Option<&UInteractionColors>,
        ),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors)) = query.get_mut(entity) {
        *interaction = UInteraction::Clicked;
        if let (Some(ref mut node), Some(color)) = (node.as_deref_mut(), colors) {
            node.background_color = color.pressed;
        }
    }
}

/// Event Handler: Pointer Out (Hover Exit)
#[doc(hidden)]
pub fn on_pointer_out(
    trigger: On<Pointer<Out>>,
    mut query: Query<
        (
            &mut UInteraction,
            Option<&mut UNode>,
            Option<&UInteractionColors>,
            Option<&UInteractionScale>,
            Option<&mut UTransitionState>,
        ),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors, scale, mut transition_state)) =
        query.get_mut(entity)
    {
        *interaction = UInteraction::Normal;
        if let (Some(ref mut node), Some(color)) = (node.as_deref_mut(), colors) {
            node.background_color = color.normal;
        }
        if let Some(scale) = scale {
            if let Some(ref mut state) = transition_state {
                state.target_scale = Vec2::splat(scale.normal);
            }
        }
    }
}

/// Event Handler: Pointer Press (Click Down)
#[doc(hidden)]
pub fn on_pointer_press(
    trigger: On<Pointer<Press>>,
    mut query: Query<
        (
            &mut UInteraction,
            Option<&mut UNode>,
            Option<&UInteractionColors>,
            Option<&UInteractionScale>,
            Option<&mut UTransitionState>,
        ),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors, scale, mut transition_state)) =
        query.get_mut(entity)
    {
        *interaction = UInteraction::Pressed;
        if let (Some(ref mut node), Some(color)) = (node.as_deref_mut(), colors) {
            node.background_color = color.pressed;
        }
        if let Some(scale) = scale {
            if let Some(ref mut state) = transition_state {
                state.target_scale = Vec2::splat(scale.pressed);
            }
        }
    }
}

/// Event Handler: Pointer Release (Click Up)
#[doc(hidden)]
pub fn on_pointer_release(
    trigger: On<Pointer<Release>>,
    mut query: Query<
        (
            &mut UInteraction,
            Option<&mut UNode>,
            Option<&UInteractionColors>,
            Option<&UInteractionScale>,
            Option<&mut UTransitionState>,
        ),
        With<UInteraction>,
    >,
) {
    let entity = trigger.entity.entity();

    if let Ok((mut interaction, mut node, colors, scale, mut transition_state)) =
        query.get_mut(entity)
    {
        *interaction = UInteraction::Released;
        if let (Some(ref mut node), Some(color)) = (node.as_deref_mut(), colors) {
            node.background_color = color.hovered;
        }
        if let Some(scale) = scale {
            if let Some(ref mut state) = transition_state {
                state.target_scale = Vec2::splat(scale.hovered);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_scale_constructors() {
        let def = UInteractionScale::default();
        assert_eq!(def.normal, 1.0);
        assert_eq!(def.hovered, 1.05);
        assert_eq!(def.pressed, 0.95);

        let bouncy = UInteractionScale::bouncy();
        assert_eq!(bouncy.hovered, 1.10);
        assert_eq!(bouncy.pressed, 0.92);

        let custom = UInteractionScale::new(1.0, 1.2, 0.9);
        assert_eq!(custom.normal, 1.0);
        assert_eq!(custom.hovered, 1.2);
        assert_eq!(custom.pressed, 0.9);
    }
}
