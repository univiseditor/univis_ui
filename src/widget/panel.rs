use bevy::prelude::*;
use crate::prelude::*;

pub struct UnivisPanelPlugin;

impl Plugin for UnivisPanelPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UPanel>()
            .add_systems(Update, sync_panel_visuals);
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct UPanel {
    pub background: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: UCornerRadius,
    pub padding: USides,
    pub gap: f32,
    pub direction: UFlexDirection,
}

impl Default for UPanel {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.13, 0.15, 0.2),
            border_color: Color::srgba(0.75, 0.8, 0.9, 0.2),
            border_width: 1.0,
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(12.0),
            gap: 10.0,
            direction: UFlexDirection::Column,
        }
    }
}

impl UPanel {
    pub fn card() -> Self {
        Self::default()
    }

    pub fn glass() -> Self {
        Self {
            background: Color::srgba(0.08, 0.1, 0.14, 0.72),
            border_color: Color::srgba(0.9, 0.95, 1.0, 0.25),
            ..default()
        }
    }

    pub fn with_padding(mut self, padding: USides) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    pub fn with_direction(mut self, direction: UFlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
}

fn sync_panel_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UPanel, &UNode), Changed<UPanel>>,
) {
    for (entity, panel, existing_node) in query.iter() {
        commands.entity(entity).insert((
            UNode {
                width: existing_node.width,
                height: existing_node.height,
                padding: panel.padding,
                margin: existing_node.margin,
                background_color: panel.background,
                border_radius: panel.border_radius,
                shape_mode: existing_node.shape_mode,
            },
            UBorder {
                color: panel.border_color,
                width: panel.border_width,
                radius: panel.border_radius,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: panel.direction,
                gap: panel.gap,
                ..default()
            },
        ));
    }
}
