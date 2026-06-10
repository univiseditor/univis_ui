use bevy::prelude::*;

use crate::layout::geometry::{UCornerRadius, UVal};
use crate::layout::univis_node::{ULayout, UNode};

/// A UI image component.
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
#[require(UNode, ULayout, Visibility)]
pub struct UImage {
    /// The image texture handle.
    pub texture: Handle<Image>,
    /// The tint color of the image.
    pub color: Color,
    /// The width of the image.
    pub width: UVal,
    /// The height of the image.
    pub height: UVal,
    /// Optional custom border radius for the image.
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
    /// Creates a new `UImage` with the given texture handle.
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            texture,
            ..default()
        }
    }

    /// Sets the width and height of the image.
    pub fn with_size(mut self, width: UVal, height: UVal) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets a custom corner radius for the image.
    pub fn with_radius(mut self, radius: UCornerRadius) -> Self {
        self.radius = Some(radius);
        self
    }
}
