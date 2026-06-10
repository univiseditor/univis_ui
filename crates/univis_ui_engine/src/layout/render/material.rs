use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::*;

/// Custom 2D material shader for Univis UI nodes.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UNodeMaterial {
    // Group 1: Vectors (16 bytes)
    /// Base background color.
    #[uniform(0)]
    pub color: LinearRgba,
    /// Color of the node border.
    #[uniform(0)]
    pub border_color: LinearRgba,
    /// Independent border radii (top-right, bottom-right, top-left, bottom-left).
    #[uniform(0)]
    pub radius: Vec4,

    // Group 2: Mixed
    /// Exact width and height of the node.
    #[uniform(0)]
    pub size: Vec2, // 8 bytes (Offset 48)

    // Group 3: Floats
    /// Thickness of the border stroke.
    #[uniform(0)]
    pub border_width: f32, // 4 bytes (Offset 56)
    /// Offset distance of the border from the element bounds.
    #[uniform(0)]
    pub border_offset: f32, // 4 bytes (Offset 60)
    /// Edge softness for anti-aliasing the SDF rendering.
    #[uniform(0)]
    pub softness: f32, // 4 bytes (Offset 64)

    // Group 4: Integers (u32)
    /// Shape mode selector (0 = Round, 1 = Cut).
    #[uniform(0)]
    pub shape_mode: u32, // 4 bytes (Offset 68)
    /// Flag indicating whether the texture binding should be sampled.
    #[uniform(0)]
    pub use_texture: u32, // 4 bytes (Offset 72)

    /// Final padding to close the uniform block at 80 bytes.
    #[uniform(0)]
    pub _pad: f32, // 4 bytes (Offset 76)

    /// Optional texture to draw inside the node bounds.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,

    /// World-space center of the clip region.
    #[uniform(0)]
    pub clip_center: Vec2, // World-space clip center
    /// Size of the clip region.
    #[uniform(0)]
    pub clip_size: Vec2, // Clip region size
    /// Corner radii of the clip region.
    #[uniform(0)]
    pub clip_radius: Vec4, // Clip corner radii for rounded SDF clipping
    /// Flag indicating whether clipping is enabled (1 = enabled, 0 = disabled).
    #[uniform(0)]
    pub use_clip: u32, // 0 = disabled, 1 = enabled
}

impl Default for UNodeMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE.into(),
            border_color: Color::BLACK.into(),
            radius: Vec4::splat(10.0),
            size: Vec2::new(100.0, 100.0),
            border_width: 0.0,
            border_offset: 0.0,
            softness: 0.5,
            _pad: 0.0,
            texture: None,
            use_texture: 0,
            shape_mode: 0,
            clip_center: Vec2::ZERO,
            clip_size: Vec2::ZERO,
            clip_radius: Vec4::ZERO,
            use_clip: 0,
        }
    }
}

impl Material2d for UNodeMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://univis_ui_engine/layout/render/shaders/unode.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        // Blend keeps transparency, soft edges, and shadow falloff intact.
        AlphaMode2d::Blend
    }
}
