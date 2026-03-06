use bevy::prelude::*;
use crate::prelude::*;

pub struct UnivisButtonPlugin;

impl Plugin for UnivisButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UButton>()
            .add_systems(Update, attach_button_observers);
    }
}

// ---(Components) ---

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

// علامة داخلية لضمان عدم تكرار المراقب
#[derive(Component)]
struct ButtonObserved;

// --- الأنظمة (Systems) ---

fn attach_button_observers(
    mut commands: Commands,
    query: Query<(Entity, &UButton), Without<ButtonObserved>>,
) {
    for (entity, button) in query.iter() {
        commands.entity(entity)
            .insert((
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
    /// إنشاء زر أساسي
    pub fn primary() -> Self {
        Self {
            background: Color::srgb(0.2, 0.5, 0.9),
            hover_color: Color::srgb(0.25, 0.55, 0.95),
            pressed_color: Color::srgb(0.15, 0.45, 0.85),
            ..default()
        }
    }

    /// إنشاء زر ثانوي
    pub fn secondary() -> Self {
        Self {
            background: Color::srgb(0.4, 0.4, 0.4),
            hover_color: Color::srgb(0.5, 0.5, 0.5),
            pressed_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        }
    }

    /// إنشاء زر خطر
    pub fn danger() -> Self {
        Self {
            background: Color::srgb(0.9, 0.2, 0.2),
            hover_color: Color::srgb(0.95, 0.25, 0.25),
            pressed_color: Color::srgb(0.85, 0.15, 0.15),
            ..default()
        }
    }

    /// إنشاء زر نجاح
    pub fn success() -> Self {
        Self {
            background: Color::srgb(0.2, 0.8, 0.3),
            hover_color: Color::srgb(0.25, 0.85, 0.35),
            pressed_color: Color::srgb(0.15, 0.75, 0.25),
            ..default()
        }
    }
}