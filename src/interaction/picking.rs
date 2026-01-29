use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::picking::backend::prelude::*;
use crate::prelude::*;
use super::math::sd_rounded_box;

/// دالة دقيقة للتحقق من القص باستخدام المصفوفات
fn is_clipped_by_ancestors(
    start_entity: Entity,
    cursor_world_pos: Vec2, // موقع الماوس في العالم
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
                // هذا يحل مشاكل الإزاحة والدوران والتحجيم (Zoom)
                let transform_matrix = transform.to_matrix();
                let inverse_matrix = transform_matrix.inverse();
                
                // تحويل النقطة
                let cursor_in_clipper_space = inverse_matrix.transform_point3(cursor_world_pos.extend(0.0)).truncate();

                // 2. حساب حدود القناع
                let half_size = Vec2::new(size.width, size.height) * 0.5;
                let radius = Vec4::new(
                    node.border_radius.top_right,
                    node.border_radius.bottom_right,
                    node.border_radius.top_left,
                    node.border_radius.bottom_left,
                );

                // 3. اختبار SDF
                // إذا كانت المسافة موجبة، فهذا يعني أن الماوس "خارج" حدود القناع
                let dist = sd_rounded_box(cursor_in_clipper_space, half_size, radius);

                if dist > 0.0 {
                    return true; // نعم، العنصر مقصوص في هذه النقطة
                }
            }
        }
    }

    false // غير مقصوص
}

pub fn univis_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera2d>>,
    
    // الاستعلام عن العناصر التفاعلية
    nodes_query: Query<(Entity, &UNode, &GlobalTransform, &ComputedSize, &Pickable), With<UInteraction>>,
    
    // استعلامات الهرمية (للقص)
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,

    mut output: MessageWriter<PointerHits>,
) {
    let Ok((cam_entity, camera, cam_transform)) = cameras.single() else { return };

    for (pointer_id, pointer_loc) in pointers.iter() {
        let Some(location) = pointer_loc.location() else { continue };
        
        // 1. تحويل الماوس من الشاشة للعالم
        let Ok(ray) = camera.viewport_to_world(cam_transform, location.position) else { continue };
        let cursor_pos_world = ray.origin.truncate(); 

        let mut hits = Vec::new();

        for (entity, node, global_transform, size, pickable) in nodes_query.iter() {
            // هل هذا العنصر قابل للنقر؟
            if !pickable.should_block_lower { continue; } 

            // 2. التحويل من العالم (World) إلى المحلي (Local) للعنصر نفسه
            // هذه الخطوة ضرورية لكي تعمل الزوايا الدائرية بشكل صحيح
            let transform_matrix = global_transform.to_matrix();
            let inverse_matrix = transform_matrix.inverse();
            
            let cursor_pos_local = inverse_matrix.transform_point3(cursor_pos_world.extend(0.0)).truncate(); 

            // 3. حساب حدود العنصر
            let half_size = Vec2::new(size.width, size.height) * 0.5;
            let radius_vec = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );

            

            // 4. اختبار SDF (هل الماوس داخل الشكل؟)
            let dist = sd_rounded_box(cursor_pos_local, half_size, radius_vec);
            
            // الشرط: المسافة <= 0 تعني أننا داخل الشكل
            if dist <= 0.0 {
                // 5. التحقق النهائي: هل أنا مخفي بسبب قص الأب؟
                if is_clipped_by_ancestors(entity, cursor_pos_world, &parents_query, &clipper_query) {
                    continue; // العنصر موجود هندسياً، لكنه مقصوص بصرياً -> تجاهل
                }

                // تم الالتقاط بنجاح!
                hits.push((entity, HitData {
                    camera: cam_entity, 
                    depth: global_transform.translation().z, 
                    position: Some(cursor_pos_world.extend(0.0)),
                    normal: None,
                }));
            }
        }

        // ترتيب النتائج حسب العمق (الأقرب للكاميرا أولاً)
        hits.sort_by(|a, b| b.1.depth.partial_cmp(&a.1.depth).unwrap_or(std::cmp::Ordering::Equal));

        if !hits.is_empty() {
            output.write(PointerHits {
                pointer: *pointer_id,
                picks: hits,
                order: camera.order as f32, 
            });
        }
    }
}