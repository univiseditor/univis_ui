use crate::internal_prelude::*;
use bevy::{ecs::relationship::Relationship, platform::collections::*, prelude::*};

/// Cache resource used to avoid repeated layout work across frames.
#[derive(Resource, Default)]
pub struct LayoutCache {
    /// Cached intrinsic sizes per entity.
    intrinsic_sizes: HashMap<Entity, IntrinsicSize>,

    /// Nodes that need to be recomputed.
    dirty_nodes: HashSet<Entity>,

    /// Entities grouped by tree depth.
    entities_by_depth: HashMap<usize, Vec<Entity>>,

    /// Running frame counter used for diagnostics.
    frame_count: u64,

    /// Last known maximum depth.
    last_max_depth: usize,
}

impl LayoutCache {
    /// Creates an empty layout cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds the entity-to-depth index.
    pub fn rebuild_depth_map(&mut self, query: &Query<(Entity, &LayoutDepth)>, max_depth: usize) {
        // مسح الخريطة القديمة
        self.entities_by_depth.clear();

        // إعادة بناء
        for (entity, depth) in query.iter() {
            self.entities_by_depth
                .entry(depth.0)
                .or_insert_with(Vec::new)
                .push(entity);
        }

        self.last_max_depth = max_depth;
    }

    /// Returns all entities known for a depth bucket.
    pub fn get_entities_at_depth(&self, depth: usize) -> Option<&Vec<Entity>> {
        self.entities_by_depth.get(&depth)
    }

    /// Marks one entity as dirty.
    pub fn mark_dirty(&mut self, entity: Entity) {
        // استخدام HashSet يمنع التكرار تلقائياً
        self.dirty_nodes.insert(entity);
    }

    /// Marks an entity and all descendants as dirty.
    pub fn mark_dirty_recursive(&mut self, entity: Entity, children_query: &Query<&Children>) {
        // فقط إذا لم تكن متسخة مسبقاً (تجنب infinite recursion)
        if !self.dirty_nodes.insert(entity) {
            return; // العقدة كانت متسخة مسبقاً، توقف
        }

        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                self.mark_dirty_recursive(child, children_query);
            }
        }
    }

    /// Marks all ancestors from the entity up to the root as dirty.
    pub fn mark_dirty_ancestors(&mut self, entity: Entity, parents_query: &Query<&ChildOf>) {
        let mut current = entity;

        while let Ok(parent) = parents_query.get(current) {
            current = parent.get();
            self.dirty_nodes.insert(current);
        }
    }

    /// Returns `true` when the entity is marked dirty.
    pub fn is_dirty(&self, entity: Entity) -> bool {
        self.dirty_nodes.contains(&entity)
    }

    /// Clears the dirty flag for one entity.
    pub fn clear_dirty(&mut self, entity: Entity) {
        self.dirty_nodes.remove(&entity);
    }

    /// Clears all dirty flags.
    pub fn clear_all_dirty(&mut self) {
        self.dirty_nodes.clear();
    }

    /// Stores an intrinsic size entry.
    pub fn cache_intrinsic(&mut self, entity: Entity, size: IntrinsicSize) {
        self.intrinsic_sizes.insert(entity, size);
    }

    /// Returns the cached intrinsic size for an entity, if present.
    pub fn get_cached_intrinsic(&self, entity: Entity) -> Option<IntrinsicSize> {
        self.intrinsic_sizes.get(&entity).copied()
    }

    /// Increments the frame counter.
    pub fn increment_frame(&mut self) {
        self.frame_count += 1;
    }

    /// Returns the current frame counter.
    pub fn current_frame(&self) -> u64 {
        self.frame_count
    }

    /// Returns how many entities are currently dirty.
    pub fn dirty_count(&self) -> usize {
        self.dirty_nodes.len()
    }

    /// Returns the percentage of dirty entities relative to `total_nodes`.
    pub fn dirty_ratio(&self, total_nodes: usize) -> f32 {
        if total_nodes == 0 {
            return 0.0;
        }
        (self.dirty_nodes.len() as f32 / total_nodes as f32) * 100.0
    }
}

