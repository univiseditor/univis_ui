use bevy::prelude::*;
use crate::prelude::*;
use std::time::Instant;

// =========================================================
// Wrapper Systems مع Profiling مدمج
// =========================================================

/// Upward Pass مع Profiling مدمج
pub fn upward_pass_with_profiling(
    mut profiler: ResMut<LayoutProfiler>,
    tree_depth: Res<LayoutTreeDepth>,
    mut cache: ResMut<LayoutCache>,
    mut params: ParamSet<(
        Query<(&IntrinsicSize, &UNode, Option<&USelf>)>,
        Query<(Entity, &UNode, &LayoutDepth, Option<&Children>, Option<&ULayout>, &mut IntrinsicSize)>,
    )>,
) {
    let start = Instant::now();
    
    // تشغيل النظام الأصلي
    upward_measure_pass_cached(tree_depth, cache, params);
    
    // حفظ الوقت
    profiler.upward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
}

/// Downward Pass مع Profiling مدمج
pub fn downward_pass_with_profiling(
    mut profiler: ResMut<LayoutProfiler>,
    tree_depth: Res<LayoutTreeDepth>,
    cache: Res<LayoutCache>,
    mut nodes: Query<(
        Entity, &UNode, Option<&ULayout>, &LayoutDepth,
        Option<&Children>, Option<&USelf>,
        &mut ComputedSize, &mut Transform
    )>,
    intrinsic_query: Query<&IntrinsicSize>,
    root_query: Query<&UWorldRoot>,
    window_query: Query<&Window>,
) {
    let start = Instant::now();
    
    // تشغيل النظام الأصلي
    downward_solve_pass_safe(
        tree_depth,
        cache,
        nodes,
        intrinsic_query,
        root_query,
        window_query,
    );
    
    // حفظ الوقت
    profiler.downward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
}

/// Material Update مع Profiling مدمج
pub fn material_update_with_profiling(
    mut profiler: ResMut<LayoutProfiler>,
    mut commands: Commands,
    mut pool: ResMut<MaterialPool>,
    mut query: Query<
        (
            Entity,
            &UNode,
            &ComputedSize,
            Option<&UBorder>,
            Option<&UImage>,
            Option<&UI3d>,
            Option<&UPbr>,
            Option<&mut MaterialHandles>,
        ),
        Or<(
            Changed<UNode>,
            Changed<ComputedSize>,
            Changed<UBorder>,
            Changed<UImage>,
            Changed<UI3d>,
            Changed<UPbr>,
        )>
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_2d: ResMut<Assets<UNodeMaterial>>,
    mut materials_3d: ResMut<Assets<UNodeMaterial3d>>,
) {
    let start = Instant::now();
    
    // حفظ العدادات قبل
    let created_before = pool.created_count;
    let reused_before = pool.reused_count;
    
    // تشغيل النظام الأصلي
    update_materials_optimized(
        commands,
        pool.reborrow(),
        query,
        meshes,
        materials_2d,
        materials_3d,
    );
    
    // تحديث الإحصائيات
    profiler.materials_created = pool.created_count - created_before;
    profiler.materials_reused = pool.reused_count - reused_before;
    profiler.material_update_time = start.elapsed().as_secs_f64() * 1000.0;
}

// =========================================================
// Plugin مع التكامل الصحيح
// =========================================================

pub struct IntegratedProfilingPlugin;

impl Plugin for IntegratedProfilingPlugin {
    fn build(&self, app: &mut App) {
        // التأكد من وجود الـ Resources
        if !app.world().contains_resource::<LayoutProfiler>() {
            app.init_resource::<LayoutProfiler>();
        }
        
        if !app.world().contains_resource::<ProfilerSettings>() {
            app.init_resource::<ProfilerSettings>();
        }
        
        // إضافة أنظمة العرض فقط (الأنظمة الأخرى يتم استبدالها)
        app
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_profiler_overlay)
            .add_systems(
                Update,
                (
                    collect_node_stats,
                    record_frame_stats,
                    profiler_controls,
                    update_profiler_overlay_text,
                    display_profiler_overlay,
                    log_performance_report,
                )
            );
    }
}

// =========================================================
// مثال على كيفية الاستخدام في Layout Plugin
// =========================================================

pub struct UnivisLayoutPluginWithProfiling;

impl Plugin for UnivisLayoutPluginWithProfiling {
    fn build(&self, app: &mut App) {
        app
            // الـ Resources الأساسية
            .init_resource::<LayoutTreeDepth>()
            .init_resource::<LayoutCache>()
            .init_resource::<LayoutProfiler>()
            
            // الأنظمة مع Profiling
            .add_systems(
                Update,
                (
                    // 1. تحديث الهرمية
                    update_layout_hierarchy,
                    
                    // 2. Upward Pass (مع Profiling)
                    upward_pass_with_profiling,
                    
                    // 3. Downward Pass (مع Profiling)
                    downward_pass_with_profiling,
                    
                    // 4. Material Update (مع Profiling)
                    material_update_with_profiling,
                )
                .chain()
            );
    }
}