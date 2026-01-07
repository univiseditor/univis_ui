use bevy::{platform::collections::*, prelude::*};
use crate::prelude::*;

/// نظام تخزين مؤقت للتخطيط - يقلل الحسابات المتكررة
#[derive(Resource, Default)]
pub struct LayoutCache {
    /// تخزين الأحجام الجوهرية المحسوبة
    intrinsic_sizes: HashMap<Entity, IntrinsicSize>,
    
    /// العقد التي تغيرت وتحتاج إعادة حساب
    dirty_nodes: HashSet<Entity>,
    
    /// العقد حسب العمق (لتجنب Filter في كل إطار)
    entities_by_depth: HashMap<usize, Vec<Entity>>,
    
    /// رقم الإطار الحالي (للتتبع)
    frame_count: u64,
    
    /// آخر عمق أقصى معروف
    last_max_depth: usize,
}

impl LayoutCache {
    /// إنشاء Cache جديد
    pub fn new() -> Self {
        Self::default()
    }

    /// تحديث قائمة العقد حسب العمق
    pub fn rebuild_depth_map(
        &mut self,
        query: &Query<(Entity, &LayoutDepth)>,
        max_depth: usize,
    ) {
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

    /// الحصول على العقد في عمق معين
    pub fn get_entities_at_depth(&self, depth: usize) -> Option<&Vec<Entity>> {
        self.entities_by_depth.get(&depth)
    }

    /// تعليم عقدة كـ "متسخة" (تحتاج إعادة حساب)
    pub fn mark_dirty(&mut self, entity: Entity) {
        // استخدام HashSet يمنع التكرار تلقائياً
        self.dirty_nodes.insert(entity);
    }

    /// تعليم عقدة وكل أبنائها كمتسخة
    pub fn mark_dirty_recursive(
        &mut self,
        entity: Entity,
        children_query: &Query<&Children>,
    ) {
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

    /// هل العقدة متسخة؟
    pub fn is_dirty(&self, entity: Entity) -> bool {
        self.dirty_nodes.contains(&entity)
    }

    /// مسح علامة "متسخ" من عقدة
    pub fn clear_dirty(&mut self, entity: Entity) {
        self.dirty_nodes.remove(&entity);
    }

    /// مسح كل العلامات المتسخة
    pub fn clear_all_dirty(&mut self) {
        self.dirty_nodes.clear();
    }

    /// حفظ الحجم الجوهري للعقدة
    pub fn cache_intrinsic(&mut self, entity: Entity, size: IntrinsicSize) {
        self.intrinsic_sizes.insert(entity, size);
    }

    /// استرجاع الحجم الجوهري المخزن
    pub fn get_cached_intrinsic(&self, entity: Entity) -> Option<IntrinsicSize> {
        self.intrinsic_sizes.get(&entity).copied()
    }

    /// زيادة عداد الإطارات
    pub fn increment_frame(&mut self) {
        self.frame_count += 1;
    }

    /// الحصول على رقم الإطار الحالي
    pub fn current_frame(&self) -> u64 {
        self.frame_count
    }
    
    /// عدد العقد المتسخة
    pub fn dirty_count(&self) -> usize {
        self.dirty_nodes.len()
    }
    
    /// نسبة العقد المتسخة
    pub fn dirty_ratio(&self, total_nodes: usize) -> f32 {
        if total_nodes == 0 {
            return 0.0;
        }
        (self.dirty_nodes.len() as f32 / total_nodes as f32) * 100.0
    }
}

/// نظام يراقب التغييرات ويحدث الـ Cache
pub fn track_layout_changes(
    mut cache: ResMut<LayoutCache>,
    
    // مراقبة التغييرات في المكونات المؤثرة
    changed_nodes: Query<
        Entity,
        Or<(
            Changed<UNode>,
            Changed<ULayout>,
            Changed<USelf>,
            Changed<Children>,
        )>
    >,
    
    // مراقبة العقد الجديدة
    added_nodes: Query<Entity, Added<UNode>>,
    
    children_query: Query<&Children>,
) {
    // تعليم العقد المتغيرة
    for entity in changed_nodes.iter() {
        cache.mark_dirty_recursive(entity, &children_query);
    }
    
    // تعليم العقد الجديدة
    for entity in added_nodes.iter() {
        cache.mark_dirty(entity);
    }
}

/// نظام يحدث خريطة العمق عند الحاجة
pub fn update_depth_cache(
    mut cache: ResMut<LayoutCache>,
    tree_depth: Res<LayoutTreeDepth>,
    depth_query: Query<(Entity, &LayoutDepth)>,
) {
    // تحديث فقط إذا تغير العمق الأقصى أو كان Cache فارغاً
    if tree_depth.max_depth != cache.last_max_depth 
        || cache.entities_by_depth.is_empty() 
    {
        cache.rebuild_depth_map(&depth_query, tree_depth.max_depth);
    }
}

/// Plugin للـ Cache System
pub struct LayoutCachePlugin;

impl Plugin for LayoutCachePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LayoutCache>()
            .add_systems(
                Update,
                (
                    track_layout_changes,
                    update_depth_cache,
                )
                .chain()
                .before(upward_measure_pass_cached)
            );
    }
}

// =========================================================
// استخدام الـ Cache في الأنظمة الموجودة
// =========================================================
