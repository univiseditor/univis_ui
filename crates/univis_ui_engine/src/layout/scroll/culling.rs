//! Automated AABB viewport culling for Univis UI scroll containers.
//!
//! Evaluates world-space bounding boxes of children inside [`UScrollContainer`]
//! and assigns [`Visibility::Hidden`] to nodes completely outside the visible viewport.

use bevy::prelude::*;

use crate::layout::query::ComputedSize;
use crate::layout::scroll::container::UScrollContainer;
use crate::layout::univis_node::UNode;

/// Configures AABB viewport culling behavior on a [`UScrollContainer`].
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UScrollCulling {
    /// Whether viewport culling is active (default: true).
    pub enabled: bool,
    /// Extra margin in physical pixels around the viewport before an item is culled.
    pub margin: f32,
}

impl Default for UScrollCulling {
    fn default() -> Self {
        Self {
            enabled: true,
            margin: 64.0,
        }
    }
}

impl UScrollCulling {
    /// Creates a scroll culling configuration with custom safety margin.
    pub fn with_margin(margin: f32) -> Self {
        Self {
            enabled: true,
            margin,
        }
    }
}

/// Automatically ensures [`UScrollCulling`] is attached to new [`UScrollContainer`] entities.
pub fn init_scroll_culling(
    mut commands: Commands,
    query: Query<Entity, (With<UScrollContainer>, Without<UScrollCulling>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(UScrollCulling::default());
    }
}

/// Evaluates scroll container child node bounds and applies [`Visibility::Hidden`] to offscreen nodes.
pub fn cull_scrolled_viewport_children(
    containers: Query<(
        Entity,
        &GlobalTransform,
        &ComputedSize,
        &UScrollCulling,
    )>,
    parents_query: Query<&ChildOf>,
    mut nodes_query: Query<(
        Entity,
        &GlobalTransform,
        &ComputedSize,
        &mut Visibility,
    ), With<UNode>>,
) {
    for (container_entity, container_gt, container_size, culling) in containers.iter() {
        if !culling.enabled || container_size.width <= 0.0 || container_size.height <= 0.0 {
            continue;
        }

        let container_pos = container_gt.translation().xy();
        let half_w = (container_size.width / 2.0) + culling.margin;
        let half_h = (container_size.height / 2.0) + culling.margin;

        let min_bound = container_pos - Vec2::new(half_w, half_h);
        let max_bound = container_pos + Vec2::new(half_w, half_h);

        // Cull descendants that belong to this container
        for (node_entity, node_gt, node_size, mut visibility) in nodes_query.iter_mut() {
            if node_entity == container_entity {
                continue;
            }

            // Check if node is an ancestor-descendant of this container
            let mut is_descendant = false;
            let mut current = node_entity;
            while let Ok(parent) = parents_query.get(current) {
                if parent.parent() == container_entity {
                    is_descendant = true;
                    break;
                }
                current = parent.parent();
            }

            if !is_descendant {
                continue;
            }

            let node_pos = node_gt.translation().xy();
            let node_half_w = node_size.width / 2.0;
            let node_half_h = node_size.height / 2.0;

            let node_min = node_pos - Vec2::new(node_half_w, node_half_h);
            let node_max = node_pos + Vec2::new(node_half_w, node_half_h);

            // AABB intersection check
            let overlaps = node_max.x >= min_bound.x
                && node_min.x <= max_bound.x
                && node_max.y >= min_bound.y
                && node_min.y <= max_bound.y;

            if overlaps {
                if *visibility == Visibility::Hidden {
                    *visibility = Visibility::Inherited;
                }
            } else if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_culling_intersection() {
        let mut app = App::new();
        app.add_systems(Update, cull_scrolled_viewport_children);

        let container = app
            .world_mut()
            .spawn((
                UScrollContainer::vertical(),
                UScrollCulling {
                    enabled: true,
                    margin: 0.0,
                },
                ComputedSize {
                    width: 200.0,
                    height: 200.0,
                    ..default()
                },
                Transform::default(),
                GlobalTransform::from_translation(Vec3::ZERO),
            ))
            .id();

        // Node inside viewport: centered at (0, 0)
        let inside_child = app
            .world_mut()
            .spawn((
                ChildOf(container),
                UNode::default(),
                ComputedSize {
                    width: 50.0,
                    height: 50.0,
                    ..default()
                },
                Transform::default(),
                GlobalTransform::from_translation(Vec3::ZERO),
                Visibility::Inherited,
            ))
            .id();

        // Node far outside viewport: centered at (0, 500)
        let outside_child = app
            .world_mut()
            .spawn((
                ChildOf(container),
                UNode::default(),
                ComputedSize {
                    width: 50.0,
                    height: 50.0,
                    ..default()
                },
                Transform::default(),
                GlobalTransform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
                Visibility::Inherited,
            ))
            .id();

        app.update();

        let vis_inside = app.world().get::<Visibility>(inside_child).unwrap();
        let vis_outside = app.world().get::<Visibility>(outside_child).unwrap();

        assert_eq!(*vis_inside, Visibility::Inherited);
        assert_eq!(*vis_outside, Visibility::Hidden);
    }
}
