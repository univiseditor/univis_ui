use super::*;

#[derive(Clone, Debug)]
pub(crate) struct TextGlyphQuad {
    pub(crate) center: Vec2,
    pub(crate) size: Vec2,
    pub(crate) uv_min: Vec2,
    pub(crate) uv_max: Vec2,
}

#[derive(Clone)]
pub(super) struct TextPageBatch {
    pub(super) texture: Handle<Image>,
    pub(super) quads: Vec<TextGlyphQuad>,
}

pub(super) fn resolved_render_scale(render_scale: f32) -> f32 {
    if render_scale.is_finite() && render_scale >= 1.0 {
        render_scale
    } else {
        1.0
    }
}

pub(crate) fn text_horizontal_offset(
    label: &UTextLabel,
    node: &UNode,
    computed_size: &ComputedSize,
    text_size: Vec2,
) -> f32 {
    let content_width = (computed_size.width - node.padding.width_sum()).max(0.0);

    match label.justify {
        Justify::Left | Justify::Start => (-content_width * 0.5) + (text_size.x * 0.5),
        Justify::Center | Justify::Justified => 0.0,
        Justify::Right | Justify::End => (content_width * 0.5) - (text_size.x * 0.5),
    }
}

pub(super) fn build_text_page_mesh(quads: &[TextGlyphQuad]) -> Mesh {
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

pub(super) fn build_text_material(
    label: &UTextLabel,
    texture: Handle<Image>,
) -> UTextLabelSdfMaterial {
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

pub(super) fn build_text_material_3d(
    label: &UTextLabel,
    texture: Handle<Image>,
) -> UTextLabelSdfMaterial3d {
    UTextLabelSdfMaterial3d {
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
