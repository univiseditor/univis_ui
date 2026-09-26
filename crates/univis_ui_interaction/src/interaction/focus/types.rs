//! Core data types, components, resources, and events for focus navigation.

use bevy::prelude::*;

/// The input modality that most recently directed focus navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum FocusModality {
    /// Focus was moved using a keyboard (e.g. Tab or Arrow keys).
    #[default]
    Keyboard,
    /// Focus was moved using a gamepad (e.g. D-pad or Analog stick).
    Gamepad,
    /// Focus was moved via direct pointer interaction (e.g. mouse click).
    Pointer,
}

/// 2D Cardinal Direction for spatial navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Hash)]
pub enum NavDirection {
    /// Upwards navigation (+Y in Bevy 2D world space).
    Up,
    /// Downwards navigation (-Y in Bevy 2D world space).
    Down,
    /// Leftwards navigation (-X in Bevy 2D world space).
    Left,
    /// Rightwards navigation (+X in Bevy 2D world space).
    Right,
}

impl NavDirection {
    /// Returns the opposite direction.
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Returns a 2D unit vector corresponding to this direction.
    pub fn as_vec2(self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0.0, 1.0),
            Self::Down => Vec2::new(0.0, -1.0),
            Self::Left => Vec2::new(-1.0, 0.0),
            Self::Right => Vec2::new(1.0, 0.0),
        }
    }
}

/// Component designating a UI entity as focusable.
///
/// Attaching this component allows the node to receive focus via keyboard Tab,
/// directional arrow keys, gamepad D-pad, or analog stick.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_interaction::prelude::*;
///
/// fn spawn_focusable(commands: &mut Commands) {
///     commands.spawn((
///         UFocusable::new().with_tab_index(1),
///         UFocusVisual::glow(Color::srgb(0.2, 0.8, 1.0)),
///     ));
/// }
/// ```
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UFocusable {
    /// Whether this entity can currently receive focus.
    pub enabled: bool,
    /// Explicit tab index for sequential navigation.
    /// Nodes with `Some(idx)` are visited in ascending order before nodes with `None`.
    pub tab_index: Option<i32>,
    /// Explicit entity to focus when navigating Up. Overrides spatial calculation.
    pub focus_up: Option<Entity>,
    /// Explicit entity to focus when navigating Down. Overrides spatial calculation.
    pub focus_down: Option<Entity>,
    /// Explicit entity to focus when navigating Left. Overrides spatial calculation.
    pub focus_left: Option<Entity>,
    /// Explicit entity to focus when navigating Right. Overrides spatial calculation.
    pub focus_right: Option<Entity>,
}

impl Default for UFocusable {
    fn default() -> Self {
        Self {
            enabled: true,
            tab_index: None,
            focus_up: None,
            focus_down: None,
            focus_left: None,
            focus_right: None,
        }
    }
}

impl UFocusable {
    /// Creates a focusable component with default settings (enabled, automatic spatial navigation).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a disabled focusable node that ignores navigation attempts.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Sets the explicit tab index for sequential navigation.
    pub fn with_tab_index(mut self, index: i32) -> Self {
        self.tab_index = Some(index);
        self
    }

    /// Sets an explicit target entity when navigating Up.
    pub fn with_up(mut self, target: Entity) -> Self {
        self.focus_up = Some(target);
        self
    }

    /// Sets an explicit target entity when navigating Down.
    pub fn with_down(mut self, target: Entity) -> Self {
        self.focus_down = Some(target);
        self
    }

    /// Sets an explicit target entity when navigating Left.
    pub fn with_left(mut self, target: Entity) -> Self {
        self.focus_left = Some(target);
        self
    }

    /// Sets an explicit target entity when navigating Right.
    pub fn with_right(mut self, target: Entity) -> Self {
        self.focus_right = Some(target);
        self
    }
}

/// Marker component present only on the currently focused entity.
///
/// Can be queried directly with `Query<Entity, With<UFocused>>`.
#[derive(Component, Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct UFocused;

/// Global settings controlling focus navigation and input behaviors.
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct UFocusNavigationSettings {
    /// Enable directional spatial navigation (Arrow keys, D-pad, Analog stick).
    pub spatial_navigation: bool,
    /// Enable sequential tab navigation (Tab, Shift+Tab).
    pub tab_navigation: bool,
    /// Enable focus activation (Enter, Space, Gamepad South button).
    pub activation: bool,
    /// Deadzone threshold for analog sticks (0.0 .. 1.0). Default: 0.5.
    pub stick_deadzone: f32,
    /// Initial delay in seconds before repeated directional navigation when holding. Default: 0.35s.
    pub repeat_delay: f32,
    /// Interval in seconds between repeated directional navigation steps when held. Default: 0.12s.
    pub repeat_rate: f32,
    /// Whether directional navigation wraps around opposite edges when no candidate exists. Default: false.
    pub wrap_around: bool,
}

