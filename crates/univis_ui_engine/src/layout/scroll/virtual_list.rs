//! High-performance virtualized list and entity recycling for Univis UI.
//!
//! Provides [`UVirtualList`] and [`UVirtualItem`] to render lists with thousands
//! or millions of items using a small, fixed pool of ECS entities ($O(1)$ memory and GPU cost).

use bevy::prelude::*;

use crate::layout::query::ComputedSize;
use crate::layout::scroll::container::UScrollContainer;

/// Scroll axis direction for [`UVirtualList`].
#[derive(Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
pub enum UVirtualDirection {
    /// Vertical scrolling (default: items stacked top-to-bottom).
    #[default]
    Vertical,
    /// Horizontal scrolling (items arranged left-to-right).
    Horizontal,
}

/// Configures an entity as a virtualized list container.
///
/// When attached to a container with [`UScrollContainer`], this component computes
/// virtual scroll extents and recycles a fixed pool of child entities as the user scrolls.
#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
#[reflect(Component)]
pub struct UVirtualList {
    /// Total number of virtual items in the dataset (e.g. 10,000).
    pub item_count: usize,
    /// Fixed extent in pixels for each item (height for Vertical, width for Horizontal).
    pub item_extent: f32,
    /// Number of extra buffer items kept alive above and below the viewport to prevent pop-in.
    pub buffer_items: usize,
    /// Layout direction (Vertical or Horizontal).
    pub direction: UVirtualDirection,
}

impl Default for UVirtualList {
    fn default() -> Self {
        Self {
            item_count: 0,
            item_extent: 50.0,
            buffer_items: 2,
            direction: UVirtualDirection::Vertical,
        }
    }
}

impl UVirtualList {
    /// Creates a new vertical virtual list with specified item count and extent.
    pub fn vertical(item_count: usize, item_extent: f32) -> Self {
        Self {
            item_count,
            item_extent,
            buffer_items: 2,
            direction: UVirtualDirection::Vertical,
        }
    }

    /// Creates a new horizontal virtual list with specified item count and extent.
    pub fn horizontal(item_count: usize, item_extent: f32) -> Self {
        Self {
            item_count,
            item_extent,
            buffer_items: 2,
            direction: UVirtualDirection::Horizontal,
        }
    }

    /// Sets custom buffer item count.
    pub fn with_buffer(mut self, buffer_items: usize) -> Self {
        self.buffer_items = buffer_items;
        self
    }
}

/// Tracks the virtual index and pool slot for a recycled child item entity.
///
/// User systems can query `Changed<UVirtualItem>` to rebind text, icons, and visual
/// state whenever an item is recycled to represent a new dataset index.
#[derive(Component, Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct UVirtualItem {
    /// The current logical data index represented by this entity (0..item_count).
    pub virtual_index: usize,
    /// The pool slot index assigned to this entity (0..pool_size).
    pub slot_index: usize,
}

/// Maintains the pool of recycled child entities for a [`UVirtualList`].
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct UVirtualPool {
    /// Entity handles currently participating in the recycling pool.
    pub entities: Vec<Entity>,
}

/// Automatically calculates virtual container maximum scroll extents.
pub fn sync_virtual_list_extents(
    mut query: Query<(&UVirtualList, &ComputedSize, &mut UScrollContainer)>,
) {
    for (vlist, size, mut container) in query.iter_mut() {
        if vlist.item_extent <= 0.0 {
            continue;
        }

        let total_extent = (vlist.item_count as f32) * vlist.item_extent;

        match vlist.direction {
            UVirtualDirection::Vertical => {
                let overflow = (total_extent - size.height).max(0.0);
                container.max_offset.y = overflow;
            }
            UVirtualDirection::Horizontal => {
                let overflow = (total_extent - size.width).max(0.0);
                container.max_offset.x = overflow;
            }
        }

        container.target_offset = container
            .target_offset
            .clamp(Vec2::ZERO, container.max_offset);
        container.scroll_offset = container
            .scroll_offset
            .clamp(Vec2::ZERO, container.max_offset);
    }
}

