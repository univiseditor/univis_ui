pub(super) mod atlas;
pub(super) mod clip_sync;
pub(super) mod mesh;
mod sdf;

use bevy::asset::{Asset, AssetId, Assets, RenderAssetUsages};
use bevy::color::LinearRgba;
use bevy::ecs::relationship::Relationship;
use bevy::image::{DynamicTextureAtlasBuilder, ImageSampler, TextureAtlasLayout};
use bevy::mesh::{Indices, Mesh, Mesh2d, Mesh3d, PrimitiveTopology};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, Extent3d, TextureDimension, TextureFormat};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d};
use bevy::text::{ComputedTextBlock, FontCx, ScaleCx};
use std::collections::HashMap;
use univis_ui_engine::layout::query::{CachedUiContext, UI3d};
use univis_ui_engine::layout::query::{ComputedSize, ResolvedRootStack, ResolvedRootUi};
use univis_ui_engine::layout::univis_node::{UClip, UNode};

use self::atlas::{
    UTextLabelAtlasCache, UTextLabelAtlasKey, UTextLabelGlyphAtlasInfo, UTextLabelGlyphKey,
    create_text_atlas_page, extract_mask_alpha,
};
use self::clip_sync::{clip_quad_to_rect, label_content_clip_rect};
use self::mesh::{
    TextGlyphQuad, TextPageBatch, build_text_material, build_text_material_3d,
    build_text_page_mesh, resolved_render_scale, text_horizontal_offset,
};
use self::sdf::build_sdf_glyph_image;
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct UTextLabelSdfMaterial3d {
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

impl Material for UTextLabelSdfMaterial3d {
    fn fragment_shader() -> ShaderRef {
        "embedded://univis_ui_widgets/widget/shaders/text_label_sdf_3d.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

pub(super) fn sync_text_label_meshes(
    mut commands: Commands,
    mut atlas_cache: ResMut<UTextLabelAtlasCache>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<UTextLabelSdfMaterial>>,
    mut materials_3d: ResMut<Assets<UTextLabelSdfMaterial3d>>,
    mut _font_cx: ResMut<FontCx>,
    mut scale_cx: ResMut<ScaleCx>,
    label_query: Query<
        (
            Entity,
            Ref<UTextLabel>,
            Ref<UNode>,
            Ref<ComputedSize>,
            Ref<ComputedTextBlock>,
            Ref<UTextLabelLayoutCache>,
            Option<&Children>,
            Has<UI3d>,
            Option<&CachedUiContext>,
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
            Option<&Mesh2d>,
            Option<&Mesh3d>,
            Option<&MeshMaterial2d<UTextLabelSdfMaterial>>,
            Option<&MeshMaterial3d<UTextLabelSdfMaterial3d>>,
        ),
        With<TextChildMarker>,
    >,
) {
    for (
        entity,
        label,
        node,
        computed_size,
        computed,
        layout_cache,
        children,
        is_3d,
        cached_context,
    ) in label_query.iter()
    {
        let mut existing_children = HashMap::new();
        if let Some(children) = children {
            for child in children {
                if let Ok((child_entity, child_of, page_marker, mesh_2d, mesh_3d, mat_2d, mat_3d)) =
                    child_query.get(*child)
                {
                    if child_of.get() == entity {
                        let child_is_3d = mesh_3d.is_some() || mat_3d.is_some();
                        if child_is_3d == is_3d {
                            let mesh_handle = if child_is_3d {
                                mesh_3d.map(|m| m.0.clone())
                            } else {
                                mesh_2d.map(|m| m.0.clone())
                            };
                            let mat_handle_2d = mat_2d.map(|m| m.0.clone());
                            let mat_handle_3d = mat_3d.map(|m| m.0.clone());

                            if let Some(m) = mesh_handle {
                                existing_children.insert(
                                    page_marker.texture_id,
                                    (child_entity, m, mat_handle_2d, mat_handle_3d),
                                );
                            }
                        }
                    }
                }
            }
        }

        let text_size = layout_cache.measured_size;
        let render_scale = resolved_render_scale(label.render_scale);
        let content_clip = label_content_clip_rect(&label, &node, &computed_size);
        let horizontal_offset = text_horizontal_offset(&label, &node, &computed_size, text_size);

        let ui_to_world_scale = if is_3d {
            cached_context.map_or(1.0, |c| c.ui_to_world_scale)
        } else {
            1.0
        };

        let mut page_batches: HashMap<AssetId<Image>, TextPageBatch> = HashMap::default();

        if !layout_cache.displayed_text.is_empty() && text_size.x > 0.0 && text_size.y > 0.0 {
            use bevy::platform::hash::FixedHasher;
            use std::hash::BuildHasher;
            use parley::PositionedLayoutItem;

            for (_line_index, line) in computed.buffer().lines().enumerate() {
                for item in line.items() {
                    if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                        let run = glyph_run.run();
                        let font = run.font();
                        let font_size = run.font_size();
                        let coords = run.normalized_coords();
                        let variations_hash = FixedHasher.hash_one(coords);

                        let font_size_physical = font_size * render_scale;

                        let Some(font_ref) =
                            swash::FontRef::from_index(font.data.as_ref(), font.index as usize)
                        else {
                            continue;
                        };

                        let mut scaler = scale_cx
                            .0
                            .builder(font_ref)
                            .size(font_size_physical)
                            .hint(false)
                            .normalized_coords(coords)
                            .build();

                        let atlas_key = UTextLabelAtlasKey {
                            font_data_id: font.data.id(),
                            font_size_bits: font_size_physical.to_bits(),
                            index: font.index,
                            variations_hash,
                        };

                        for glyph in glyph_run.positioned_glyphs() {
                            let Ok(glyph_id) = u16::try_from(glyph.id) else {
                                continue;
                            };

                            let glyph_key = UTextLabelGlyphKey {
                                glyph_id,
                                x_bin_bits: (glyph.x * render_scale).to_bits(),
                                y_bin_bits: (glyph.y * render_scale).to_bits(),
                            };

                            let atlas_pages = atlas_cache.pages_by_font.entry(atlas_key).or_default();
                            let atlas_info = if let Some(info) = atlas_pages
                                .iter()
                                .find_map(|page| page.glyphs.get(&glyph_key))
                                .cloned()
                            {
                                info
                            } else {
                                let image = swash::scale::Render::new(&[
                                    swash::scale::Source::ColorOutline(0),
                                    swash::scale::Source::ColorBitmap(swash::scale::StrikeWith::BestFit),
                                    swash::scale::Source::Outline,
                                ])
                                .format(swash::zeno::Format::Alpha)
                                .render(&mut scaler, glyph_id);

                                let Some(image) = image else {
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
                                    let Some(mut atlas_layout) = texture_atlases.get_mut(&page.texture_atlas)
                                    else {
                                        continue;
                                    };
                                    let Some(mut atlas_image) = images.get_mut(&page.texture) else {
                                        continue;
                                    };
                                    if let Ok(glyph_index) =
                                        page.builder
                                            .add_texture(&mut *atlas_layout, &glyph_image, &mut *atlas_image)
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
                                    let Some(mut atlas_layout) = texture_atlases.get_mut(&page.texture_atlas)
                                    else {
                                        continue;
                                    };
                                    let Some(mut atlas_image) = images.get_mut(&page.texture) else {
                                        continue;
                                    };
                                    let Ok(glyph_index) =
                                        page.builder
                                            .add_texture(&mut *atlas_layout, &glyph_image, &mut *atlas_image)
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
                                + glyph.x * render_scale)
                                / render_scale;
                            let position_y = (glyph.y * render_scale
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
                            if let Some(mut q) = clipped_quad {
                                q.center *= ui_to_world_scale;
                                q.size *= ui_to_world_scale;

                                page_batches
                                    .entry(atlas_info.texture.id())
                                    .and_modify(|batch| batch.quads.push(q.clone()))
                                    .or_insert_with(|| TextPageBatch {
                                        texture: atlas_info.texture.clone(),
                                        quads: vec![q],
                                    });
                            }
                        }
                    }
                }
            }
        }

        for (texture_id, batch) in page_batches {
            let existing = existing_children.remove(&texture_id);
            let mesh = build_text_page_mesh(&batch.quads);

            match existing {
                Some((child_entity, mesh_handle, mat_handle_2d, mat_handle_3d)) => {
                    if let Some(mut existing_mesh) = meshes.get_mut(&mesh_handle) {
                        *existing_mesh = mesh;
                    }
                    if is_3d {
                        if let Some(h) = mat_handle_3d {
                            if let Some(mut existing_material) = materials_3d.get_mut(&h) {
                                *existing_material =
                                    build_text_material_3d(&label, batch.texture.clone());
                            }
                        }
                    } else {
                        if let Some(h) = mat_handle_2d {
                            if let Some(mut existing_material) = materials.get_mut(&h) {
                                *existing_material =
                                    build_text_material(&label, batch.texture.clone());
                            }
                        }
                    }
                    commands
                        .entity(child_entity)
                        .insert(TextRenderPageMarker { texture_id });
                }
                None => {
                    commands.entity(entity).with_children(|parent| {
                        let mut child_cmd = parent.spawn((
                            TextRenderPageMarker { texture_id },
                            TextChildMarker,
                            Transform::default(),
                            Visibility::Inherited,
                        ));
                        if is_3d {
                            child_cmd.insert((
                                Mesh3d(meshes.add(mesh)),
                                MeshMaterial3d(
                                    materials_3d
                                        .add(build_text_material_3d(&label, batch.texture.clone())),
                                ),
                            ));
                        } else {
                            child_cmd.insert((
                                Mesh2d(meshes.add(mesh)),
                                MeshMaterial2d(
                                    materials
                                        .add(build_text_material(&label, batch.texture.clone())),
                                ),
                            ));
                        }
                    });
                }
            }
        }

        for (_, (stale_child, _, _, _)) in existing_children {
            commands.entity(stale_child).despawn();
        }
    }
}
