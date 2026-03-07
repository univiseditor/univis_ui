use bevy::prelude::*;
use crate::internal_prelude::*;

pub struct UnivisDividerPlugin;

impl Plugin for UnivisDividerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UDivider>()
            .register_type::<UDividerOrientation>()
            .add_systems(Update, sync_divider_visuals);
    }
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum UDividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode)]
pub struct UDivider {
    pub orientation: UDividerOrientation,
    pub thickness: f32,
    pub length: UVal,
    pub color: Color,
    pub margin: USides,
}

impl Default for UDivider {
    fn default() -> Self {
        Self {
            orientation: UDividerOrientation::Horizontal,
            thickness: 1.0,
            length: UVal::Percent(1.0),
            color: Color::srgba(0.8, 0.82, 0.9, 0.35),
            margin: USides::axes(0.0, 6.0),
        }
    }
}

impl UDivider {
    pub fn horizontal() -> Self {
        Self::default()
    }

    pub fn vertical() -> Self {
        Self {
            orientation: UDividerOrientation::Vertical,
            margin: USides::axes(6.0, 0.0),
            ..default()
        }
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness.max(0.5);
        self
    }

    pub fn with_length(mut self, length: UVal) -> Self {
        self.length = length;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

fn sync_divider_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UDivider), Changed<UDivider>>,
) {
    for (entity, divider) in query.iter() {
        let (width, height) = match divider.orientation {
            UDividerOrientation::Horizontal => (divider.length, UVal::Px(divider.thickness)),
            UDividerOrientation::Vertical => (UVal::Px(divider.thickness), divider.length),
        };

        commands.entity(entity).insert(UNode {
            width,
            height,
            margin: divider.margin,
            background_color: divider.color,
            ..default()
        });
    }
}
