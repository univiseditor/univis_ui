use bevy::prelude::*;
use crate::prelude::*; 

/// System to update the `LayoutDepth` component for all UI nodes.
///
/// This runs periodically (or on change) to ensure every node knows its depth level.
/// It also calculates the `LayoutTreeDepth` resource (maximum depth).
pub fn update_layout_hierarchy(
    root_query: Query<Entity, Or<(With<UWorldRoot>, With<UScreenRoot>)>>,
    children_query: Query<&Children>,
    mut commands: Commands,
    mut tree_depth: ResMut<LayoutTreeDepth>,
) {
    let mut max_depth = 0;

    for root_entity in root_query.iter() {
        max_depth = max_depth.max(traverse_and_mark(
            root_entity,
            0,
            &children_query,
            &mut commands,
        ));
    }

    tree_depth.max_depth = max_depth;
}

/// Recursive helper function to traverse the tree and mark depth.
fn traverse_and_mark(
    entity: Entity,
    depth: usize,
    children_q: &Query<&Children>,
    commands: &mut Commands,
) -> usize {
    // Insert or update depth component
    commands.entity(entity).insert(LayoutDepth(depth));

    let mut current_max = depth;

    if let Ok(children) = children_q.get(entity) {
        for &child in children {
            // Traverse children
            let child_depth = traverse_and_mark(child, depth + 1, children_q, commands);
            current_max = current_max.max(child_depth);
        }
    }
    
    current_max
}