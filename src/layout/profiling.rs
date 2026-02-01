use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use std::time::Instant;
use crate::prelude::*;
// =========================================================
// Diagnostic Paths
// =========================================================

pub const LAYOUT_UPWARD_TIME: &str = "layout_upward_ms";
pub const LAYOUT_DOWNWARD_TIME: &str = "layout_downward_ms";
pub const MATERIAL_UPDATE_TIME: &str = "material_update_ms";
pub const LAYOUT_TOTAL_TIME: &str = "layout_total_ms";

// =========================================================
// Resources
// =========================================================

/// Resource لتتبع الأداء في الوقت الفعلي
#[derive(Resource)]
pub struct LayoutProfiler {
    // أوقات الأنظمة (بالميلي ثانية)
    pub upward_pass_time: f64,
    pub downward_pass_time: f64,
    pub material_update_time: f64,
    
    // إحصائيات العقد
    pub total_nodes: usize,
    pub dirty_nodes: usize,
    pub visible_nodes: usize,
    
    // سجل الإطارات (آخر 300 إطار = ~5 ثواني عند 60 FPS)
    pub frame_history: Vec<FrameStats>,
    pub max_history: usize,
    
    // معلومات إضافية
    pub materials_created: usize,
    pub materials_reused: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameStats {
    pub frame: u64,
    pub upward_ms: f64,
    pub downward_ms: f64,
    pub material_ms: f64,
    pub total_layout_ms: f64,
    pub fps: f64,
    pub node_count: usize,
    pub dirty_count: usize,
}

impl Default for LayoutProfiler {
    fn default() -> Self {
        Self {
            upward_pass_time: 0.0,
            downward_pass_time: 0.0,
            material_update_time: 0.0,
            total_nodes: 0,
            dirty_nodes: 0,
            visible_nodes: 0,
            frame_history: Vec::new(),
            max_history: 300,
            materials_created: 0,
            materials_reused: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

impl LayoutProfiler {
    pub fn total_time(&self) -> f64 {
        self.upward_pass_time + self.downward_pass_time + self.material_update_time
    }

    pub fn record_frame(&mut self, frame: u64, fps: f64) {
        let stats = FrameStats {
            frame,
            upward_ms: self.upward_pass_time,
            downward_ms: self.downward_pass_time,
            material_ms: self.material_update_time,
            total_layout_ms: self.total_time(),
            fps,
            node_count: self.total_nodes,
            dirty_count: self.dirty_nodes,
        };

        self.frame_history.push(stats);

        if self.frame_history.len() > self.max_history {
            self.frame_history.remove(0);
        }
    }

    pub fn average_total_time(&self) -> f64 {
        if self.frame_history.is_empty() { return 0.0; }
        let sum: f64 = self.frame_history.iter().map(|s| s.total_layout_ms).sum();
        sum / self.frame_history.len() as f64
    }

    pub fn max_total_time(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    pub fn min_total_time(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    pub fn average_fps(&self) -> f64 {
        if self.frame_history.is_empty() { return 0.0; }
        let sum: f64 = self.frame_history.iter().map(|s| s.fps).sum();
        sum / self.frame_history.len() as f64
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 { return 0.0; }
        (self.cache_hits as f64 / total as f64) * 100.0
    }

    pub fn material_reuse_rate(&self) -> f64 {
        let total = self.materials_created + self.materials_reused;
        if total == 0 { return 0.0; }
        (self.materials_reused as f64 / total as f64) * 100.0
    }
}

/// إعدادات عرض الأداء
#[derive(Resource)]
pub struct ProfilerSettings {
    pub enabled: bool,
    pub show_overlay: bool,
    pub show_graph: bool,
    pub log_interval: f32,
    pub overlay_position: OverlayPosition,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for ProfilerSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_overlay: true,
            show_graph: false,
            log_interval: 5.0,
            overlay_position: OverlayPosition::TopLeft,
        }
    }
}

// =========================================================
// Profiling Systems
// =========================================================

/// نظام لتتبع وقت Upward Pass
pub fn profile_upward_pass(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else {
        profiler.upward_pass_time = start_time.unwrap().elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}

/// نظام لتتبع وقت Downward Pass
pub fn profile_downward_pass(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else {
        profiler.downward_pass_time = start_time.unwrap().elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}

/// نظام لتتبع وقت Material Update
pub fn profile_material_update(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else {
        profiler.material_update_time = start_time.unwrap().elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}

/// جمع إحصائيات العقد
pub fn collect_node_stats(
    mut profiler: ResMut<LayoutProfiler>,
    cache: Option<Res<LayoutCache>>,
    nodes: Query<Entity, With<UNode>>,
    visible: Query<&Visibility>,
) {
    profiler.total_nodes = nodes.iter().count();
    
    if let Some(cache) = cache {
        let dirty = cache.dirty_count();
        profiler.dirty_nodes = dirty;
        
        // ✅ التصحيح: Cache Hits = العقد النظيفة (لم تُعاد حسابها)
        // Cache Misses = العقد المتسخة (تمت إعادة حسابها)
        profiler.cache_misses = dirty;
        profiler.cache_hits = profiler.total_nodes.saturating_sub(dirty);
    } else {
        // لا يوجد Cache، كل شيء Miss
        profiler.dirty_nodes = 0;
        profiler.cache_hits = 0;
        profiler.cache_misses = profiler.total_nodes;
    }
    
    profiler.visible_nodes = visible.iter()
        .filter(|v| **v == Visibility::Visible)
        .count();
}

/// تسجيل إحصائيات الإطار
pub fn record_frame_stats(
    mut profiler: ResMut<LayoutProfiler>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
) {
    // حساب FPS من DiagnosticsStore
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    
    // استخدام elapsed_secs_f64 كـ frame counter بديل
    let frame = (time.elapsed_secs_f64() * 60.0) as u64;
    
    profiler.record_frame(frame, fps);
}

/// عرض معلومات الأداء على الشاشة
pub fn display_profiler_overlay(
    mut gizmos: Gizmos,
    _profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    windows: Query<&Window>,
) {
    if !settings.enabled || !settings.show_overlay {
        return;
    }

    let Ok(window) = windows.single() else { return };
    
    let (x, y) = match settings.overlay_position {
        OverlayPosition::TopLeft => (20.0, window.height() - 20.0),
        OverlayPosition::TopRight => (window.width() - 250.0, window.height() - 20.0),
        OverlayPosition::BottomLeft => (20.0, 150.0),
        OverlayPosition::BottomRight => (window.width() - 250.0, 150.0),
    };

    // رسم خلفية شفافة
    gizmos.rect_2d(
        Isometry2d::from_xy(x + 115.0, y - 75.0),
        Vec2::new(230.0, 130.0),
        Color::srgba(0.0, 0.0, 0.0, 0.7),
    );

    // النص سيتم رسمه باستخدام Text في نظام منفصل
}

/// نظام لطباعة تقرير دوري في Console
pub fn log_performance_report(
    profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    if !settings.enabled {
        return;
    }

    *timer += time.delta_secs();

    if *timer >= settings.log_interval {
        info!("╔════════════════════════════════════════════════════╗");
        info!("║         UNIVIS LAYOUT PERFORMANCE REPORT          ║");
        info!("╠════════════════════════════════════════════════════╣");
        info!("║ Frame Stats:                                       ║");
        info!("║   Average FPS: {:.1}", profiler.average_fps());
        info!("║   Total Nodes: {}", profiler.total_nodes);
        info!("║   Dirty Nodes: {} ({:.1}%)", 
            profiler.dirty_nodes,
            (profiler.dirty_nodes as f64 / profiler.total_nodes.max(1) as f64) * 100.0
        );
        info!("║   Visible Nodes: {}", profiler.visible_nodes);
        info!("╠════════════════════════════════════════════════════╣");
        info!("║ Layout Timing (ms):                                ║");
        info!("║   Upward Pass:    {:.3}", profiler.upward_pass_time);
        info!("║   Downward Pass:  {:.3}", profiler.downward_pass_time);
        info!("║   Material Update: {:.3}", profiler.material_update_time);
        info!("║   ─────────────────────────────────────────────    ║");
        info!("║   Total:          {:.3}", profiler.total_time());
        info!("╠════════════════════════════════════════════════════╣");
        info!("║ Statistics (last {} frames):                      ║", profiler.frame_history.len());
        info!("║   Average:  {:.3} ms", profiler.average_total_time());
        info!("║   Min:      {:.3} ms", profiler.min_total_time());
        info!("║   Max:      {:.3} ms", profiler.max_total_time());
        info!("╠════════════════════════════════════════════════════╣");
        info!("║ Optimization Metrics:                              ║");
        info!("║   Cache Hit Rate:     {:.1}%", profiler.cache_hit_rate());
        info!("║   Material Reuse:     {:.1}%", profiler.material_reuse_rate());
        info!("║   Materials Created:  {}", profiler.materials_created);
        info!("║   Materials Reused:   {}", profiler.materials_reused);
        info!("╚════════════════════════════════════════════════════╝");

        *timer = 0.0;
    }
}

/// نظام للتحكم بالـ Profiler عبر لوحة المفاتيح
pub fn profiler_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<ProfilerSettings>,
) {
    // F10: تشغيل/إيقاف Profiler
    if keyboard.just_pressed(KeyCode::F10) {
        settings.enabled = !settings.enabled;
        info!("Profiler: {}", if settings.enabled { "ON" } else { "OFF" });
    }

    // F11: عرض/إخفاء Overlay
    if keyboard.just_pressed(KeyCode::F11) {
        settings.show_overlay = !settings.show_overlay;
        info!("Overlay: {}", if settings.show_overlay { "ON" } else { "OFF" });
    }

    // F12: تغيير موضع Overlay
    if keyboard.just_pressed(KeyCode::F12) {
        settings.overlay_position = match settings.overlay_position {
            OverlayPosition::TopLeft => OverlayPosition::TopRight,
            OverlayPosition::TopRight => OverlayPosition::BottomRight,
            OverlayPosition::BottomRight => OverlayPosition::BottomLeft,
            OverlayPosition::BottomLeft => OverlayPosition::TopLeft,
        };
        info!("Overlay Position: {:?}", settings.overlay_position);
    }
}

// =========================================================
// Text Overlay System (باستخدام Text2d)
// =========================================================

#[derive(Component)]
struct ProfilerOverlayText;

fn setup_profiler_overlay(
    mut commands: Commands,
) {
    commands.spawn((
        Text2d::default(),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, 0.0, 1000.0), // Z عالي لضمان الظهور
        ProfilerOverlayText,
    ));
}

fn update_profiler_overlay_text(
    profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    mut query: Query<(&mut Text2d, &mut Transform, &mut Visibility), With<ProfilerOverlayText>>,
    windows: Query<&Window>,
) {
    let Ok((mut text, mut transform, mut visibility)) = query.single_mut() else {
        return;
    };

    if !settings.enabled || !settings.show_overlay {
        *visibility = Visibility::Hidden;
        return;
    } else {
        *visibility = Visibility::Visible;
    }

    let Ok(window) = windows.single() else { return };

    // تحديد الموضع
    let (x, y) = match settings.overlay_position {
        OverlayPosition::TopLeft => (-window.width() / 2.0 + 120.0, window.height() / 2.0 - 80.0),
        OverlayPosition::TopRight => (window.width() / 2.0 - 120.0, window.height() / 2.0 - 80.0),
        OverlayPosition::BottomLeft => (-window.width() / 2.0 + 120.0, -window.height() / 2.0 + 80.0),
        OverlayPosition::BottomRight => (window.width() / 2.0 - 120.0, -window.height() / 2.0 + 80.0),
    };

    transform.translation = Vec3::new(x, y, 1000.0);

    // بناء النص
    **text = format!(
        "┌─ UNIVIS PROFILER ─┐\n\
         │ FPS: {:.1}\n\
         │ Nodes: {} ({} dirty)\n\
         ├─ Layout Time ─────┤\n\
         │ Up:   {:.2}ms\n\
         │ Down: {:.2}ms\n\
         │ Mat:  {:.2}ms\n\
         │ Total: {:.2}ms\n\
         ├─ Stats ───────────┤\n\
         │ Avg: {:.2}ms\n\
         │ Max: {:.2}ms\n\
         │ Cache: {:.0}%\n\
         └───────────────────┘",
        profiler.average_fps(),
        profiler.total_nodes,
        profiler.dirty_nodes,
        profiler.upward_pass_time,
        profiler.downward_pass_time,
        profiler.material_update_time,
        profiler.total_time(),
        profiler.average_total_time(),
        profiler.max_total_time(),
        profiler.cache_hit_rate(),
    );
}

// =========================================================
// Plugin
// =========================================================

pub struct LayoutProfilingPlugin;

impl Plugin for LayoutProfilingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LayoutProfiler>()
            .init_resource::<ProfilerSettings>()
            
            // إضافة FrameTimeDiagnosticsPlugin مع الإعدادات الافتراضية
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            
            .add_systems(Startup, setup_profiler_overlay)
            
            .add_systems(
                Update,
                (
                    // جمع الإحصائيات
                    collect_node_stats,
                    record_frame_stats,
                    
                    // التحكم
                    profiler_controls,
                    
                    // العرض
                    update_profiler_overlay_text,
                    display_profiler_overlay,
                    log_performance_report,
                )
            );
    }
}