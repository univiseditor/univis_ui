use bevy::ecs::relationship::Relationship;
use bevy::prelude::{ChildOf, Entity, GlobalTransform, Query, Vec2, Vec3, Vec4};

use crate::interaction::math::sd_rounded_box;
use univis_ui_engine::layout::query::{ComputedSize, UiPickingContext};
use univis_ui_engine::layout::univis_node::{UClip, UNode};

pub(super) fn is_clipped_by_ancestors(
    start_entity: Entity,
    cursor_world_pos: Vec3,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> bool {
    let mut current_entity = start_entity;

    while let Ok(parent) = parents_query.get(current_entity) {
        current_entity = parent.get();

        if let Ok((transform, size, node, clip)) = clipper_query.get(current_entity)
            && clip.enabled
        {
            let transform_matrix = transform.to_matrix();
            let inverse_matrix = transform_matrix.inverse();
            let cursor_in_clipper_space =
                inverse_matrix.transform_point3(cursor_world_pos).truncate();

            let half_size = Vec2::new(size.width, size.height) * 0.5;
            let radius = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );

            if sd_rounded_box(cursor_in_clipper_space, half_size, radius) > 0.0 {
                return true;
            }
        }
    }

    false
}

pub(super) fn is_clipped_by_picking_context(
    picking_context: Option<&UiPickingContext>,
    start_entity: Entity,
    cursor_world_pos: Vec3,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> bool {
    if let Some(clip_entity) = picking_context.and_then(|value| value.clip_ancestor)
        && let Ok((transform, size, node, clip)) = clipper_query.get(clip_entity)
        && clip.enabled
    {
        let cursor_in_clipper_space = transform
            .to_matrix()
            .inverse()
            .transform_point3(cursor_world_pos)
            .truncate();
        let half_size = Vec2::new(size.width, size.height) * 0.5;
        let radius = Vec4::new(
            node.border_radius.top_right,
            node.border_radius.bottom_right,
            node.border_radius.top_left,
            node.border_radius.bottom_left,
        );
        return sd_rounded_box(cursor_in_clipper_space, half_size, radius) > 0.0;
    }

    is_clipped_by_ancestors(start_entity, cursor_world_pos, parents_query, clipper_query)
}
