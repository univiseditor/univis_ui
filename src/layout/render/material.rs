use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::*;
use bevy::sprite_render::*;

/// Material definition for Univis Nodes.
/// Used to render rounded rectangles using SDF shaders.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// #[derive(Component)]
pub struct UNodeMaterial {
    // Uniform 0: Basic Data
    #[uniform(0)]
    pub color: LinearRgba,
    
    #[uniform(0)]
    pub size: Vec2,        // Box size
    
    #[uniform(0)]
    pub radius: Vec4,      // (TopRight, BottomRight, TopLeft, BottomLeft)
    
    #[uniform(0)]
    pub border_color: LinearRgba,
    
    #[uniform(0)]
    pub border_width: f32,
    
    #[uniform(0)]
    pub softness: f32,     // Antialiasing softness
}

impl Material2d for UNodeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/unode.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}