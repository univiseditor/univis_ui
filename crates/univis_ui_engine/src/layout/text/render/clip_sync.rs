use super::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalClipRect {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

#[derive(Clone, Copy, Debug, Default)]
struct MaterialClipInfo {
    center: Vec2,
    size: Vec2,
    radius: Vec4,
    use_clip: u32,
}

pub(crate) fn clip_quad_to_rect(
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

pub(crate) fn label_content_clip_rect(
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

pub(crate) fn sync_text_clipper_materials(
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
        let Some(mut material) = materials.get_mut(&material_handle.0) else {
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
