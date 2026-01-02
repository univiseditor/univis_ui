use bevy::prelude::*;
use crate::prelude::*;

/// System to create and update the visual representation (Mesh & Material) of nodes.
///
/// It handles:
/// 1. **Initialization:** Adds `Mesh2d` and `MeshMaterial2d` to new nodes.
/// 2. **Updates:** Recreates the mesh/material if the node properties (size, color, border) change.
pub fn update_shader_visuals(
    mut commands: Commands,
    
    // 1. Update Query: Nodes that have changed
    query: Query<
        (Entity, &UNode, Option<&UBorder>, &ComputedSize), 
        (
            Or<(Changed<UNode>, Changed<UBorder>, Changed<ComputedSize>)>, 
            With<MeshMaterial2d<UNodeMaterial>> 
        )
    >,

    // 2. Init Query: New nodes without visuals
    init_query: Query<
        (Entity, &UNode, &ComputedSize), 
        (
            Without<MeshMaterial2d<UNodeMaterial>>, 
            Without<Mesh2d> 
        )
    >,
    
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<UNodeMaterial>>,
) {
    // 1. Initialization
    for (entity, node, size) in init_query.iter() {
        let size_vec = Vec2::new(size.width, size.height);
        
        let mesh = meshes.add(Rectangle::new(size_vec.x, size_vec.y));
        let material = materials.add(create_material(node, None, size_vec));

        commands.entity(entity).insert((
            Mesh2d(mesh),                 
            MeshMaterial2d(material),     
        ));
    }

    // 2. Update logic
    for (entity, node, border, size) in query.iter() {
        let size_vec = Vec2::new(size.width, size.height);

        let mesh = meshes.add(Rectangle::new(size_vec.x, size_vec.y));
        let material = materials.add(create_material(node, border, size_vec));

        commands.entity(entity).insert((
            Mesh2d(mesh),
            MeshMaterial2d(material),
        ));
    }
}

// Helper to construct the custom material
fn create_material(node: &UNode, border: Option<&UBorder>, size: Vec2) -> UNodeMaterial {
    let (b_color, b_width) = if let Some(b) = border {
        (LinearRgba::from(b.color), b.width)
    } else {
        (LinearRgba::NONE, 0.0)
    };

    UNodeMaterial {
        color: LinearRgba::from(node.background_color),
        size: size,
        radius: Vec4::new(
            node.border_radius.top_right,
            node.border_radius.bottom_right,
            node.border_radius.top_left,
            node.border_radius.bottom_left,
        ),
        border_color: b_color,
        border_width: b_width,
        softness: 1.0, 
    }
}