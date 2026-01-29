use bevy::prelude::*;
use bevy::picking::backend::prelude::*;
use crate::prelude::*;
use super::math::sd_rounded_box;

/// Custom picking backend for Univis.
/// Uses Signed Distance Fields (SDF) to accurately detect hits on rounded shapes.
pub fn univis_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    // 1. We need the camera to convert Screen -> World
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera2d>>,
    nodes_query: Query<(Entity, &UNode, &GlobalTransform, &ComputedSize, &Pickable), With<UInteraction>>,
    mut output: MessageWriter<PointerHits>,
) {
    // Assume one UI camera
    let Ok((cam_entity, camera, cam_transform)) = cameras.single() else { return };

    for (pointer_id, pointer_loc) in pointers.iter() {
        let Some(location) = pointer_loc.location() else { continue };
        
        // 2. Transform: Screen Point -> World Point
        let Ok(ray) = camera.viewport_to_world(cam_transform, location.position) else { continue };
        let cursor_pos_world = ray.origin.truncate(); // Vec3 -> Vec2

        let mut hits = Vec::new();

        for (entity, node, global_transform, size, pickable) in nodes_query.iter() {
            // Check pickable
            
            if !pickable.should_block_lower { continue; } 
                        
            // 3. Transform: World Space -> Local Space
            let transform_matrix = global_transform.to_matrix();
            let inverse_matrix = transform_matrix.inverse();
            
            let cursor_pos_local_3d = inverse_matrix.transform_point3(cursor_pos_world.extend(0.0));
            let cursor_pos_local = cursor_pos_local_3d.truncate(); 

            // 4. SDF Test (Is mouse inside rounded box?)
            let half_size = Vec2::new(size.width, size.height) * 0.5;
            let radius_vec = Vec4::new(
                node.border_radius.top_right,
                node.border_radius.bottom_right,
                node.border_radius.top_left,
                node.border_radius.bottom_left,
            );

            // Distance calculation
            let dist = sd_rounded_box(cursor_pos_local, half_size, radius_vec);

            // If distance <= 0, we are inside
            if dist <= 0.0 {
                hits.push((entity, HitData {
                    camera: cam_entity, 
                    depth: global_transform.translation().z, 
                    position: Some(cursor_pos_world.extend(0.0)),
                    normal: None,
                }));
            }
        }

        // Sort by depth (Z-index)
        hits.sort_by(|a, b| b.1.depth.partial_cmp(&a.1.depth).unwrap_or(std::cmp::Ordering::Equal));

        if !hits.is_empty() {
            output.write(PointerHits {
                pointer: *pointer_id,
                picks: hits,
                order: 0.0, 
            });
        }
    }
}