use bevy::prelude::*;
use crate::prelude::*;

pub struct UnivisIconButtonPlugin;

impl Plugin for UnivisIconButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UIconButton>()
            .add_systems(Update, attach_icon_button_observers);
    }
}

// ---(Components) ---

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(ULayout, Pickable)]
pub struct UIconButton {
    pub width: UVal,
    pub height: UVal,
    pub padding: USides,
    pub background: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub border_radius: UCornerRadius,
    pub icon: &'static str,
    pub icon_size: f32,
    pub icon_color: Color,
}

impl Default for UIconButton {
    fn default() -> Self {
        Self {
            height: UVal::Content,
            width: UVal::Content,
            padding: USides::axes(16.0, 10.0),
            background: Color::srgb(0.2, 0.5, 0.9),
            hover_color: Color::srgb(0.25, 0.55, 0.95),
            pressed_color: Color::srgb(0.15, 0.45, 0.85),
            border_radius: UCornerRadius::all(8.0),
            icon: Icon::BITCOIN,
            icon_size: 18.0,
            icon_color: Color::WHITE,
        }
    }
}

// علامة داخلية لضمان عدم تكرار المراقب
#[derive(Component)]
struct ButtonObserved;

// --- (Systems) ---

fn attach_icon_button_observers(
    mut commands: Commands,
    query: Query<(Entity, &UIconButton), Without<ButtonObserved>>,
    font_icon: Res<Theme>,
) {
    for (entity, button) in query.iter() {
        commands.entity(entity)
            .insert((
                UNode {
                    width: button.width,
                    height: button.height,
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
                ULayout {
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                    ..default()
                },
                ButtonObserved,
            )).with_children(|icon| {
                icon.spawn((                    
                    UNode::default(),
                    TextColor(button.icon_color),
                    Text2d::new(button.icon),
                    TextFont {
                        font: font_icon.icon.font.clone(),
                        font_size: button.icon_size,
                        ..default()
                    },
                ));
            });
        }
}

// --- Helper Functions ---

impl UIconButton {
    pub fn primary(icon: &'static str) -> Self {
        Self {
            icon: icon,
            background: Color::srgb(0.2, 0.5, 0.9),
            hover_color: Color::srgb(0.25, 0.55, 0.95),
            pressed_color: Color::srgb(0.15, 0.45, 0.85),
            ..default()
        }
    }

    pub fn secondary(icon: &'static str) -> Self {
        Self {
            icon: icon,
            background: Color::srgb(0.4, 0.4, 0.4),
            hover_color: Color::srgb(0.5, 0.5, 0.5),
            pressed_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        }
    }

    pub fn danger(icon: &'static str) -> Self {
        Self {
            icon: icon,
            background: Color::srgb(0.9, 0.2, 0.2),
            hover_color: Color::srgb(0.95, 0.25, 0.25),
            pressed_color: Color::srgb(0.85, 0.15, 0.15),
            ..default()
        }
    }

    pub fn success(icon: &'static str) -> Self {
        Self {
            icon: icon,
            background: Color::srgb(0.2, 0.8, 0.3),
            hover_color: Color::srgb(0.25, 0.85, 0.35),
            pressed_color: Color::srgb(0.15, 0.75, 0.25),
            ..default()
        }
    }
}