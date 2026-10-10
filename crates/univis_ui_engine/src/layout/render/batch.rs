use bevy::asset::{AssetServer, Handle, load_embedded_asset};
use bevy::camera::visibility::ViewVisibility;
use bevy::core_pipeline::core_2d::{CORE_2D_DEPTH_FORMAT, Transparent2d};
use bevy::ecs::prelude::*;
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::SystemParamItem;
use bevy::ecs::system::lifetimeless::*;
use bevy::math::{FloatOrd, Vec2};
use bevy::mesh::VertexBufferLayout;
use bevy::prelude::*;
use bevy::render::{
    Extract, ExtractSchedule, Render, RenderApp, RenderSystems,
    render_phase::{
        AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
        RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
    },
    render_resource::{
        BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
        ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
        FragmentState, IndexFormat, MultisampleState, PipelineCache, RawBufferVec,
        RenderPipelineDescriptor, ShaderStages, SpecializedRenderPipeline,
        SpecializedRenderPipelines, StencilFaceState, StencilState, TextureFormat, VertexFormat,
        VertexState, VertexStepMode, binding_types::uniform_buffer,
    },
    renderer::{RenderDevice, RenderQueue},
    sync_world::MainEntity,
    view::{ExtractedView, Msaa, ViewUniform, ViewUniformOffset, ViewUniforms},
};
use bevy::shader::Shader;
use bytemuck::{Pod, Zeroable};

use crate::layout::components::UI3d;
use crate::layout::image::UImage;
use crate::layout::query::ComputedSize;
use crate::layout::render::clip_sync::UActiveClip;
use crate::layout::univis_node::{UBorder, UClip, UNode};
use crate::layout::visual::{UGradient, UInnerGlow, UShadow};
use crate::prelude::UShapeMode;
use crate::schedule::UiRolloutConfig;

/// 160-byte GPU representation of a 2D Univis UI node, sent directly to VRAM in batches.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuNodeInstance {
    pub pos_size: [f32; 4],      // xy = world_position.xy, zw = size.xy
    pub color: [f32; 4],         // LinearRgba (r, g, b, a)
    pub border_color: [f32; 4],  // LinearRgba (r, g, b, a)
    pub radius: [f32; 4],        // (tr, br, tl, bl)
    pub border_params: [f32; 4], // x = width, y = offset, z = softness, w = shape_mode (0=round, 1=cut)
    pub clip_rect: [f32; 4],     // xy = clip_center, zw = clip_size
    pub clip_radius: [f32; 4],   // clip corner radii
    pub shadow_color: [f32; 4],  // shadow color LinearRgba
    pub shadow_params: [f32; 4], // xy = shadow_offset, z = blur, w = spread
    pub flags: [f32; 4],         // x = use_clip (1.0 or 0.0), y = world_z, zw = padding
}

/// Extracted node data passed from Main World to Render World.
pub struct ExtractedNode {
    pub entity: Entity,
    pub main_entity: MainEntity,
    pub z_order: f32,
    pub instance: GpuNodeInstance,
}

#[derive(Resource, Default)]
pub struct ExtractedNodes {
    pub nodes: Vec<ExtractedNode>,
}

#[derive(Resource)]
pub struct UnivisBatchMeta {
    pub view_bind_group: Option<BindGroup>,
    pub instance_buffer: RawBufferVec<GpuNodeInstance>,
    pub index_buffer: RawBufferVec<u32>,
    pub total_instances: u32,
}

impl Default for UnivisBatchMeta {
    fn default() -> Self {
        Self {
            view_bind_group: None,
            instance_buffer: RawBufferVec::new(bevy::render::render_resource::BufferUsages::VERTEX),
            index_buffer: RawBufferVec::new(bevy::render::render_resource::BufferUsages::INDEX),
            total_instances: 0,
        }
    }
}