impl Default for UFocusNavigationSettings {
    fn default() -> Self {
        Self {
            spatial_navigation: true,
            tab_navigation: true,
            activation: true,
            stick_deadzone: 0.5,
            repeat_delay: 0.35,
            repeat_rate: 0.12,
            wrap_around: false,
        }
    }
}

/// Resource tracking the active focus state and modality.
#[derive(Resource, Debug, Reflect, Clone)]
#[reflect(Resource)]
pub struct UFocusState {
    /// Currently focused entity, if any.
    pub focused: Option<Entity>,
    /// Previously focused entity.
    pub last_focused: Option<Entity>,
    /// Last input modality that caused focus navigation.
    pub modality: FocusModality,
    /// Internal repeat timer for held navigation inputs.
    #[reflect(ignore)]
    pub(crate) repeat_timer: Timer,
    /// Active held navigation direction.
    #[reflect(ignore)]
    pub(crate) held_direction: Option<NavDirection>,
    /// Whether the repeat timer has passed its initial delay phase.
    #[reflect(ignore)]
    pub(crate) repeat_repeating: bool,
}

impl Default for UFocusState {
    fn default() -> Self {
        Self {
            focused: None,
            last_focused: None,
            modality: FocusModality::Keyboard,
            repeat_timer: Timer::from_seconds(0.35, TimerMode::Once),
            held_direction: None,
            repeat_repeating: false,
        }
    }
}

impl UFocusState {
    /// Returns the currently focused entity.
    pub fn focused(&self) -> Option<Entity> {
        self.focused
    }

    /// Returns the previously focused entity.
    pub fn last_focused(&self) -> Option<Entity> {
        self.last_focused
    }

    /// Checks if a specific entity is currently focused.
    pub fn is_focused(&self, entity: Entity) -> bool {
        self.focused == Some(entity)
    }

    /// Returns the current input modality.
    pub fn modality(&self) -> FocusModality {
        self.modality
    }

    /// Clears the focus state.
    pub fn clear(&mut self) {
        self.focused = None;
        self.held_direction = None;
        self.repeat_repeating = false;
    }
}

/// Triggered on an entity when it gains focus.
#[derive(EntityEvent, Clone, Copy, Debug, Reflect, PartialEq)]
pub struct FocusGained {
    /// The entity that received focus.
    pub entity: Entity,
}

/// Triggered on an entity when it loses focus.
#[derive(EntityEvent, Clone, Copy, Debug, Reflect, PartialEq)]
pub struct FocusLost {
    /// The entity that lost focus.
    pub entity: Entity,
}

/// Triggered on the focused entity when an activation input is pressed
/// (e.g. Enter, Spacebar, Gamepad South button).
#[derive(EntityEvent, Clone, Copy, Debug, Reflect, PartialEq)]
pub struct UFocusActivate {
    /// The entity that was activated.
    pub entity: Entity,
}

/// Updates the active focus, dispatching [`FocusLost`] and [`FocusGained`] triggers
/// and inserting/removing the [`UFocused`] marker component.
pub fn set_focus(
    commands: &mut Commands,
    focus_state: &mut UFocusState,
    target: Option<Entity>,
    modality: FocusModality,
) {
    let previous = focus_state.focused;
    if previous == target {
        focus_state.modality = modality;
        return;
    }

    if let Some(prev_entity) = previous {
        commands.entity(prev_entity).remove::<UFocused>();
        commands
            .entity(prev_entity)
            .trigger(|entity| FocusLost { entity });
        focus_state.last_focused = Some(prev_entity);
    }

    focus_state.focused = target;
    focus_state.modality = modality;

    if let Some(new_entity) = target {
        commands.entity(new_entity).insert(UFocused);
        commands
            .entity(new_entity)
            .trigger(|entity| FocusGained { entity });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focusable_builder() {
        let f = UFocusable::new()
            .with_tab_index(5)
            .with_up(Entity::from_bits(1))
            .with_down(Entity::from_bits(2))
            .with_left(Entity::from_bits(3))
            .with_right(Entity::from_bits(4));

        assert!(f.enabled);
        assert_eq!(f.tab_index, Some(5));
        assert_eq!(f.focus_up, Some(Entity::from_bits(1)));
        assert_eq!(f.focus_down, Some(Entity::from_bits(2)));
        assert_eq!(f.focus_left, Some(Entity::from_bits(3)));
        assert_eq!(f.focus_right, Some(Entity::from_bits(4)));

        let disabled = UFocusable::disabled();
        assert!(!disabled.enabled);
    }
}
