//! Visual focus states and default observers for Univis focus navigation.

use bevy::prelude::*;
use univis_ui_engine::layout::{
    transition::UTransitionState,
    univis_node::{UBorder, UNode},
};

use super::types::{
    FocusGained, FocusLost, FocusModality, UFocusActivate, UFocusState, UFocusable, set_focus,
};
use crate::interaction::feedback::{UInteraction, UInteractionColors, UInteractionScale};

/// Declarative focus styling applied when an entity gains focus and restored when focus is lost.
#[derive(Component, Clone, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UFocusVisual {
    /// Border color applied when focused.
    pub focused_border_color: Option<Color>,
    /// Border width applied when focused.
    pub focused_border_width: Option<f32>,
    /// Background color applied when focused.
    pub focused_background_color: Option<Color>,
    /// Scale factor applied when focused (modifies [`UTransitionState`]).
    pub focused_scale: Option<f32>,
    /// Saved original border color before focus was gained.
    #[reflect(ignore)]
    pub(crate) original_border_color: Option<Color>,
    /// Saved original border width before focus was gained.
    #[reflect(ignore)]
    pub(crate) original_border_width: Option<f32>,
    /// Saved original background color before focus was gained.
    #[reflect(ignore)]
    pub(crate) original_background_color: Option<Color>,
    /// Saved original scale before focus was gained.
    #[reflect(ignore)]
    pub(crate) original_scale: Option<f32>,
    /// Whether this entity already had a UBorder component before focus.
    #[reflect(ignore)]
    pub(crate) had_border_component: bool,
    /// Whether original values have been captured.
    #[reflect(ignore)]
    pub(crate) captured: bool,
}

impl Default for UFocusVisual {
    fn default() -> Self {
        Self {
            focused_border_color: Some(Color::srgb(0.3, 0.7, 1.0)),
            focused_border_width: Some(2.5),
            focused_background_color: None,
            focused_scale: Some(1.05),
            original_border_color: None,
            original_border_width: None,
            original_background_color: None,
            original_scale: None,
            had_border_component: false,
            captured: false,
        }
    }
}

impl UFocusVisual {
    /// Creates a focus visual with a specific border color and width.
    pub fn border(color: Color, width: f32) -> Self {
        Self {
            focused_border_color: Some(color),
            focused_border_width: Some(width),
            focused_background_color: None,
            focused_scale: None,
            ..Default::default()
        }
    }

    /// Creates a focus visual with a standard outline (2.0px).
    pub fn outline(color: Color) -> Self {
        Self::border(color, 2.0)
    }

    /// Creates a focus visual with spring scaling zoom.
    pub fn scale(scale: f32) -> Self {
        Self {
            focused_border_color: None,
            focused_border_width: None,
            focused_background_color: None,
            focused_scale: Some(scale),
            ..Default::default()
        }
    }

    /// Creates a focus visual combining an accent border and subtle zoom.
    pub fn glow(color: Color) -> Self {
        Self {
            focused_border_color: Some(color),
            focused_border_width: Some(2.5),
            focused_background_color: None,
            focused_scale: Some(1.06),
            ..Default::default()
        }
    }

    /// Creates a focus visual that alters background color.
    pub fn background(color: Color) -> Self {
        Self {
            focused_border_color: None,
            focused_border_width: None,
            focused_background_color: Some(color),
            focused_scale: None,
            ..Default::default()
        }
    }

    /// Sets the focused scale factor.
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.focused_scale = Some(scale);
        self
    }

    /// Sets the focused background color.
    pub fn with_background(mut self, color: Color) -> Self {
        self.focused_background_color = Some(color);
        self
    }
}

