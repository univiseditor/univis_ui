use bevy::picking::Pickable;
use bevy::prelude::*;
use univis_ui_engine::layout::geometry::{UCornerRadius, USides};
use univis_ui_engine::layout::univis_node::{ULayout, UNode};
use univis_ui_interaction::interaction::feedback::UInteractionColors;

/// Registers the built-in button widget behavior.
pub struct UnivisButtonPlugin;

impl Plugin for UnivisButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UButton>()
            .add_systems(Update, attach_button_observers);
    }
}

// ---(Components) ---

/// A clickable container-style button widget.
///
/// `UButton` synchronizes its visual state into an underlying [`UNode`] and
/// attaches hover/press colors through [`UInteractionColors`].
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use univis_ui_widgets::prelude::*;
///
/// fn spawn_button(commands: &mut Commands) {
///     commands.spawn(UButton::primary());
/// }
/// ```
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(ULayout, Pickable)]
pub struct UButton {
    pub padding: USides,
    pub background: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub border_radius: UCornerRadius,
}

impl Default for UButton {
    fn default() -> Self {
        Self {
            padding: USides::axes(16.0, 10.0),
            background: Color::srgb(0.2, 0.5, 0.9),
            hover_color: Color::srgb(0.25, 0.55, 0.95),
            pressed_color: Color::srgb(0.15, 0.45, 0.85),
            border_radius: UCornerRadius::all(8.0),
        }
    }
}

// Internal marker to avoid attaching duplicate observers.
#[derive(Component)]
struct ButtonObserved;

// --- Systems ---

fn attach_button_observers(
    mut commands: Commands,
    query: Query<(Entity, &UButton), Without<ButtonObserved>>,
) {
    for (entity, button) in query.iter() {
        commands.entity(entity).insert((
            UNode {
                padding: button.padding,
                background_color: button.background,
                border_radius: button.border_radius,
                ..default()
            },
            UInteractionColors {
                normal: button.background,
                hovered: button.hover_color,
                pressed: button.pressed_color,
            },
            ButtonObserved,
        ));
    }
}

// --- Helper Functions ---

impl UButton {
    /// Returns the default "primary action" button style.
    pub fn primary() -> Self {
        Self {
            background: Color::srgb(0.2, 0.5, 0.9),
            hover_color: Color::srgb(0.25, 0.55, 0.95),
            pressed_color: Color::srgb(0.15, 0.45, 0.85),
            ..default()
        }
    }

    /// Returns a more neutral secondary button style.
    pub fn secondary() -> Self {
        Self {
            background: Color::srgb(0.4, 0.4, 0.4),
            hover_color: Color::srgb(0.5, 0.5, 0.5),
            pressed_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        }
    }

    /// Returns a destructive or danger-styled button.
    pub fn danger() -> Self {
        Self {
            background: Color::srgb(0.9, 0.2, 0.2),
            hover_color: Color::srgb(0.95, 0.25, 0.25),
            pressed_color: Color::srgb(0.85, 0.15, 0.15),
            ..default()
        }
    }

    /// Returns a success-styled button.
    pub fn success() -> Self {
        Self {
            background: Color::srgb(0.2, 0.8, 0.3),
            hover_color: Color::srgb(0.25, 0.85, 0.35),
            pressed_color: Color::srgb(0.15, 0.75, 0.25),
            ..default()
        }
    }
}
