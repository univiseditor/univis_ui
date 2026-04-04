use crate::internal_prelude::*;
use bevy::asset::{Asset, AssetId, Assets, RenderAssetUsages};
use bevy::color::LinearRgba;
use bevy::ecs::relationship::Relationship;
use bevy::image::{DynamicTextureAtlasBuilder, ImageSampler, TextureAtlasLayout};
use bevy::mesh::{Indices, Mesh, Mesh2d, PrimitiveTopology};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, Extent3d, TextureDimension, TextureFormat};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d};
use bevy::text::{ComputedTextBlock, CosmicFontSystem};
use std::collections::HashMap;

use super::model::{TextChildMarker, UTextLabel, UTextLabelLayoutCache, UTextOverflow};

const DEFAULT_TEXT_EDGE_SOFTNESS: f32 = 0.75;
const TEXT_SDF_PADDING: u32 = 12;
const TEXT_SDF_ATLAS_SIZE: u32 = 1024;
const DISTANCE_FIELD_INF: f32 = 1.0e20;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct TextRenderPageMarker {
    texture_id: AssetId<Image>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct UTextLabelSdfMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(0)]
    clip_radius: Vec4,
    #[uniform(0)]
    clip_center: Vec2,
    #[uniform(0)]
    clip_size: Vec2,
    #[uniform(0)]
    edge_softness: f32,
    #[uniform(0)]
    use_clip: u32,
    #[uniform(0)]
    _pad: Vec2,
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl Material2d for UTextLabelSdfMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://univis_ui_widgets/widget/shaders/text_label_sdf.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Resource, Default)]
pub(super) struct UTextLabelAtlasCache {
    pages_by_font: HashMap<UTextLabelAtlasKey, Vec<UTextLabelAtlasPage>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UTextLabelAtlasKey {
    font_id: AssetId<Font>,
    font_size_bits: u32,
    font_weight: u16,
    flags_bits: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UTextLabelGlyphKey {
    glyph_id: u16,
    x_bin_bits: u32,
    y_bin_bits: u32,
}

struct UTextLabelAtlasPage {
    texture: Handle<Image>,
    texture_atlas: Handle<TextureAtlasLayout>,
    builder: DynamicTextureAtlasBuilder,
    glyphs: HashMap<UTextLabelGlyphKey, UTextLabelGlyphAtlasInfo>,
}

#[derive(Clone)]
struct UTextLabelGlyphAtlasInfo {
    texture: Handle<Image>,
    texture_atlas: Handle<TextureAtlasLayout>,
    glyph_index: usize,
    offset: IVec2,
}

#[derive(Clone, Debug)]
pub(super) struct TextGlyphQuad {
    pub(super) center: Vec2,
    pub(super) size: Vec2,
    pub(super) uv_min: Vec2,
    pub(super) uv_max: Vec2,
}

#[derive(Clone)]
struct TextPageBatch {
    texture: Handle<Image>,
    quads: Vec<TextGlyphQuad>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct LocalClipRect {
    pub(super) min: Vec2,
    pub(super) max: Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
struct MaterialClipInfo {
    center: Vec2,
    size: Vec2,
    radius: Vec4,
    use_clip: u32,
}

fn resolved_render_scale(render_scale: f32) -> f32 {
    if render_scale.is_finite() && render_scale >= 1.0 {
        render_scale
    } else {
        1.0
    }
}

pub(super) fn text_horizontal_offset(
    label: &UTextLabel,
    node: &UNode,
    computed_size: &ComputedSize,
    text_size: Vec2,
) -> f32 {
    let content_width = (computed_size.width - node.padding.width_sum()).max(0.0);

    match label.justify {
        Justify::Left => (-content_width * 0.5) + (text_size.x * 0.5),
        Justify::Center | Justify::Justified => 0.0,
        Justify::Right => (content_width * 0.5) - (text_size.x * 0.5),
    }
}

fn extract_mask_alpha(data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let pixel_count = width * height;
    match data.len() {
        len if len == pixel_count => data.to_vec(),
        len if len == pixel_count * 4 => data.chunks_exact(4).map(|px| px[3]).collect(),
        len if len == pixel_count * 3 => data
            .chunks_exact(3)
            .map(|px| ((u16::from(px[0]) + u16::from(px[1]) + u16::from(px[2])) / 3) as u8)
            .collect(),
        _ => vec![0; pixel_count],
    }
}

fn squared_distance_transform_1d(f: &[f32], d: &mut [f32], v: &mut [usize], z: &mut [f32]) {
    let n = f.len();
    if n == 0 {
        return;
    }

    let mut k = 0usize;
    v[0] = 0;
    z[0] = -DISTANCE_FIELD_INF;
    z[1] = DISTANCE_FIELD_INF;

    for q in 1..n {
        let mut s;
        loop {
            let p = v[k];
            s = ((f[q] + (q * q) as f32) - (f[p] + (p * p) as f32)) / (2.0 * (q - p) as f32);
            if s > z[k] || k == 0 {
                break;
            }
            k -= 1;
        }

        if s <= z[k] && k == 0 {
            z[1] = DISTANCE_FIELD_INF;
            v[0] = q;
            continue;
        }

        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = DISTANCE_FIELD_INF;
    }

    k = 0;
    for q in 0..n {
        while z[k + 1] < q as f32 {
            k += 1;
        }
        let p = v[k];
        d[q] = (q as f32 - p as f32).powi(2) + f[p];
    }
}

fn distance_transform(mask: &[bool], width: usize, height: usize) -> Vec<f32> {
    let mut grid = vec![0.0; width * height];
    for (dst, &is_feature) in grid.iter_mut().zip(mask.iter()) {
        *dst = if is_feature { 0.0 } else { DISTANCE_FIELD_INF };
    }

    let mut tmp = vec![0.0; width * height];
    let mut input = vec![0.0; width.max(height)];
    let mut output = vec![0.0; width.max(height)];
    let mut v = vec![0usize; width.max(height)];
    let mut z = vec![0.0; width.max(height) + 1];

    for x in 0..width {
        for y in 0..height {
            input[y] = grid[y * width + x];
        }
        squared_distance_transform_1d(
            &input[..height],
            &mut output[..height],
            &mut v[..height],
            &mut z[..=height],
        );
        for y in 0..height {
            tmp[y * width + x] = output[y];
        }
    }

    let mut out = vec![0.0; width * height];
    for y in 0..height {
        let row = &tmp[y * width..(y + 1) * width];
        input[..width].copy_from_slice(row);
        squared_distance_transform_1d(
            &input[..width],
            &mut output[..width],
            &mut v[..width],
            &mut z[..=width],
        );
        out[y * width..(y + 1) * width].copy_from_slice(&output[..width]);
    }

    out
}

fn build_sdf_glyph_image(mask: &[u8], width: u32, height: u32) -> Image {
    let padded_width = width + TEXT_SDF_PADDING * 2;
    let padded_height = height + TEXT_SDF_PADDING * 2;
    let padded_len = (padded_width * padded_height) as usize;
    let mut inside_mask = vec![false; padded_len];
    let mut coverage = vec![0.0; padded_len];

    for y in 0..height as usize {
        for x in 0..width as usize {
            let src = y * width as usize + x;
            let dst = (y + TEXT_SDF_PADDING as usize) * padded_width as usize
                + (x + TEXT_SDF_PADDING as usize);
            let alpha = f32::from(mask[src]) / 255.0;
            inside_mask[dst] = alpha >= 0.5;
            coverage[dst] = alpha;
        }
    }

    let outside_mask: Vec<bool> = inside_mask.iter().map(|inside| !inside).collect();
    let distance_to_inside =
        distance_transform(&inside_mask, padded_width as usize, padded_height as usize);
    let distance_to_outside =
        distance_transform(&outside_mask, padded_width as usize, padded_height as usize);
    let spread = TEXT_SDF_PADDING as f32;

    let mut data = Vec::with_capacity(padded_len * 4);
    for idx in 0..padded_len {
        let signed_distance = distance_to_outside[idx].sqrt() - distance_to_inside[idx].sqrt()
            + (coverage[idx] - 0.5);
        let alpha = (0.5 + 0.5 * (signed_distance / spread)).clamp(0.0, 1.0);
        let alpha_u8 = (alpha * 255.0).round() as u8;
        data.extend_from_slice(&[255, 255, 255, alpha_u8]);
    }

    let mut image = Image::new(
        Extent3d {
            width: padded_width,
            height: padded_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::linear();
    image
}

fn create_text_atlas_page(
    images: &mut Assets<Image>,
    texture_atlases: &mut Assets<TextureAtlasLayout>,
    minimum_size: u32,
) -> UTextLabelAtlasPage {
    let size = minimum_size.max(TEXT_SDF_ATLAS_SIZE).next_power_of_two();
    let mut atlas_image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    atlas_image.sampler = ImageSampler::linear();
    let texture = images.add(atlas_image);
    let texture_atlas = texture_atlases.add(TextureAtlasLayout::new_empty(UVec2::splat(size)));

    UTextLabelAtlasPage {
        texture,
        texture_atlas,
        builder: DynamicTextureAtlasBuilder::new(UVec2::splat(size), 2),
        glyphs: HashMap::default(),
    }
}

pub(super) fn clip_quad_to_rect(
    quad: TextGlyphQuad,
    clip_rect: LocalClipRect,
) -> Option<TextGlyphQuad> {
    let half = quad.size * 0.5;
    let original_min = quad.center - half;
    let original_max = quad.center + half;

    let clipped_min = Vec2::new(
        original_min.x.max(clip_rect.min.x),
        original_min.y.max(clip_rect.min.y),
    );
    let clipped_max = Vec2::new(
        original_max.x.min(clip_rect.max.x),
        original_max.y.min(clip_rect.max.y),
    );

    if clipped_min.x >= clipped_max.x || clipped_min.y >= clipped_max.y {
        return None;
    }

    let original_width = original_max.x - original_min.x;
    let original_height = original_max.y - original_min.y;
    if original_width <= 0.0 || original_height <= 0.0 {
        return None;
    }

    let left_t = (clipped_min.x - original_min.x) / original_width;
    let right_t = (clipped_max.x - original_min.x) / original_width;
    let bottom_t = (clipped_min.y - original_min.y) / original_height;
    let top_t = (clipped_max.y - original_min.y) / original_height;

    let uv_left = quad.uv_min.x + (quad.uv_max.x - quad.uv_min.x) * left_t;
    let uv_right = quad.uv_min.x + (quad.uv_max.x - quad.uv_min.x) * right_t;
    let uv_bottom = quad.uv_max.y + (quad.uv_min.y - quad.uv_max.y) * bottom_t;
    let uv_top = quad.uv_max.y + (quad.uv_min.y - quad.uv_max.y) * top_t;

    Some(TextGlyphQuad {
        center: (clipped_min + clipped_max) * 0.5,
        size: clipped_max - clipped_min,
        uv_min: Vec2::new(uv_left, uv_top),
        uv_max: Vec2::new(uv_right, uv_bottom),
    })
}

pub(super) fn label_content_clip_rect(
    label: &UTextLabel,
    node: &UNode,
    computed_size: &ComputedSize,
) -> Option<LocalClipRect> {
    if label.overflow == UTextOverflow::Visible {
        return None;
    }

    if computed_size.width <= 0.0 || computed_size.height <= 0.0 {
        return None;
    }

    let width = (computed_size.width - node.padding.width_sum()).max(0.0);
    let height = (computed_size.height - node.padding.height_sum()).max(0.0);

    Some(LocalClipRect {
        min: Vec2::new(-width * 0.5, -height * 0.5),
        max: Vec2::new(width * 0.5, height * 0.5),
    })
}

fn build_text_page_mesh(quads: &[TextGlyphQuad]) -> Mesh {
    let mut positions = Vec::with_capacity(quads.len() * 4);
    let mut normals = Vec::with_capacity(quads.len() * 4);
    let mut uvs = Vec::with_capacity(quads.len() * 4);
    let mut indices = Vec::with_capacity(quads.len() * 6);

    for (quad_index, quad) in quads.iter().enumerate() {
        let base = (quad_index * 4) as u32;
        let half = quad.size * 0.5;
        let min = quad.center - half;
        let max = quad.center + half;

        positions.extend_from_slice(&[
            [min.x, min.y, 0.0],
            [max.x, min.y, 0.0],
            [min.x, max.y, 0.0],
            [max.x, max.y, 0.0],
        ]);
        normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
        uvs.extend_from_slice(&[
            [quad.uv_min.x, quad.uv_max.y],
            [quad.uv_max.x, quad.uv_max.y],
            [quad.uv_min.x, quad.uv_min.y],
            [quad.uv_max.x, quad.uv_min.y],
        ]);
        indices.extend_from_slice(&[base, base + 2, base + 1, base + 1, base + 2, base + 3]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn build_text_material(label: &UTextLabel, texture: Handle<Image>) -> UTextLabelSdfMaterial {
    UTextLabelSdfMaterial {
        color: label.color.into(),
        clip_radius: Vec4::ZERO,
        clip_center: Vec2::ZERO,
        clip_size: Vec2::ZERO,
        edge_softness: DEFAULT_TEXT_EDGE_SOFTNESS,
        use_clip: 0,
        _pad: Vec2::ZERO,
        texture,
    }
}

fn text_world_scale_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
) -> f32 {
    let mut current = entity;

    loop {
        if let Ok(root) = root_query.get(current) {
            return root.ui_units_to_world_scale();
        }

        let Ok(parent) = parents_query.get(current) else {
            return 1.0;
        };
        current = parent.parent();
    }
}

fn text_root_stack_for_entity(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    root_stack_query: &Query<&ResolvedRootStack>,
) -> ResolvedRootStack {
    let mut current = entity;

    loop {
        if let Ok(stack) = root_stack_query.get(current) {
            return *stack;
        }

        let Ok(parent) = parents_query.get(current) else {
            return ResolvedRootStack::default();
        };
        current = parent.parent();
    }
}

fn text_render_transform(world_scale: f32, depth_offset: f32) -> Transform {
    Transform {
        translation: Vec3::new(0.0, 0.0, depth_offset),
        scale: Vec3::splat(world_scale),
        ..default()
    }
}

pub(super) fn sync_text_label_meshes(
    mut commands: Commands,
    mut atlas_cache: ResMut<UTextLabelAtlasCache>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<UTextLabelSdfMaterial>>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut swash_cache: ResMut<bevy::text::SwashCache>,
    label_query: Query<
        (
            Entity,
            Ref<UTextLabel>,
            Ref<UNode>,
            Ref<ComputedSize>,
            Ref<ComputedTextBlock>,
            Ref<UTextLabelLayoutCache>,
            Option<&Children>,
        ),
        Or<(
            Added<UTextLabel>,
            Changed<UTextLabel>,
            Changed<UNode>,
            Changed<ComputedSize>,
            Changed<ComputedTextBlock>,
            Changed<UTextLabelLayoutCache>,
        )>,
    >,
    child_query: Query<
        (
            Entity,
            &ChildOf,
            &TextRenderPageMarker,
            &Mesh2d,
            &MeshMaterial2d<UTextLabelSdfMaterial>,
        ),
        With<TextChildMarker>,
    >,
) {
    for (entity, label, node, computed_size, computed, layout_cache, children) in label_query.iter()
    {
        let mut existing_children = HashMap::new();
        if let Some(children) = children {
            for child in children {
                if let Ok((child_entity, child_of, page_marker, mesh_handle, material_handle)) =
                    child_query.get(*child)
                {
                    if child_of.get() == entity {
                        existing_children.insert(
                            page_marker.texture_id,
                            (
                                child_entity,
                                mesh_handle.0.clone(),
                                material_handle.0.clone(),
                            ),
                        );
                    }
                }
            }
        }

        let text_size = layout_cache.measured_size;
        let render_scale = resolved_render_scale(label.render_scale);
        let content_clip = label_content_clip_rect(&label, &node, &computed_size);
        let horizontal_offset = text_horizontal_offset(&label, &node, &computed_size, text_size);
        let mut page_batches: HashMap<AssetId<Image>, TextPageBatch> = HashMap::default();

        if !layout_cache.displayed_text.is_empty() && text_size.x > 0.0 && text_size.y > 0.0 {
            for run in computed.buffer().layout_runs() {
                let line_y = (run.line_y * render_scale).round();

                for layout_glyph in run.glyphs.iter() {
                    let physical_glyph = layout_glyph.physical((0.0, 0.0), render_scale);
                    let cache_key = physical_glyph.cache_key;
                    let atlas_key = UTextLabelAtlasKey {
                        font_id: label.font.id(),
                        font_size_bits: cache_key.font_size_bits,
                        font_weight: cache_key.font_weight.0,
                        flags_bits: cache_key.flags.bits(),
                    };
                    let glyph_key = UTextLabelGlyphKey {
                        glyph_id: cache_key.glyph_id,
                        x_bin_bits: cache_key.x_bin.as_float().to_bits(),
                        y_bin_bits: cache_key.y_bin.as_float().to_bits(),
                    };

                    let atlas_pages = atlas_cache.pages_by_font.entry(atlas_key).or_default();
                    let atlas_info = if let Some(info) = atlas_pages
                        .iter()
                        .find_map(|page| page.glyphs.get(&glyph_key))
                        .cloned()
                    {
                        info
                    } else {
                        let Some(image) = swash_cache
                            .0
                            .get_image_uncached(&mut font_system.0, physical_glyph.cache_key)
                        else {
                            continue;
                        };

                        let width = image.placement.width;
                        let height = image.placement.height;
                        if width == 0 || height == 0 {
                            continue;
                        }

                        let mask = extract_mask_alpha(&image.data, width as usize, height as usize);
                        if mask.iter().all(|alpha| *alpha == 0) {
                            continue;
                        }

                        let glyph_image = build_sdf_glyph_image(&mask, width, height);
                        let glyph_offset = IVec2::new(
                            image.placement.left - TEXT_SDF_PADDING as i32,
                            image.placement.top + TEXT_SDF_PADDING as i32,
                        );

                        let mut stored = None;
                        for page in atlas_pages.iter_mut() {
                            let Some(atlas_layout) = texture_atlases.get_mut(&page.texture_atlas)
                            else {
                                continue;
                            };
                            let Some(atlas_image) = images.get_mut(&page.texture) else {
                                continue;
                            };
                            if let Ok(glyph_index) =
                                page.builder
                                    .add_texture(atlas_layout, &glyph_image, atlas_image)
                            {
                                let info = UTextLabelGlyphAtlasInfo {
                                    texture: page.texture.clone(),
                                    texture_atlas: page.texture_atlas.clone(),
                                    glyph_index,
                                    offset: glyph_offset,
                                };
                                page.glyphs.insert(glyph_key, info.clone());
                                stored = Some(info);
                                break;
                            }
                        }

                        if let Some(info) = stored {
                            info
                        } else {
                            let min_size = glyph_image
                                .texture_descriptor
                                .size
                                .width
                                .max(glyph_image.texture_descriptor.size.height);
                            let mut page =
                                create_text_atlas_page(&mut images, &mut texture_atlases, min_size);
                            let Some(atlas_layout) = texture_atlases.get_mut(&page.texture_atlas)
                            else {
                                continue;
                            };
                            let Some(atlas_image) = images.get_mut(&page.texture) else {
                                continue;
                            };
                            let Ok(glyph_index) =
                                page.builder
                                    .add_texture(atlas_layout, &glyph_image, atlas_image)
                            else {
                                continue;
                            };
                            let info = UTextLabelGlyphAtlasInfo {
                                texture: page.texture.clone(),
                                texture_atlas: page.texture_atlas.clone(),
                                glyph_index,
                                offset: glyph_offset,
                            };
                            page.glyphs.insert(glyph_key, info.clone());
                            atlas_pages.push(page);
                            info
                        }
                    };

                    let Some(atlas_layout) = texture_atlases.get(&atlas_info.texture_atlas) else {
                        continue;
                    };
                    let Some(atlas_image) = images.get(&atlas_info.texture) else {
                        continue;
                    };
                    let glyph_rect = atlas_layout.textures[atlas_info.glyph_index];
                    let glyph_size_pixels =
                        Vec2::new(glyph_rect.width() as f32, glyph_rect.height() as f32);
                    let glyph_size = glyph_size_pixels / render_scale;

                    let position_x = (glyph_size_pixels.x * 0.5
                        + atlas_info.offset.x as f32
                        + physical_glyph.x as f32)
                        / render_scale;
                    let position_y = (line_y + physical_glyph.y as f32
                        - atlas_info.offset.y as f32
                        + glyph_size_pixels.y * 0.5)
                        / render_scale;

                    let local_center = Vec2::new(
                        horizontal_offset + position_x - text_size.x * 0.5,
                        text_size.y * 0.5 - position_y,
                    );
                    let atlas_width = atlas_image.texture_descriptor.size.width as f32;
                    let atlas_height = atlas_image.texture_descriptor.size.height as f32;
                    let uv_min = Vec2::new(
                        glyph_rect.min.x as f32 / atlas_width,
                        glyph_rect.min.y as f32 / atlas_height,
                    );
                    let uv_max = Vec2::new(
                        glyph_rect.max.x as f32 / atlas_width,
                        glyph_rect.max.y as f32 / atlas_height,
                    );

                    let quad = TextGlyphQuad {
                        center: local_center,
                        size: glyph_size,
                        uv_min,
                        uv_max,
                    };
                    let clipped_quad = if let Some(clip_rect) = content_clip {
                        clip_quad_to_rect(quad, clip_rect)
                    } else {
                        Some(quad)
                    };
                    let Some(quad) = clipped_quad else {
                        continue;
                    };

                    page_batches
                        .entry(atlas_info.texture.id())
                        .and_modify(|batch| batch.quads.push(quad.clone()))
                        .or_insert_with(|| TextPageBatch {
                            texture: atlas_info.texture.clone(),
                            quads: vec![quad],
                        });
                }
            }
        }

        for (texture_id, batch) in page_batches {
            let existing = existing_children.remove(&texture_id);
            let mesh = build_text_page_mesh(&batch.quads);

            match existing {
                Some((child_entity, mesh_handle, material_handle)) => {
                    if let Some(existing_mesh) = meshes.get_mut(&mesh_handle) {
                        *existing_mesh = mesh;
                    }
                    if let Some(existing_material) = materials.get_mut(&material_handle) {
                        *existing_material = build_text_material(&label, batch.texture.clone());
                    }
                    commands
                        .entity(child_entity)
                        .insert(TextRenderPageMarker { texture_id });
                }
                None => {
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            TextRenderPageMarker { texture_id },
                            TextChildMarker,
                            Mesh2d(meshes.add(mesh)),
                            MeshMaterial2d(
                                materials.add(build_text_material(&label, batch.texture.clone())),
                            ),
                            Transform::default(),
                            Visibility::Inherited,
                        ));
                    });
                }
            }
        }

        for (_, (stale_child, _, _)) in existing_children {
            commands.entity(stale_child).despawn();
        }
    }
}

fn active_clipper_for_entity(
    start_entity: Entity,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: &Query<&ResolvedRootUi>,
) -> MaterialClipInfo {
    let mut current = start_entity;
    let world_scale = text_world_scale_for_entity(start_entity, parents_query, root_query);

    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();

        let Ok((transform, computed_size, node, clip)) = clipper_query.get(current) else {
            continue;
        };

        if !clip.enabled {
            continue;
        }

        let size = Vec2::new(computed_size.width, computed_size.height) * world_scale;
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }

        return MaterialClipInfo {
            center: transform.translation().truncate(),
            size,
            radius: Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            ) * world_scale,
            use_clip: 1,
        };
    }

    MaterialClipInfo::default()
}

pub(super) fn sync_text_clipper_materials(
    mut text_query: Query<
        (
            Entity,
            &MeshMaterial2d<UTextLabelSdfMaterial>,
            &mut Transform,
        ),
        With<TextChildMarker>,
    >,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: Query<&ResolvedRootUi>,
    root_stack_query: Query<&ResolvedRootStack>,
    mut materials: ResMut<Assets<UTextLabelSdfMaterial>>,
) {
    for (entity, material_handle, mut transform) in text_query.iter_mut() {
        let world_scale = text_world_scale_for_entity(entity, &parents_query, &root_query);
        let root_stack = text_root_stack_for_entity(entity, &parents_query, &root_stack_query);
        let clip = active_clipper_for_entity(entity, &parents_query, &clipper_query, &root_query);
        let Some(material) = materials.get_mut(&material_handle.0) else {
            continue;
        };
        let desired_transform = text_render_transform(world_scale, root_stack.text_child_offset());
        if *transform != desired_transform {
            *transform = desired_transform;
        }
        if material.clip_center != clip.center
            || material.clip_size != clip.size
            || material.clip_radius != clip.radius
            || material.use_clip != clip.use_clip
            || material.edge_softness != DEFAULT_TEXT_EDGE_SOFTNESS
        {
            material.clip_center = clip.center;
            material.clip_size = clip.size;
            material.clip_radius = clip.radius;
            material.use_clip = clip.use_clip;
            material.edge_softness = DEFAULT_TEXT_EDGE_SOFTNESS;
        }
    }
}