/// Observer updating visual state when an entity gains focus.
pub fn on_focus_gained_visual(
    trigger: On<FocusGained>,
    mut commands: Commands,
    mut query: Query<(
        Option<&mut UNode>,
        Option<&mut UBorder>,
        &mut UFocusVisual,
        Option<&mut UTransitionState>,
    )>,
) {
    let entity = trigger.entity;
    if let Ok((mut node_opt, mut border_opt, mut visual, mut transition_opt)) =
        query.get_mut(entity)
    {
        if !visual.captured {
            if let Some(ref node) = node_opt {
                visual.original_background_color = Some(node.background_color);
            }
            if let Some(ref border) = border_opt {
                visual.original_border_color = Some(border.color);
                visual.original_border_width = Some(border.width);
                visual.had_border_component = true;
            } else {
                visual.had_border_component = false;
            }
            visual.captured = true;
        }

        // Apply background color
        if let (Some(ref mut node), Some(bg)) =
            (node_opt.as_deref_mut(), visual.focused_background_color)
        {
            node.background_color = bg;
        }

        // Apply border
        if let Some(ref mut border) = border_opt {
            if let Some(color) = visual.focused_border_color {
                border.color = color;
            }
            if let Some(width) = visual.focused_border_width {
                border.width = width;
            }
        } else if visual.focused_border_color.is_some() || visual.focused_border_width.is_some() {
            let color = visual.focused_border_color.unwrap_or(Color::WHITE);
            let width = visual.focused_border_width.unwrap_or(2.0);
            commands.entity(entity).insert(UBorder {
                color,
                width,
                ..Default::default()
            });
        }

        // Apply spring scale
        if let Some(scale) = visual.focused_scale
            && let Some(ref mut state) = transition_opt
        {
            if visual.original_scale.is_none() {
                visual.original_scale = Some(state.target_scale.x);
            }
            state.target_scale = Vec2::splat(scale);
        }
    }
}

/// Observer restoring original styling when an entity loses focus.
pub fn on_focus_lost_visual(
    trigger: On<FocusLost>,
    mut commands: Commands,
    mut query: Query<(
        Option<&mut UNode>,
        Option<&mut UBorder>,
        &mut UFocusVisual,
        Option<&mut UTransitionState>,
    )>,
) {
    let entity = trigger.entity;
    if let Ok((mut node_opt, mut border_opt, mut visual, mut transition_opt)) =
        query.get_mut(entity)
        && visual.captured
    {
        // Restore background color
        if let (Some(ref mut node), Some(bg)) =
            (node_opt.as_deref_mut(), visual.original_background_color)
        {
            node.background_color = bg;
        }

        // Restore border
        if visual.had_border_component {
            if let Some(ref mut border) = border_opt {
                if let Some(color) = visual.original_border_color {
                    border.color = color;
                }
                if let Some(width) = visual.original_border_width {
                    border.width = width;
                }
            }
        } else if border_opt.is_some() {
            // Remove border if it was added only for focus
            commands.entity(entity).remove::<UBorder>();
        }

        // Restore scale
        if let Some(scale) = visual.original_scale
            && let Some(ref mut state) = transition_opt
        {
            state.target_scale = Vec2::splat(scale);
        }
        visual.captured = false;
    }
}

/// Observer setting focus when clicking directly on a focusable node.
pub fn on_pointer_focus_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut focus_state: ResMut<UFocusState>,
    query: Query<&UFocusable>,
) {
    let entity = trigger.entity;
    if let Ok(focusable) = query.get(entity)
        && focusable.enabled
    {
        set_focus(
            &mut commands,
            &mut focus_state,
            Some(entity),
            FocusModality::Pointer,
        );
    }
}

/// Default observer for [`UFocusActivate`], updating interaction state and visual feedback.
pub fn on_focus_activate_default(
    trigger: On<UFocusActivate>,
    mut query: Query<(
        &mut UInteraction,
        Option<&mut UNode>,
        Option<&UInteractionColors>,
        Option<&UInteractionScale>,
        Option<&mut UTransitionState>,
    )>,
) {
    let entity = trigger.entity;
    if let Ok((mut interaction, mut node, colors, scale, mut transition_state)) =
        query.get_mut(entity)
    {
        *interaction = UInteraction::Clicked;
        if let (Some(ref mut node), Some(color)) = (node.as_deref_mut(), colors) {
            node.background_color = color.pressed;
        }
        if let Some(scale) = scale
            && let Some(ref mut state) = transition_state
        {
            state.target_scale = Vec2::splat(scale.pressed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_visual_constructors() {
        let outline = UFocusVisual::outline(Color::srgb(1.0, 0.0, 0.0));
        assert_eq!(
            outline.focused_border_color,
            Some(Color::srgb(1.0, 0.0, 0.0))
        );
        assert_eq!(outline.focused_border_width, Some(2.0));

        let glow = UFocusVisual::glow(Color::srgb(0.0, 1.0, 0.0));
        assert_eq!(glow.focused_scale, Some(1.06));

        let scale = UFocusVisual::scale(1.15);
        assert_eq!(scale.focused_scale, Some(1.15));
    }
}
