use bevy::ecs::relationship::Relationship;
use bevy::picking::backend::prelude::PointerLocation;
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

use super::{UPanelWindow, model::PanelResizeEdge};
use univis_ui_engine::layout::geometry::UVal;
use univis_ui_engine::layout::query::{ComputedSize, ResolvedRootUi};
use univis_ui_engine::layout::univis_node::{UNode, UPositionType, USelf};

#[derive(Clone, Copy, Debug)]
pub(super) struct PanelRect {
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) left: f32,
    pub(super) top: f32,
}

pub(super) fn ensure_panel_absolute_geometry(
    panel_entity: Entity,
    panel_window: &UPanelWindow,
    node: &mut UNode,
    uself_opt: &mut Option<Mut<USelf>>,
    computed: &ComputedSize,
    transform: &Transform,
    parent: Option<&ChildOf>,
    parent_size_query: &Query<&ComputedSize>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
) -> bool {
    let needs_absolute = !matches!(node.width, UVal::Px(_))
        || !matches!(node.height, UVal::Px(_))
        || uself_opt.as_ref().map_or(true, |uself| {
            uself.position_type != UPositionType::Absolute
                || !matches!(uself.left, UVal::Px(_))
                || !matches!(uself.top, UVal::Px(_))
        });

    if !needs_absolute {
        return true;
    }

    let width = computed.width.max(panel_window.min_width.max(1.0));
    let height = computed.height.max(panel_window.min_height.max(1.0));
    node.width = UVal::Px(width);
    node.height = UVal::Px(height);

    let parent_size = resolve_parent_size(
        panel_entity,
        parent,
        parent_size_query,
        parents_query,
        root_query,
        windows,
    )
    .unwrap_or(Vec2::new(width, height));

    let left = transform.translation.x + (parent_size.x * 0.5) - (computed.width.max(1.0) * 0.5);
    let top = (parent_size.y * 0.5) - transform.translation.y - (computed.height.max(1.0) * 0.5);

    if let Some(uself) = uself_opt.as_mut() {
        uself.position_type = UPositionType::Absolute;
        uself.left = UVal::Px(left);
        uself.top = UVal::Px(top);
        true
    } else {
        commands.entity(panel_entity).insert(USelf {
            position_type: UPositionType::Absolute,
            left: UVal::Px(left),
            top: UVal::Px(top),
            ..default()
        });
        false
    }
}

pub(super) fn resolve_parent_size(
    panel_entity: Entity,
    parent: Option<&ChildOf>,
    parent_size_query: &Query<&ComputedSize>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
    windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    if let Some(parent) = parent {
        if let Ok(size) = parent_size_query.get(parent.get()) {
            return Some(Vec2::new(size.width.max(1.0), size.height.max(1.0)));
        }
    }

    if let Some(root) = resolve_root_for_entity(panel_entity, parents_query, root_query) {
        return Some(root.canvas_size.max(Vec2::splat(1.0)));
    }

    if let Ok(window) = windows.single() {
        return Some(Vec2::new(window.width(), window.height()));
    }

    None
}

pub(super) fn cursor_in_parent_space(
    panel_entity: Entity,
    parent: Option<&ChildOf>,
    parents_query: &Query<&ChildOf>,
    root_query: &Query<&ResolvedRootUi>,
    pointers: &Query<(&PointerId, &PointerLocation)>,
    cameras: &Query<(Entity, &Camera, &GlobalTransform), With<Camera>>,
    global_query: &Query<&GlobalTransform>,
) -> Option<Vec2> {
    let location = pointers.iter().find_map(|(pointer_id, pointer_loc)| {
        pointer_id
            .is_mouse()
            .then(|| pointer_loc.location())
            .flatten()
    })?;
    let root = resolve_root_for_entity(panel_entity, parents_query, root_query)?;
    let camera_entity = root.camera_entity?;
    let Ok((_, camera, camera_transform)) = cameras.get(camera_entity) else {
        return None;
    };
    if !camera
        .logical_viewport_rect()
        .is_some_and(|rect| rect.contains(location.position))
    {
        return None;
    }

    let ray = camera
        .viewport_to_world(camera_transform, location.position)
        .ok()?;
    let plane_entity = parent.map(|value| value.get()).unwrap_or(root.root_entity);
    let plane_transform = global_query.get(plane_entity).ok()?;
    let cursor_world =
        intersect_ray_with_ui_plane(ray.origin, ray.direction.as_vec3(), plane_transform)?;
    let local_matrix = plane_transform.to_matrix().inverse();
    Some(local_matrix.transform_point3(cursor_world).truncate())
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

pub(super) fn intersect_ray_with_ui_plane(
    ray_origin: Vec3,
    ray_direction: Vec3,
    plane_transform: &GlobalTransform,
) -> Option<Vec3> {
    let plane_origin = plane_transform.translation();
    let plane_normal = plane_transform.back().as_vec3();
    let denominator = ray_direction.dot(plane_normal);

    if denominator.abs() <= f32::EPSILON {
        return None;
    }

    let distance = (plane_origin - ray_origin).dot(plane_normal) / denominator;
    if distance < 0.0 {
        return None;
    }

    Some(ray_origin + ray_direction * distance)
}

pub(super) fn apply_resize_delta_to_rect(
    edge: PanelResizeEdge,
    delta_parent: Vec2,
    min_width: f32,
    min_height: f32,
    rect: &mut PanelRect,
) {
    let min_width = min_width.max(1.0);
    let min_height = min_height.max(1.0);

    if edge.has_east() {
        rect.width = (rect.width + delta_parent.x).max(min_width);
    }

    if edge.has_west() {
        let prev = rect.width;
        rect.width = (rect.width - delta_parent.x).max(min_width);
        rect.left += prev - rect.width;
    }

    let delta_down = -delta_parent.y;
    if edge.has_south() {
        rect.height = (rect.height + delta_down).max(min_height);
    }

    if edge.has_north() {
        let prev = rect.height;
        rect.height = (rect.height - delta_down).max(min_height);
        rect.top += prev - rect.height;
    }
}

pub(super) fn uval_px(value: UVal) -> Option<f32> {
    match value {
        UVal::Px(v) => Some(v),
        _ => None,
    }
}