/// Recycles pooled items and updates their transforms and virtual data indices.
pub fn recycle_virtual_items(
    containers: Query<(
        Entity,
        &UVirtualList,
        &UScrollContainer,
        &ComputedSize,
        &UVirtualPool,
    )>,
    mut items_query: Query<(
        &mut UVirtualItem,
        &mut Transform,
        &mut Visibility,
        Option<&ChildOf>,
    )>,
) {
    for (_container_entity, vlist, container, size, pool) in containers.iter() {
        if vlist.item_extent <= 0.0 || pool.entities.is_empty() {
            continue;
        }

        let (current_offset, _viewport_size) = match vlist.direction {
            UVirtualDirection::Vertical => (container.scroll_offset.y, size.height),
            UVirtualDirection::Horizontal => (container.scroll_offset.x, size.width),
        };

        // Determine starting visible index
        let first_visible_idx = ((current_offset / vlist.item_extent).floor() as isize).max(0) as usize;
        let start_idx = first_visible_idx.saturating_sub(vlist.buffer_items);

        for (slot, &entity) in pool.entities.iter().enumerate() {
            let Ok((mut item, mut transform, mut visibility, _child_of)) =
                items_query.get_mut(entity)
            else {
                continue;
            };

            let target_virtual_idx = start_idx + slot;

            if target_virtual_idx < vlist.item_count {
                // Item is active inside dataset
                if item.virtual_index != target_virtual_idx || item.slot_index != slot {
                    item.virtual_index = target_virtual_idx;
                    item.slot_index = slot;
                }

                *visibility = Visibility::Inherited;

                // Position item along layout axis
                let physical_offset = (target_virtual_idx as f32) * vlist.item_extent;
                match vlist.direction {
                    UVirtualDirection::Vertical => {
                        // In screen coordinates with origin at top-left, y goes downwards
                        transform.translation.y = -physical_offset;
                    }
                    UVirtualDirection::Horizontal => {
                        transform.translation.x = physical_offset;
                    }
                }
            } else {
                // Out of range: hide completely from rendering and extraction
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_list_constructors() {
        let v = UVirtualList::vertical(1000, 48.0).with_buffer(3);
        assert_eq!(v.item_count, 1000);
        assert_eq!(v.item_extent, 48.0);
        assert_eq!(v.buffer_items, 3);
        assert_eq!(v.direction, UVirtualDirection::Vertical);

        let h = UVirtualList::horizontal(500, 120.0);
        assert_eq!(h.item_count, 500);
        assert_eq!(h.item_extent, 120.0);
        assert_eq!(h.direction, UVirtualDirection::Horizontal);
    }

    #[test]
    fn test_sync_virtual_list_extents() {
        let mut app = App::new();
        app.add_systems(Update, sync_virtual_list_extents);

        let entity = app
            .world_mut()
            .spawn((
                UVirtualList::vertical(100, 50.0), // Total height: 5000.0
                ComputedSize {
                    width: 400.0,
                    height: 800.0,
                    ..default()
                },
                UScrollContainer::vertical(),
            ))
            .id();

        app.update();

        let container = app.world().get::<UScrollContainer>(entity).unwrap();
        // 5000.0 - 800.0 = 4200.0
        assert_eq!(container.max_offset.y, 4200.0);
    }

    #[test]
    fn test_recycle_virtual_items_windowing() {
        let mut app = App::new();
        app.add_systems(Update, recycle_virtual_items);

        let mut pool_entities = Vec::new();
        for slot in 0..5 {
            let child = app
                .world_mut()
                .spawn((
                    UVirtualItem {
                        virtual_index: slot,
                        slot_index: slot,
                    },
                    Transform::default(),
                    Visibility::Inherited,
                ))
                .id();
            pool_entities.push(child);
        }

        let mut container = UScrollContainer::vertical();
        // Scroll down to 250px (item_extent = 50.0 -> index 5)
        container.scroll_offset.y = 250.0;

        let _list_entity = app
            .world_mut()
            .spawn((
                UVirtualList {
                    item_count: 100,
                    item_extent: 50.0,
                    buffer_items: 0,
                    direction: UVirtualDirection::Vertical,
                },
                container,
                ComputedSize {
                    width: 300.0,
                    height: 200.0,
                    ..default()
                },
                UVirtualPool {
                    entities: pool_entities.clone(),
                },
            ))
            .id();

        app.update();

        // Slot 0 should now represent virtual index 5 (250 / 50)
        let item0 = app.world().get::<UVirtualItem>(pool_entities[0]).unwrap();
        let trans0 = app.world().get::<Transform>(pool_entities[0]).unwrap();
        let vis0 = app.world().get::<Visibility>(pool_entities[0]).unwrap();

        assert_eq!(item0.virtual_index, 5);
        assert_eq!(trans0.translation.y, -250.0);
        assert_eq!(*vis0, Visibility::Inherited);

        // Slot 4 should represent virtual index 9
        let item4 = app.world().get::<UVirtualItem>(pool_entities[4]).unwrap();
        assert_eq!(item4.virtual_index, 9);
    }

    #[test]
    fn test_recycle_hides_out_of_bounds_items() {
        let mut app = App::new();
        app.add_systems(Update, recycle_virtual_items);

        let mut pool_entities = Vec::new();
        for _slot in 0..5 {
            let child = app
                .world_mut()
                .spawn((
                    UVirtualItem::default(),
                    Transform::default(),
                    Visibility::Inherited,
                ))
                .id();
            pool_entities.push(child);
        }

        // Only 2 total items in the dataset
        let _list_entity = app
            .world_mut()
            .spawn((
                UVirtualList {
                    item_count: 2,
                    item_extent: 50.0,
                    buffer_items: 0,
                    direction: UVirtualDirection::Vertical,
                },
                UScrollContainer::vertical(),
                ComputedSize {
                    width: 300.0,
                    height: 200.0,
                    ..default()
                },
                UVirtualPool {
                    entities: pool_entities.clone(),
                },
            ))
            .id();

        app.update();

        // Slots 0 and 1 are within bounds -> Inherited
        let vis0 = app.world().get::<Visibility>(pool_entities[0]).unwrap();
        let vis1 = app.world().get::<Visibility>(pool_entities[1]).unwrap();
        assert_eq!(*vis0, Visibility::Inherited);
        assert_eq!(*vis1, Visibility::Inherited);

        // Slots 2, 3, 4 are out of bounds -> Hidden
        let vis2 = app.world().get::<Visibility>(pool_entities[2]).unwrap();
        let vis3 = app.world().get::<Visibility>(pool_entities[3]).unwrap();
        let vis4 = app.world().get::<Visibility>(pool_entities[4]).unwrap();
        assert_eq!(*vis2, Visibility::Hidden);
        assert_eq!(*vis3, Visibility::Hidden);
        assert_eq!(*vis4, Visibility::Hidden);
    }
}
