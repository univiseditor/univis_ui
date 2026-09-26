//! Synchronizes hardware SDF rounded clipping uniforms between viewport containers
//! and their descendant `UNodeMaterial` assets.

use bevy::prelude::*;

use crate::layout::geometry::ComputedSize;
use crate::layout::layout_system::ResolvedRootUi;
use crate::layout::render::material::UNodeMaterial;
use crate::layout::univis_node::{UClip, UNode};

/// Computed clip bounding information for a node.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodeMaterialClipInfo {
    /// World-space center of the clip box.
    pub center: Vec2,
    /// Physical size of the clip box in world units.
    pub size: Vec2,
    /// Corner radii of the clip box in world units.
    pub radius: Vec4,
    /// 1 if clipping is active, 0 otherwise.
    pub use_clip: u32,
}

fn node_world_scale_for_entity(
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

/// Discovers the nearest active ancestor clipper and extracts its world-space geometry.
pub fn active_clipper_for_node(
    start_entity: Entity,
    parents_query: &Query<&ChildOf>,
    clipper_query: &Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: &Query<&ResolvedRootUi>,
) -> NodeMaterialClipInfo {
    let mut current = start_entity;
    let world_scale = node_world_scale_for_entity(start_entity, parents_query, root_query);

    while let Ok(parent) = parents_query.get(current) {
        current = parent.parent();

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

        return NodeMaterialClipInfo {
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

    NodeMaterialClipInfo::default()
}

/// Synchronizes hardware SDF clipping uniforms for all 2D node materials with active viewport containers.
pub fn sync_node_clipper_materials(
    nodes_query: Query<(Entity, &MeshMaterial2d<UNodeMaterial>)>,
    parents_query: Query<&ChildOf>,
    clipper_query: Query<(&GlobalTransform, &ComputedSize, &UNode, &UClip)>,
    root_query: Query<&ResolvedRootUi>,
    mut materials_2d: ResMut<Assets<UNodeMaterial>>,
) {
    for (entity, material_handle) in nodes_query.iter() {
        let Some(mut material) = materials_2d.get_mut(&material_handle.0) else {
            continue;
        };

        let clip = active_clipper_for_node(entity, &parents_query, &clipper_query, &root_query);

        if material.clip_center != clip.center
            || material.clip_size != clip.size
            || material.clip_radius != clip.radius
            || material.use_clip != clip.use_clip
        {
            material.clip_center = clip.center;
            material.clip_size = clip.size;
            material.clip_radius = clip.radius;
            material.use_clip = clip.use_clip;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::geometry::UCornerRadius;

    #[test]
    fn test_active_clipper_for_node() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(bevy::asset::AssetPlugin::default())
            .init_asset::<UNodeMaterial>();

        let root = app
            .world_mut()
            .spawn((
                UNode::default(),
                ComputedSize {
                    width: 800.0,
                    height: 600.0,
                    ..default()
                },
                Transform::default(),
                GlobalTransform::from_translation(Vec3::ZERO),
            ))
            .id();

        let clipper = app
            .world_mut()
            .spawn((
                ChildOf(root),
                UNode {
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                ComputedSize {
                    width: 300.0,
                    height: 200.0,
                    ..default()
                },
                UClip { enabled: true },
                Transform::default(),
                GlobalTransform::from_translation(Vec3::new(100.0, 50.0, 0.0)),
            ))
            .id();

        let child = app
            .world_mut()
            .spawn((
                ChildOf(clipper),
                UNode::default(),
                ComputedSize {
                    width: 100.0,
                    height: 50.0,
                    ..default()
                },
                Transform::default(),
                GlobalTransform::from_translation(Vec3::new(120.0, 60.0, 0.0)),
            ))
            .id();

        let mut materials = app.world_mut().resource_mut::<Assets<UNodeMaterial>>();
        let mat_handle = materials.add(UNodeMaterial::default());
        app.world_mut()
            .entity_mut(child)
            .insert(MeshMaterial2d(mat_handle.clone()));

        app.add_systems(Update, sync_node_clipper_materials);
        app.update();

        let materials = app.world().resource::<Assets<UNodeMaterial>>();
        let material = materials.get(&mat_handle).unwrap();
        assert_eq!(material.use_clip, 1);
        assert_eq!(material.clip_center, Vec2::new(100.0, 50.0));
        assert_eq!(material.clip_size, Vec2::new(300.0, 200.0));
        assert_eq!(material.clip_radius, Vec4::splat(8.0));
    }
}