#[derive(Resource)]
pub struct UnivisBatchPipeline {
    pub view_layout: BindGroupLayoutDescriptor,
    pub shader: Handle<Shader>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnivisPipelineKey {
    pub target_format: TextureFormat,
    pub msaa_samples: u32,
}

impl SpecializedRenderPipeline for UnivisBatchPipeline {
    type Key = UnivisPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let instance_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuNodeInstance>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                // @location(0) pos_size
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                // @location(1) color
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 16,
                    shader_location: 1,
                },
                // @location(2) border_color
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 32,
                    shader_location: 2,
                },
                // @location(3) radius
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 48,
                    shader_location: 3,
                },
                // @location(4) border_params
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 64,
                    shader_location: 4,
                },
                // @location(5) clip_rect
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 80,
                    shader_location: 5,
                },
                // @location(6) clip_radius
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 96,
                    shader_location: 6,
                },
                // @location(7) shadow_color
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 112,
                    shader_location: 7,
                },
                // @location(8) shadow_params
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 128,
                    shader_location: 8,
                },
                // @location(9) flags
                bevy::render::render_resource::VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 144,
                    shader_location: 9,
                },
            ],
        };

        RenderPipelineDescriptor {
            label: Some("univis_batch_pipeline".into()),
            layout: vec![self.view_layout.clone()],
            immediate_size: 0,
            vertex: VertexState {
                shader: self.shader.clone(),
                shader_defs: vec![],
                entry_point: Some("vertex".into()),
                buffers: vec![instance_layout],
            },
            primitive: bevy::render::render_resource::PrimitiveState {
                topology: bevy::render::render_resource::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: bevy::render::render_resource::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: bevy::render::render_resource::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: CORE_2D_DEPTH_FORMAT,
                depth_write_enabled: Some(false),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            multisample: MultisampleState {
                count: key.msaa_samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                shader_defs: vec![],
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format: key.target_format,
                    blend: Some(bevy::render::render_resource::BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            zero_initialize_workgroup_memory: false,
        }
    }
}

pub struct UnivisBatchPlugin;

impl Plugin for UnivisBatchPlugin {
    fn build(&self, app: &mut App) {
        let asset_server = app.world().resource::<AssetServer>().clone();
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let view_layout = BindGroupLayoutDescriptor::new(
            "univis_view_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::VERTEX_FRAGMENT,
                uniform_buffer::<ViewUniform>(true),
            ),
        );

        let shader = load_embedded_asset!(&asset_server, "shaders/unode_batch.wgsl");

        render_app
            .insert_resource(UnivisBatchPipeline {
                view_layout,
                shader,
            })
            .init_resource::<ExtractedNodes>()
            .init_resource::<UnivisBatchMeta>()
            .init_resource::<SpecializedRenderPipelines<UnivisBatchPipeline>>()
            .add_render_command::<Transparent2d, DrawUnivisBatch>()
            .add_systems(ExtractSchedule, extract_univis_nodes)
            .add_systems(
                Render,
                (
                    prepare_univis_nodes.in_set(RenderSystems::PrepareResources),
                    queue_univis_nodes.in_set(RenderSystems::Queue),
                ),
            );
    }
}

