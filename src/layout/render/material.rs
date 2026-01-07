use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::*; // تأكد من استخدام sprite بدلاً من sprite_render

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UNodeMaterial {
    // Group 1: Vectors (16 bytes)
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub border_color: LinearRgba,
    #[uniform(0)]
    pub radius: Vec4,

    // Group 2: Mixed
    #[uniform(0)]
    pub size: Vec2,         // 8 bytes (Offset 48)
    
    // Group 3: Floats
    #[uniform(0)]
    pub border_width: f32,  // 4 bytes (Offset 56)
    #[uniform(0)]
    pub border_offset: f32, // 4 bytes (Offset 60)
    #[uniform(0)]
    pub softness: f32,      // 4 bytes (Offset 64)

    // Group 4: Integers (u32)
    #[uniform(0)]
    pub shape_mode: u32,    // 4 bytes (Offset 68)
    #[uniform(0)]
    pub use_texture: u32,   // 4 bytes (Offset 72)
    
    // الحشو النهائي لإغلاق الكتلة عند 80 بايت
    #[uniform(0)]
    pub _pad: f32,          // 4 bytes (Offset 76)

    // Textures
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
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
            softness: 1.0,
            _pad: 0.0,
            texture: None,
            use_texture: 0,
            shape_mode: 0,
        }
    }
}

impl Material2d for UNodeMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://univis_ui/layout/render/shaders/unode.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        // نستخدم Blend للسماح بالشفافية والظلال والزوايا الناعمة
        AlphaMode2d::Blend
    }
}