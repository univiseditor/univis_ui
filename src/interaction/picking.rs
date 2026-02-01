use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::picking::backend::prelude::*;
use crate::prelude::*;
use super::math::sd_rounded_box;

/// دالة دقيقة للتحقق من القص باستخدام المصفوفات
fn is_clipped_by_ancestors(
    start_entity: Entity,
    cursor_world_pos: Vec2,
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
                let cursor_in_clipper_space = inverse_matrix
                    .transform_point3(cursor_world_pos.extend(0.0))
                    .truncate();

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

pub fn univis_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera2d>>,
    
    nodes_query: Query<(
        Entity, 
        &UNode, 
        &GlobalTransform, 
        &ComputedSize, 
        Option<&LayoutDepth>,
    ), With<UInteraction>>,
    
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,

    mut output: MessageWriter<PointerHits>,
) {
    let Ok((cam_entity, camera, cam_transform)) = cameras.single() else { 
        return; 
    };

    for (pointer_id, pointer_loc) in pointers.iter() {
        let Some(location) = pointer_loc.location() else { 
            continue; 
        };
        
        let Ok(ray) = camera.viewport_to_world(cam_transform, location.position) else { 
            continue; 
        };
        
        let cursor_pos_world = ray.origin.truncate(); 

        // المرحلة 1: جمع كل الـ hits المحتملة
        let mut all_hits: Vec<(Entity, HitData, f32)> = Vec::new();

        for (entity, node, global_transform, size, depth_comp) in nodes_query.iter() {
            // التحويل لـ Local Space
            let transform_matrix = global_transform.to_matrix();
            let inverse_matrix = transform_matrix.inverse();
            let cursor_pos_local = inverse_matrix
                .transform_point3(cursor_pos_world.extend(0.0))
                .truncate(); 

            let half_size = Vec2::new(size.width, size.height) * 0.5;
            let radius_vec = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );

            let dist = sd_rounded_box(cursor_pos_local, half_size, radius_vec);

            if dist <= 0.0 {
                // التحقق من القص
                if is_clipped_by_ancestors(
                    entity, 
                    cursor_pos_world, 
                    &parents_query, 
                    &clipper_query
                ) {
                    continue; 
                }

                // حساب العمق
                let tree_depth = depth_comp.map(|d| d.0).unwrap_or(0) as f32;
                let z_depth = global_transform.translation().z;
                let final_depth = tree_depth * 1000.0 + z_depth;

                all_hits.push((
                    entity, 
                    HitData {
                        camera: cam_entity, 
                        depth: final_depth,
                        position: Some(cursor_pos_world.extend(0.0)),
                        normal: Some(Vec3::Z),
                    },
                    final_depth, // نحتفظ بالعمق للمقارنة
                ));
            }
        }

        // ✅ المرحلة 2: تصفية الآباء إذا كان هناك أبناء
        // نريد فقط إبقاء العنصر الأعمق من كل عائلة
        let mut filtered_hits: Vec<(Entity, HitData)> = Vec::new();

        for (entity, hit_data, depth) in all_hits.iter() {
            let mut should_include = true;

            // فحص: هل هناك ابن لهذا الكيان تم التقاطه أيضاً؟
            for (other_entity, _, other_depth) in all_hits.iter() {
                if entity == other_entity {
                    continue;
                }

                // إذا كان other_entity ابن لـ entity وعمقه أكبر (أقرب للكاميرا)
                if is_ancestor_of(*entity, *other_entity, &parents_query) && other_depth > depth {
                    should_include = false;
                    break;
                }
            }

            if should_include {
                filtered_hits.push((*entity, hit_data.clone()));
            }
        }

        // إرسال النتائج المفلترة
        if !filtered_hits.is_empty() {
            output.write(PointerHits {
                pointer: *pointer_id,
                picks: filtered_hits,
                order: 0.0, 
            });
        }
    }
}