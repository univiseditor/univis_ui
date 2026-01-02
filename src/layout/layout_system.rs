use bevy::prelude::*;

/// Marker for the Screen Root node (HUD).
///
/// Use this for UI that stays fixed to the camera/screen.
/// It typically takes its size automatically from the window dimensions.
#[derive(Component, Default)]
pub struct UScreenRoot; 

/// Marker for World Space UI Root.
///
/// Use this for UI elements that exist in the 3D world (e.g., floating over a character).
/// You must manually define the `size` (Canvas Size).
#[derive(Component)]
pub struct UWorldRoot {
    pub size: Vec2, 
    /// Scaling factor for text resolution/quality relative to the size.
    pub resolution_scale: f32, 
}

impl Default for UWorldRoot {
    fn default() -> Self {
        Self {
            size: Vec2::new(800.0, 600.0),
            resolution_scale: 1.0,
        }
    }
}