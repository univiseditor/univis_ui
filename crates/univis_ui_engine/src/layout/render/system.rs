#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use crate::internal_prelude::*;
use bevy::{ecs::relationship::Relationship, prelude::*};
use std::collections::HashMap;

/// Cached material handles attached to a rendered node.
#[derive(Component, Default)]
pub struct MaterialHandles {
    /// Cached 2D material handle.
    pub material_2d: Option<Handle<UNodeMaterial>>,
    /// Cached 3D material handle.
    pub material_3d: Option<Handle<UNodeMaterial3d>>,
}

/// Simple statistics resource for material pooling.
#[derive(Resource, Default)]
pub struct MaterialPool {
    /// Number of material handles reused from the pool.
    pub reused_count: usize,
    /// Number of new material handles created this frame.
    pub created_count: usize,
}

impl MaterialPool {
    /// Resets the pool statistics for the current frame.
    pub fn reset_stats(&mut self) {
        self.reused_count = 0;
        self.created_count = 0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MeshCacheKey {
    width_bits: u32,
    height_bits: u32,
}

impl MeshCacheKey {
    fn from_size(size: Vec2) -> Self {
        Self {
            width_bits: size.x.to_bits(),
            height_bits: size.y.to_bits(),
        }
    }
}

/// Shared rectangle mesh cache keyed by the final logical size used for
/// rendering.
#[derive(Resource, Default)]
pub struct MeshPool {
    meshes_by_size: HashMap<MeshCacheKey, Handle<Mesh>>,
    /// Number of mesh handles reused from the pool.
    pub reused_count: usize,
    /// Number of new mesh handles created this frame.
    pub created_count: usize,
}

impl MeshPool {
    /// Retrieves an existing mesh matching the size from the cache, or creates a new one.
    pub fn mesh_for_size(&mut self, size: Vec2, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        let key = MeshCacheKey::from_size(size);

        if let Some(existing) = self.meshes_by_size.get(&key) {
            self.reused_count += 1;
            return existing.clone();
        }

        let mesh = meshes.add(Rectangle::new(size.x, size.y));
        self.meshes_by_size.insert(key, mesh.clone());
        self.created_count += 1;
        mesh
    }

    /// Resets the pool statistics for the current frame.
    pub fn reset_stats(&mut self) {
        self.reused_count = 0;
        self.created_count = 0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResolvedRenderMode {
    Flat2d,
    World3d,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ResolvedRenderContext {
    mode: ResolvedRenderMode,
    ui_to_world_scale: f32,
}

/// Internal system that refreshes node materials without leaking handles.
#[doc(hidden)]
pub fn update_materials_optimized(
    mut commands: Commands,
    rollout: Option<Res<UiRolloutConfig>>,
    mut cache: ResMut<LayoutCache>,
    mut pool: ResMut<MaterialPool>,
    mut mesh_pool: ResMut<MeshPool>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut nodes: Query<(
        Entity,
        &UNode,
        &ComputedSize,
        Option<&UBorder>,
        Option<&UImage>,
        Option<&UPbr>,
        Option<&CachedUiContext>,
        Option<&mut MaterialHandles>,
    )>,
    // Clipping queries
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: Query<&ResolvedRootUi>,

    // Render resources
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_2d: ResMut<Assets<UNodeMaterial>>,
    mut materials_3d: ResMut<Assets<UNodeMaterial3d>>,
) {
    let start = std::time::Instant::now();
    let use_cached_ui_context = rollout
        .as_ref()
        .map_or(true, |config| config.use_cached_ui_context);
    let use_incremental_render = rollout
        .as_ref()
        .map_or(true, |config| config.use_incremental_render);
    let use_mesh_cache = rollout
        .as_ref()
        .map_or(true, |config| config.use_mesh_cache);
    let created_before = pool.created_count;
    let reused_before = pool.reused_count;
    let mesh_created_before = mesh_pool.created_count;
    let mesh_reused_before = mesh_pool.reused_count;
    let render_frontier = if use_incremental_render {
        cache.take_render_frontier()
    } else {
        cache.all_entities_top_down()
    };

    for entity in render_frontier {
        let Ok((entity, node, size, border, image, pbr_opt, cached_context, handles_opt)) =
            nodes.get_mut(entity)
        else {
            continue;
        };

        sync_entity_material(
            entity,
            node,
            size,
            border,
            image,
            pbr_opt,
            cached_context,
            use_cached_ui_context,
            handles_opt,
            &parents_query,
            &clipper_query,
            &root_query,
            &mut commands,
            &mut pool,
            &mut mesh_pool,
            use_mesh_cache,
            &mut meshes,
            &mut materials_2d,
            &mut materials_3d,
        );
        cache.complete_render(entity);
    }

    if let Some(ref mut prof) = profiler {
        prof.materials_created = pool.created_count - created_before;
        prof.materials_reused = pool.reused_count - reused_before;
        prof.meshes_created = mesh_pool.created_count - mesh_created_before;
        prof.meshes_reused = mesh_pool.reused_count - mesh_reused_before;
        prof.material_update_time = start.elapsed().as_secs_f64() * 1000.0;
    }
}

fn sync_entity_material(
    entity: Entity,
    node: &UNode,
    size: &ComputedSize,
    border: Option<&UBorder>,
    image: Option<&UImage>,
    pbr_opt: Option<&UPbr>,
    cached_context: Option<&CachedUiContext>,
    use_cached_ui_context: bool,
    handles_opt: Option<Mut<'_, MaterialHandles>>,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: &Query<&ResolvedRootUi>,
    commands: &mut Commands,
    pool: &mut MaterialPool,
    mesh_pool: &mut MeshPool,
    use_mesh_cache: bool,
    meshes: &mut Assets<Mesh>,
    materials_2d: &mut Assets<UNodeMaterial>,
    materials_3d: &mut Assets<UNodeMaterial3d>,
) {
    let logical_size = Vec2::new(size.width, size.height);
    if logical_size.x <= 0.0 || logical_size.y <= 0.0 {
        return;
    }

    let cached_context = if use_cached_ui_context {
        cached_context
    } else {
        None
    };
    let render_context = resolve_render_context(entity, cached_context, parents_query, root_query);
    let world_scale = render_context.ui_to_world_scale;
    let size_vec = logical_size * world_scale;
    let softness = minimum_local_softness(world_scale);

    // --- shared material inputs ---
    let (tex_handle, use_tex, base_color) = if let Some(img) = image {
        (Some(img.texture.clone()), 1, LinearRgba::from(img.color))
    } else {
        (None, 0, LinearRgba::from(node.background_color))
    };

    let (b_color, b_offset, b_width) = if let Some(b) = border {
        (
            LinearRgba::from(b.color),
            b.offset * world_scale,
            b.width * world_scale,
        )
    } else {
        (LinearRgba::NONE, 0.0, 0.0)
    };

    let radius = Vec4::new(
        node.border_radius.top_right,
        node.border_radius.bottom_right,
        node.border_radius.top_left,
        node.border_radius.bottom_left,
    ) * world_scale;

    let shape_mode = match node.shape_mode {
        UShapeMode::Round => 0,
        UShapeMode::Cut => 1,
    };

    let (clip_center, clip_size, clip_radius, use_clip) =
        resolve_clipper(cached_context, entity, parents_query, clipper_query);
    let clip_size = clip_size * world_scale;
    let clip_radius = clip_radius * world_scale;

    let mesh = if use_mesh_cache {
        mesh_pool.mesh_for_size(size_vec, meshes)
    } else {
        meshes.add(Rectangle::new(size_vec.x, size_vec.y))
    };

    match render_context.mode {
        ResolvedRenderMode::World3d => {
            let (metallic, roughness, emissive_val) = if let Some(pbr) = pbr_opt {
                (pbr.metallic, pbr.roughness, pbr.emissive.to_vec4())
            } else {
                (0.0, 0.5, Vec4::ZERO)
            };

            let material_handle = if let Some(mut handles) = handles_opt {
                if let Some(existing_handle) = &handles.material_3d {
                    if let Some(mut existing_mat) = materials_3d.get_mut(existing_handle) {
                        existing_mat.color = base_color.to_vec4();
                        existing_mat.size = size_vec;
                        existing_mat.radius = radius;
                        existing_mat.border_color = b_color.to_vec4();
                        existing_mat.emissive = emissive_val;
                        existing_mat.border_width = b_width;
                        existing_mat.softness = softness;
                        existing_mat.metallic = metallic;
                        existing_mat.roughness = roughness;
                        existing_mat.use_texture = use_tex;
                        existing_mat.shape_mode = shape_mode;
                        existing_mat.texture = tex_handle.clone();

                        pool.reused_count += 1;
                        existing_handle.clone()
                    } else {
                        let new_mat = materials_3d.add(create_3d_material(
                            base_color,
                            size_vec,
                            radius,
                            b_color,
                            emissive_val,
                            b_width,
                            softness,
                            metallic,
                            roughness,
                            use_tex,
                            shape_mode,
                            tex_handle.clone(),
                        ));
                        handles.material_3d = Some(new_mat.clone());
                        pool.created_count += 1;
                        new_mat
                    }
                } else {
                    let new_mat = materials_3d.add(create_3d_material(
                        base_color,
                        size_vec,
                        radius,
                        b_color,
                        emissive_val,
                        b_width,
                        softness,
                        metallic,
                        roughness,
                        use_tex,
                        shape_mode,
                        tex_handle.clone(),
                    ));
                    handles.material_3d = Some(new_mat.clone());
                    pool.created_count += 1;
                    new_mat
                }
            } else {
                let new_mat = materials_3d.add(create_3d_material(
                    base_color,
                    size_vec,
                    radius,
                    b_color,
                    emissive_val,
                    b_width,
                    softness,
                    metallic,
                    roughness,
                    use_tex,
                    shape_mode,
                    tex_handle.clone(),
                ));
                commands.entity(entity).insert(MaterialHandles {
                    material_2d: None,
                    material_3d: Some(new_mat.clone()),
                });
                pool.created_count += 1;
                new_mat
            };

            commands
                .entity(entity)
                .insert((Mesh3d(mesh), MeshMaterial3d(material_handle)))
                .remove::<(Mesh2d, MeshMaterial2d<UNodeMaterial>)>();
        }
        ResolvedRenderMode::Flat2d => {
            let material_handle = if let Some(mut handles) = handles_opt {
                if let Some(existing_handle) = &handles.material_2d {
                    if let Some(mut existing_mat) = materials_2d.get_mut(existing_handle) {
                        existing_mat.color = base_color;
                        existing_mat.radius = radius;
                        existing_mat.border_color = b_color;
                        existing_mat.size = size_vec;
                        existing_mat.border_width = b_width;
                        existing_mat.border_offset = b_offset;
                        existing_mat.softness = softness;
                        existing_mat.use_texture = use_tex;
                        existing_mat.shape_mode = shape_mode;
                        existing_mat.texture = tex_handle.clone();
                        existing_mat.clip_center = clip_center;
                        existing_mat.clip_size = clip_size;
                        existing_mat.clip_radius = clip_radius;
                        existing_mat.use_clip = use_clip;

                        pool.reused_count += 1;
                        existing_handle.clone()
                    } else {
                        let new_mat = materials_2d.add(create_2d_material(
                            base_color,
                            radius,
                            b_color,
                            size_vec,
                            b_width,
                            b_offset,
                            softness,
                            use_tex,
                            shape_mode,
                            tex_handle.clone(),
                            clip_center,
                            clip_size,
                            clip_radius,
                            use_clip,
                        ));
                        handles.material_2d = Some(new_mat.clone());
                        pool.created_count += 1;
                        new_mat
                    }
                } else {
                    let new_mat = materials_2d.add(create_2d_material(
                        base_color,
                        radius,
                        b_color,
                        size_vec,
                        b_width,
                        b_offset,
                        softness,
                        use_tex,
                        shape_mode,
                        tex_handle.clone(),
                        clip_center,
                        clip_size,
                        clip_radius,
                        use_clip,
                    ));
                    handles.material_2d = Some(new_mat.clone());
                    pool.created_count += 1;
                    new_mat
                }
            } else {
                let new_mat = materials_2d.add(create_2d_material(
                    base_color,
                    radius,
                    b_color,
                    size_vec,
                    b_width,
                    b_offset,
                    softness,
                    use_tex,
                    shape_mode,
                    tex_handle.clone(),
                    clip_center,
                    clip_size,
                    clip_radius,
                    use_clip,
                ));
                commands.entity(entity).insert(MaterialHandles {
                    material_2d: Some(new_mat.clone()),
                    material_3d: None,
                });
                pool.created_count += 1;
                new_mat
            };

            commands
                .entity(entity)
                .insert((Mesh2d(mesh), MeshMaterial2d(material_handle)))
                .remove::<(Mesh3d, MeshMaterial3d<UNodeMaterial3d>)>();
        }
    }
}

fn resolve_render_context(
    entity: Entity,
    cached_context: Option<&CachedUiContext>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
) -> ResolvedRenderContext {
    if let Some(context) = cached_context
        && context.root_entity.is_some()
    {
        let mode = match context.space {
            UiSpace::World3d => ResolvedRenderMode::World3d,
            UiSpace::Screen | UiSpace::World2d => ResolvedRenderMode::Flat2d,
        };

        return ResolvedRenderContext {
            mode,
            ui_to_world_scale: context.ui_to_world_scale,
        };
    }

    let Some(root) = resolve_root_for_entity(entity, parents_query, root_query) else {
        return ResolvedRenderContext {
            mode: ResolvedRenderMode::Flat2d,
            ui_to_world_scale: 1.0,
        };
    };

    let mode = match root.space {
        UiSpace::World3d => ResolvedRenderMode::World3d,
        UiSpace::Screen | UiSpace::World2d => ResolvedRenderMode::Flat2d,
    };

    ResolvedRenderContext {
        mode,
        ui_to_world_scale: root.ui_units_to_world_scale(),
    }
}

fn resolve_root_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
) -> Option<ResolvedRootUi> {
    let mut current = entity;

    loop {
        if let Ok(root) = root_query.get(current) {
            return Some(*root);
        }

        let parent = parents_query.get(current).ok()?;
        current = parent.get();
    }
}

// ===== Helper Functions =====
// 1. Shared clip resolution helper
fn resolve_clipper(
    cached_context: Option<&CachedUiContext>,
    start_entity: Entity,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> (Vec2, Vec2, Vec4, u32) {
    if let Some(clip_entity) = cached_context.and_then(|context| context.clip_ancestor)
        && let Ok((transform, size, node, clip)) = clipper_query.get(clip_entity)
        && clip.enabled
    {
        let center = transform.translation().truncate();
        let clip_size = Vec2::new(size.width, size.height);
        let radius = Vec4::new(
            node.border_radius.top_right,
            node.border_radius.bottom_right,
            node.border_radius.top_left,
            node.border_radius.bottom_left,
        );
        return (center, clip_size, radius, 1);
    }

    let mut current_entity = start_entity;
    while let Ok(parent) = parents_query.get(current_entity) {
        current_entity = parent.get();
        if let Ok((transform, size, node, clip)) = clipper_query.get(current_entity)
            && clip.enabled
        {
            let center = transform.translation().truncate();
            let clip_size = Vec2::new(size.width, size.height);
            let radius = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );
            return (center, clip_size, radius, 1);
        }
    }
    (Vec2::ZERO, Vec2::ZERO, Vec4::ZERO, 0)
}

fn minimum_local_softness(ui_to_world_scale: f32) -> f32 {
    (0.5 * ui_to_world_scale).max(1.0e-4)
}

// 2. 2D material builder with clip data
fn create_2d_material(
    base_color: LinearRgba,
    radius: Vec4,
    b_color: LinearRgba,
    size_vec: Vec2,
    b_width: f32,
    b_offset: f32,
    softness: f32,
    use_tex: u32,
    shape_mode: u32,
    tex: Option<Handle<Image>>,
    // Clip payload
    clip_center: Vec2,
    clip_size: Vec2,
    clip_radius: Vec4,
    use_clip: u32,
) -> UNodeMaterial {
    UNodeMaterial {
        color: base_color,
        radius,
        border_color: b_color,
        size: size_vec,
        border_width: b_width,
        border_offset: b_offset,
        softness,
        use_texture: use_tex,
        shape_mode,
        texture: tex,
        _pad: 0.0,
        // Clip fields
        clip_center,
        clip_size,
        clip_radius,
        use_clip,
    }
}

// 3. 3D material builder
fn create_3d_material(
    base_color: LinearRgba,
    size_vec: Vec2,
    radius: Vec4,
    b_color: LinearRgba,
    emissive: Vec4,
    b_width: f32,
    softness: f32,
    metallic: f32,
    roughness: f32,
    use_tex: u32,
    shape_mode: u32,
    tex: Option<Handle<Image>>,
) -> UNodeMaterial3d {
    UNodeMaterial3d {
        color: base_color.to_vec4(),
        size: size_vec,
        radius,
        border_color: b_color.to_vec4(),
        emissive,
        border_width: b_width,
        softness,
        metallic,
        roughness,
        use_texture: use_tex,
        shape_mode,
        texture: tex,
    }
}
