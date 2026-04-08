use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::shader::ShaderRef;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UNodeMaterial3d {
    // --- group 1: Vec4 values (16-byte aligned) ---
    #[uniform(0)]
    pub color: Vec4,
    #[uniform(0)]
    pub radius: Vec4,
    #[uniform(0)]
    pub border_color: Vec4,
    #[uniform(0)]
    pub emissive: Vec4,

    // --- group 2: Vec2 values ---
    #[uniform(0)]
    pub size: Vec2,

    // --- group 3: scalar values ---
    #[uniform(0)]
    pub border_width: f32,
    #[uniform(0)]
    pub softness: f32,
    #[uniform(0)]
    pub metallic: f32,
    #[uniform(0)]
    pub roughness: f32,
    #[uniform(0)]
    pub use_texture: u32,

    // --- shape selector ---
    // 0 = Round, 1 = Cut
    #[uniform(0)]
    pub shape_mode: u32,

    // --- texture bindings ---
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
