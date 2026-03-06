use bevy::prelude::*;

use crate::layout::geometry::{UCornerRadius, UVal};
use crate::layout::univis_node::{ULayout, UNode};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility)]
pub struct UImage {
    pub texture: Handle<Image>,
    pub color: Color,
    pub width: UVal,
    pub height: UVal,
    pub radius: Option<UCornerRadius>,
}

impl Default for UImage {
    fn default() -> Self {
        Self {
            texture: Handle::default(),
            color: Color::WHITE,
            width: UVal::Auto,
            height: UVal::Auto,
            radius: None,
        }
    }
}

impl UImage {
    pub fn new(texture: Handle<Image>) -> Self {
        Self { texture, ..default() }
    }

    pub fn with_size(mut self, width: UVal, height: UVal) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_radius(mut self, radius: UCornerRadius) -> Self {
        self.radius = Some(radius);
        self
    }
}
