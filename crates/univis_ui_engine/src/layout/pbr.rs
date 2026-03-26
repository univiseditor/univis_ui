use bevy::prelude::*;

/// Physically based material overrides for `World3d` UI content.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct UPbr {
    /// Surface metallic factor, where `0.0` is dielectric and `1.0` is metallic.
    pub metallic: f32,

    /// Surface roughness, where `0.0` is mirror-like and `1.0` is very rough.
    pub roughness: f32,

    /// Emissive contribution for self-lit UI surfaces.
    pub emissive: LinearRgba,
}

impl Default for UPbr {
    fn default() -> Self {
        Self {
            metallic: 0.0,
            roughness: 0.5,
            emissive: LinearRgba::BLACK,
        }
    }
}
