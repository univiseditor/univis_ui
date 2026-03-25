use super::math::sd_rounded_box;
use crate::internal_prelude::*;
use bevy::ecs::relationship::Relationship;
use bevy::picking::backend::prelude::*;
use bevy::picking::pointer::Location;
use bevy::prelude::*;
use std::collections::HashMap;

/// دالة دقيقة للتحقق من القص باستخدام المصفوفات
fn is_clipped_by_ancestors(
    start_entity: Entity,
    cursor_world_pos: Vec3,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
) -> bool {
    let mut current_entity = start_entity;

    // نصعد في شجرة الآباء
    while let Ok(parent) = parents_query.get(current_entity) {
        current_entity = parent.get();

        if let Ok((transform, size, node, clip)) = clipper_query.get(current_entity) {
            if clip.enabled {
                // 1. التحويل من العالم (World) إلى المحلي (Local) الخاص بالأب القاطع
                let transform_matrix = transform.to_matrix();
                let inverse_matrix = transform_matrix.inverse();

                // تحويل النقطة
                let cursor_in_clipper_space =
                    inverse_matrix.transform_point3(cursor_world_pos).truncate();

                // 2. حساب حدود القناع
                let half_size = Vec2::new(size.width, size.height) * 0.5;
                let radius = Vec4::new(
                    node.border_radius.top_right,
                    node.border_radius.bottom_right,
                    node.border_radius.top_left,
                    node.border_radius.bottom_left,
                );

                // 3. اختبار SDF
                let dist = sd_rounded_box(cursor_in_clipper_space, half_size, radius);

                if dist > 0.0 {
                    return true; // نعم، العنصر مقصوص في هذه النقطة
                }
            }
        }
    }

    false // غير مقصوص
}

#[derive(Clone, Copy)]
struct CachedPointerRay {
    order: f32,
    origin: Vec3,
    direction: Vec3,
}

#[derive(Default)]
struct CameraHitBucket {
    order: f32,
    hits: Vec<(Entity, HitData, f32)>,
}

/// ✅ دالة جديدة: فحص إذا كان الكيان هو أب لكيان آخر
fn is_ancestor_of(
    potential_ancestor: Entity,
    potential_descendant: Entity,
    parents_query: &Query<&ChildOf>,
) -> bool {
    let mut current = potential_descendant;

    // نصعد في الشجرة حتى نجد الأب أو نصل للجذر
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if current == potential_ancestor {
            return true;
        }
    }

    false
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

fn pointer_ray_for_camera(
    location: &Location,
    camera_entity: Entity,
    cameras: &Query<(Entity, &Camera, &GlobalTransform)>,
) -> Option<CachedPointerRay> {
    let Ok((_, camera, camera_transform)) = cameras.get(camera_entity) else {
        return None;
    };

    if !camera
        .logical_viewport_rect()
        .is_some_and(|rect| rect.contains(location.position))
    {
        return None;
    }

    let ray = camera.viewport_to_world(camera_transform, location.position).ok()?;
    Some(CachedPointerRay {
        order: camera.order as f32,
        origin: ray.origin,
        direction: ray.direction.as_vec3(),
    })
}

fn intersect_ray_with_node_plane(
    ray: &CachedPointerRay,
    global_transform: &GlobalTransform,
) -> Option<(Vec3, Vec2, Vec3, f32)> {
    let plane_origin = global_transform.translation();
    let plane_normal = global_transform.back().as_vec3();
    let denominator = ray.direction.dot(plane_normal);

    if denominator.abs() <= f32::EPSILON {
        return None;
    }

    let distance = (plane_origin - ray.origin).dot(plane_normal) / denominator;
    if distance < 0.0 {
        return None;
    }

    let world_hit = ray.origin + ray.direction * distance;
    let local_hit = global_transform
        .to_matrix()
        .inverse()
        .transform_point3(world_hit)
        .truncate();

    Some((world_hit, local_hit, plane_normal, distance))
}

pub fn univis_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform)>,
    root_query: Query<&ResolvedRootUi>,
    nodes_query: Query<
        (
            Entity,
            &UNode,
            &GlobalTransform,
            &ComputedSize,
            Option<&LayoutDepth>,
        ),
        With<UInteraction>,
    >,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    mut output: MessageWriter<PointerHits>,
) {
    for (pointer_id, pointer_loc) in pointers.iter() {
        let Some(location) = pointer_loc.location() else {
            continue;
        };

        let mut ray_cache: HashMap<Entity, Option<CachedPointerRay>> = HashMap::new();
        let mut hits_by_camera: HashMap<Entity, CameraHitBucket> = HashMap::new();

        for (entity, node, global_transform, size, depth_comp) in nodes_query.iter() {
            let Some(root) = resolve_root_for_entity(entity, &parents_query, &root_query) else {
                continue;
            };
            let Some(camera_entity) = root.camera_entity else {
                continue;
            };

            let ray = ray_cache
                .entry(camera_entity)
                .or_insert_with(|| pointer_ray_for_camera(location, camera_entity, &cameras))
                .as_ref();
            let Some(ray) = ray else {
                continue;
            };

            let Some((hit_world, cursor_pos_local, hit_normal, hit_distance)) =
                intersect_ray_with_node_plane(ray, global_transform)
            else {
                continue;
            };

            let half_size = Vec2::new(size.width, size.height) * 0.5;
            let radius_vec = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );

            let dist = sd_rounded_box(cursor_pos_local, half_size, radius_vec);

            if dist <= 0.0 {
                if is_clipped_by_ancestors(entity, hit_world, &parents_query, &clipper_query) {
                    continue;
                }

                // نستخدم مسافة الشعاع كأساس للعمق، مع انحياز صغير يمنح الأبناء أولوية
                // على الآباء عند وجودهما على نفس المستوى البصري تقريبًا.
                let tree_bias = depth_comp.map(|d| d.0).unwrap_or(0) as f32 * 1e-4;
                let final_depth = (hit_distance - tree_bias).max(0.0);

                hits_by_camera
                    .entry(camera_entity)
                    .or_insert_with(|| CameraHitBucket {
                        order: ray.order,
                        ..default()
                    })
                    .hits
                    .push((
                        entity,
                        HitData::new(
                            camera_entity,
                            final_depth,
                            Some(hit_world),
                            Some(hit_normal),
                        ),
                        final_depth,
                    ));
            }
        }

        for bucket in hits_by_camera.into_values() {
            let mut filtered_hits: Vec<(Entity, HitData)> = Vec::new();

            for (entity, hit_data, depth) in bucket.hits.iter() {
                let mut should_include = true;

                // نريد فقط إبقاء أعمق عنصر في نفس العائلة.
                for (other_entity, _, other_depth) in bucket.hits.iter() {
                    if entity == other_entity {
                        continue;
                    }

                    if is_ancestor_of(*entity, *other_entity, &parents_query)
                        && other_depth < depth
                    {
                        should_include = false;
                        break;
                    }
                }

                if should_include {
                    filtered_hits.push((*entity, hit_data.clone()));
                }
            }

            if !filtered_hits.is_empty() {
                output.write(PointerHits::new(*pointer_id, filtered_hits, bucket.order));
            }
        }
    }
}