/// Internal system that tracks layout-affecting changes.
#[doc(hidden)]
pub fn track_layout_changes(
    mut cache: ResMut<LayoutCache>,

    // نستخدم Ref لنتمكن من فحص is_changed() لكل مكون على حدة
    nodes: Query<
        (
            Entity,
            Option<&Children>,
            Ref<UNode>,
            Option<Ref<ULayout>>,
            Option<Ref<USelf>>,
            Ref<IntrinsicSize>,
        ),
        // الفلتر العام: نمر فقط على العقد التي تغير فيها شيء ما
        Or<(
            Changed<UNode>,
            Changed<ULayout>,
            Changed<USelf>,
            Changed<Children>,
            Changed<IntrinsicSize>,
        )>,
    >,

    added_nodes: Query<Entity, Added<UNode>>,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
) {
    // 1. معالجة التغييرات
    for (entity, children, node, layout, uself, intrinsic) in nodes.iter() {
        let change_flags = LayoutChangeFlags {
            intrinsic_changed: intrinsic.is_changed(),
            node_changed: node.is_changed(),
            layout_changed: layout.map_or(false, |l| l.is_changed()),
            uself_changed: uself.map_or(false, |s| s.is_changed()),
        };

        // إذا كان التغيير الوحيد IntrinsicSize على عقدة حاوية، نتجاهله لتفادي
        // إعادة توسيخ الشجرة بشكل متكرر بسبب مخرجات القياس الداخلية.
        if should_skip_intrinsic_only_container_change(
            change_flags,
            children.is_some_and(|kids| !kids.is_empty()),
        ) {
            continue;
        }

        // في جميع الحالات الأخرى، نعتبر العنصر متسخاً
        cache.mark_dirty_recursive(entity, &children_query);
        cache.mark_dirty_ancestors(entity, &parents_query);
    }

    // 2. معالجة العناصر الجديدة
    for entity in added_nodes.iter() {
        cache.mark_dirty(entity);
        cache.mark_dirty_ancestors(entity, &parents_query);
    }
}

#[derive(Clone, Copy)]
struct LayoutChangeFlags {
    intrinsic_changed: bool,
    node_changed: bool,
    layout_changed: bool,
    uself_changed: bool,
}

fn should_skip_intrinsic_only_container_change(
    flags: LayoutChangeFlags,
    has_children: bool,
) -> bool {
    flags.intrinsic_changed
        && !flags.node_changed
        && !flags.layout_changed
        && !flags.uself_changed
        && has_children
}

/// Internal system that rebuilds the depth cache when the tree changes.
#[doc(hidden)]
pub fn update_depth_cache(
    mut cache: ResMut<LayoutCache>,
    tree_depth: Res<LayoutTreeDepth>,

    // الاستعلام الكامل لإعادة البناء
    depth_query: Query<(Entity, &LayoutDepth)>,

    // === [الإضافة الهامة] ===
    // مراقبة هل تم إضافة مكون LayoutDepth جديد؟
    // هذا يعني أن هناك عقدة جديدة دخلت النظام
    added_nodes: Query<Entity, Added<LayoutDepth>>,

    // مراقبة هل تم حذف عقد؟ (لتنظيف الكاش)
    mut removed_nodes: RemovedComponents<LayoutDepth>,
) {
    // هل تغير الهيكل؟ (إضافة أو حذف عقد)
    let structure_changed = !added_nodes.is_empty() || removed_nodes.read().count() > 0;

    // شروط إعادة البناء:
    // 1. تغير الهيكل (عقد جديدة/محذوفة)
    // 2. تغير العمق الأقصى
    // 3. الكاش فارغ (أول إطار)
    if structure_changed
        || tree_depth.max_depth != cache.last_max_depth
        || cache.entities_by_depth.is_empty()
    {
        cache.rebuild_depth_map(&depth_query, tree_depth.max_depth);
    }
}
/// Internal plugin that installs the layout cache systems.
#[doc(hidden)]
pub struct LayoutCachePlugin;

impl Plugin for LayoutCachePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LayoutCache>()
            .add_systems(Update, (track_layout_changes, update_depth_cache).chain());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intrinsic_only_flags() -> LayoutChangeFlags {
        LayoutChangeFlags {
            intrinsic_changed: true,
            node_changed: false,
            layout_changed: false,
            uself_changed: false,
        }
    }

    #[test]
    fn intrinsic_only_change_on_container_is_skipped() {
        assert!(should_skip_intrinsic_only_container_change(
            intrinsic_only_flags(),
            true,
        ));
    }

    #[test]
    fn intrinsic_only_change_on_leaf_is_not_skipped() {
        assert!(!should_skip_intrinsic_only_container_change(
            intrinsic_only_flags(),
            false,
        ));
    }

    #[test]
    fn non_intrinsic_change_is_not_skipped() {
        let mut flags = intrinsic_only_flags();
        flags.node_changed = true;

        assert!(!should_skip_intrinsic_only_container_change(flags, true));
    }

    #[test]
    fn layout_change_is_not_skipped() {
        let mut flags = intrinsic_only_flags();
        flags.layout_changed = true;

        assert!(!should_skip_intrinsic_only_container_change(flags, true));
    }

    #[test]
    fn uself_change_is_not_skipped() {
        let mut flags = intrinsic_only_flags();
        flags.uself_changed = true;

        assert!(!should_skip_intrinsic_only_container_change(flags, true));
    }
}

// =========================================================
// استخدام الـ Cache في الأنظمة الموجودة
// =========================================================