/// Extract all visible, pure 2D Univis UI nodes into RenderApp.
fn extract_univis_nodes(
    mut extracted: ResMut<ExtractedNodes>,
    rollout: Extract<Option<Res<UiRolloutConfig>>>,
    nodes_query: Extract<
        Query<
            (
                Entity,
                &ViewVisibility,
                &UNode,
                &ComputedSize,
                &GlobalTransform,
                Option<&UBorder>,
                Option<&UShadow>,
                Option<&UClip>,
                Option<&UActiveClip>,
            ),
            (
                Without<UI3d>,
                Without<UImage>,
                Without<UGradient>,
                Without<UInnerGlow>,
            ),
        >,
    >,
) {
    extracted.nodes.clear();

    if let Some(config) = rollout.as_ref() {
        if !config.use_instanced_batching {
            return;
        }
    }

    for (entity, visibility, node, size, transform, border_opt, shadow_opt, clip_opt, active_clip_opt) in
        nodes_query.iter()
    {
        if !visibility.get() || size.width <= 0.0 || size.height <= 0.0 {
            continue;
        }

        // Skip completely invisible nodes (transparent background, no border, no shadow)
        let bg_alpha = node.background_color.alpha();
        let has_visible_border =
            border_opt.is_some_and(|b| b.width > 0.0 && b.color.alpha() > 0.0001);
        let has_visible_shadow = shadow_opt.is_some_and(|s| s.blur > 0.0 || s.spread > 0.0);
        if bg_alpha <= 0.0001 && !has_visible_border && !has_visible_shadow {
            continue;
        }

        let translation = transform.translation();
        let logical_size = Vec2::new(size.width, size.height);

        let (border_width, border_offset, border_color) = if let Some(b) = border_opt {
            (b.width, b.offset, LinearRgba::from(b.color).to_f32_array())
        } else {
            (0.0, 0.0, [0.0; 4])
        };

        let (shadow_color, shadow_params) = if let Some(s) = shadow_opt {
            (
                LinearRgba::from(s.color).to_f32_array(),
                [s.offset.x, s.offset.y, s.blur, s.spread],
            )
        } else {
            ([0.0; 4], [0.0; 4])
        };

        let (clip_rect, clip_radius, use_clip) = if let Some(active_clip) = active_clip_opt {
            if active_clip.use_clip {
                (
                    [
                        active_clip.center.x,
                        active_clip.center.y,
                        active_clip.size.x,
                        active_clip.size.y,
                    ],
                    [
                        active_clip.radius.x,
                        active_clip.radius.y,
                        active_clip.radius.z,
                        active_clip.radius.w,
                    ],
                    1.0f32,
                )
            } else {
                ([0.0; 4], [0.0; 4], 0.0f32)
            }
        } else if let Some(clip) = clip_opt {
            let radius = [
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            ];
            (
                [translation.x, translation.y, logical_size.x, logical_size.y],
                radius,
                if clip.enabled { 1.0f32 } else { 0.0f32 },
            )
        } else {
            ([0.0; 4], [0.0; 4], 0.0f32)
        };

        let shape_mode_f = match node.shape_mode {
            UShapeMode::Round => 0.0,
            UShapeMode::Cut => 1.0,
        };

        let instance = GpuNodeInstance {
            pos_size: [translation.x, translation.y, logical_size.x, logical_size.y],
            color: LinearRgba::from(node.background_color).to_f32_array(),
            border_color,
            radius: [
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            ],
            border_params: [border_width, border_offset, 0.75, shape_mode_f],
            clip_rect,
            clip_radius,
            shadow_color,
            shadow_params,
            flags: [use_clip, translation.z, 0.0, 0.0],
        };

        extracted.nodes.push(ExtractedNode {
            entity,
            main_entity: entity.into(),
            z_order: translation.z,
            instance,
        });
    }
}

