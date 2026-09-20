use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::shader::ShaderRef;

/// Custom 3D material shader for Univis UI nodes in the 3D world.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UNodeMaterial3d {
    // --- group 1: Vec4 values (16-byte aligned) ---
    /// Base background color.
    #[uniform(0)]
    pub color: Vec4,
    /// Independent border radii.
    #[uniform(0)]
    pub radius: Vec4,
    /// Color of the node border.
    #[uniform(0)]
    pub border_color: Vec4,
    /// Emissive color glow for PBR.
    #[uniform(0)]
    pub emissive: Vec4,

    // --- group 2: Vec2 values ---
    /// Exact width and height of the node.
    #[uniform(0)]
    pub size: Vec2,

    // --- group 3: scalar values ---
    /// Thickness of the border stroke.
    #[uniform(0)]
    pub border_width: f32,
    /// Edge softness for anti-aliasing the SDF rendering.
    #[uniform(0)]
    pub softness: f32,
    /// Metallic factor for PBR.
    #[uniform(0)]
    pub metallic: f32,
    /// Roughness factor for PBR.
    #[uniform(0)]
    pub roughness: f32,
    /// Flag indicating whether the texture binding should be sampled.
    #[uniform(0)]
    pub use_texture: u32,

    // --- shape selector ---
    /// Shape mode selector (0 = Round, 1 = Cut).
    #[uniform(0)]
    pub shape_mode: u32,

    // --- visual extensions (16-byte aligned blocks) ---
    /// Gradient start color.
    #[uniform(0)]
    pub grad_start: Vec4,
    /// Gradient end color.
    #[uniform(0)]
    pub grad_end: Vec4,
    /// Gradient parameters: x = type (0=none, 1=linear, 2=radial), y = angle/radius, z = center.x, w = center.y.
    #[uniform(0)]
    pub grad_params: Vec4,

    /// Shadow or outer glow color.
    #[uniform(0)]
    pub shadow_color: Vec4,
    /// Shadow parameters: x = offset.x, y = offset.y, z = blur, w = spread.
    #[uniform(0)]
    pub shadow_params: Vec4,

    /// Inner holographic glow color.
    #[uniform(0)]
    pub inner_glow_color: Vec4,
    /// Inner glow parameters: x = blur, y = active (1.0 or 0.0), z = spread, w = 0.0.
    #[uniform(0)]
    pub inner_glow_params: Vec4,

    // --- texture bindings ---
    /// Optional texture to draw inside the node bounds.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Material for UNodeMaterial3d {
    fn fragment_shader() -> ShaderRef {
        "embedded://univis_ui_engine/layout/render/shaders/unode_3d.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