/// Prepare GPU instance and index buffers.
fn prepare_univis_nodes(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<UnivisBatchPipeline>,
    view_uniforms: Res<ViewUniforms>,
    mut meta: ResMut<UnivisBatchMeta>,
    mut extracted: ResMut<ExtractedNodes>,
) {
    meta.instance_buffer.clear();
    meta.total_instances = extracted.nodes.len() as u32;

    if meta.total_instances == 0 {
        return;
    }

    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        meta.view_bind_group = Some(render_device.create_bind_group(
            "univis_view_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipeline.view_layout),
            &BindGroupEntries::single(view_binding),
        ));
    }

    // Sort nodes by Z-order for correct 2D blending
    extracted.nodes.sort_by(|a, b| {
        a.z_order
            .partial_cmp(&b.z_order)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for node in &extracted.nodes {
        meta.instance_buffer.push(node.instance);
    }

    meta.instance_buffer
        .write_buffer(&render_device, &render_queue);

    // Ensure unit quad index buffer (6 indices for 2 triangles)
    if meta.index_buffer.len() != 6 {
        meta.index_buffer.clear();
        meta.index_buffer.push(0);
        meta.index_buffer.push(1);
        meta.index_buffer.push(2);
        meta.index_buffer.push(3);
        meta.index_buffer.push(4);
        meta.index_buffer.push(5);
        meta.index_buffer
            .write_buffer(&render_device, &render_queue);
    }
}

/// Queue batch draw item into Transparent2d phase.
fn queue_univis_nodes(
    draw_functions: Res<DrawFunctions<Transparent2d>>,
    pipeline: Res<UnivisBatchPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<UnivisBatchPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meta: Res<UnivisBatchMeta>,
    extracted: Res<ExtractedNodes>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    views: Query<(Entity, &ExtractedView, &Msaa)>,
) {
    if meta.total_instances == 0 {
        return;
    }

    let draw_function = draw_functions.read().id::<DrawUnivisBatch>();
    let min_z = extracted.nodes.first().map(|n| n.z_order).unwrap_or(0.0);

    for (entity, view, msaa) in views.iter() {
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
        else {
            continue;
        };

        let key = UnivisPipelineKey {
            target_format: view.target_format,
            msaa_samples: msaa.samples(),
        };

        let pipeline_id = pipelines.specialize(&pipeline_cache, &pipeline, key);

        // Add 1 single phase item to draw all instances.
        // NOTE: batch_range MUST be 0..1 in the phase items list so the render loop
        // does not skip subsequent items (like text) in the Transparent2d phase!
        transparent_phase.add_transient(Transparent2d {
            sort_key: FloatOrd(min_z),
            entity: (entity, MainEntity::from(entity)),
            pipeline: pipeline_id,
            draw_function,
            batch_range: 0..1,
            extra_index: PhaseItemExtraIndex::None,
            extracted_index: 0,
            indexed: true,
        });
    }
}

pub type DrawUnivisBatch = (
    SetItemPipeline,
    SetUnivisViewBindGroup<0>,
    DrawUnivisBatchCommand,
);

pub struct SetUnivisViewBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetUnivisViewBindGroup<I> {
    type Param = SRes<UnivisBatchMeta>;
    type ViewQuery = Read<ViewUniformOffset>;
    type ItemQuery = ();

    fn render<'w>(
        _item: &P,
        view_uniform: &'w ViewUniformOffset,
        _entity: Option<()>,
        meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(view_bind_group) = meta.into_inner().view_bind_group.as_ref() else {
            return RenderCommandResult::Failure("view_bind_group not available");
        };

        pass.set_bind_group(I, view_bind_group, &[view_uniform.offset]);
        RenderCommandResult::Success
    }
}

pub struct DrawUnivisBatchCommand;

impl<P: PhaseItem> RenderCommand<P> for DrawUnivisBatchCommand {
    type Param = SRes<UnivisBatchMeta>;
    type ViewQuery = ();
    type ItemQuery = ();

    fn render<'w>(
        _item: &P,
        _view: ROQueryItem<'w, '_, Self::ViewQuery>,
        _entity: Option<()>,
        meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let meta = meta.into_inner();
        let Some(index_buf) = meta.index_buffer.buffer() else {
            return RenderCommandResult::Skip;
        };
        let Some(inst_buf) = meta.instance_buffer.buffer() else {
            return RenderCommandResult::Skip;
        };

        pass.set_index_buffer(index_buf.slice(..), IndexFormat::Uint32);
        pass.set_vertex_buffer(0, inst_buf.slice(..));
        pass.draw_indexed(0..6, 0, 0..meta.total_instances);
        RenderCommandResult::Success
    }
}
