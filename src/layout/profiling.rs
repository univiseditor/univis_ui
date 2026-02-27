use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::prelude::*;
use std::cmp::Ordering;
use std::time::Instant;

// =========================================================
// Diagnostic Paths
// =========================================================

pub const LAYOUT_UPWARD_TIME: &str = "layout_upward_ms";
pub const LAYOUT_DOWNWARD_TIME: &str = "layout_downward_ms";
pub const MATERIAL_UPDATE_TIME: &str = "material_update_ms";
pub const LAYOUT_TOTAL_TIME: &str = "layout_total_ms";

const DEFAULT_TARGET_FPS: f32 = 60.0;
const DEFAULT_PANEL_WIDTH: f32 = 520.0;
const DEFAULT_GRAPH_HEIGHT: f32 = 92.0;
const DEFAULT_GRAPH_SAMPLES: usize = 120;
const DEFAULT_TEXT_SECTION_HEIGHT: f32 = 132.0;
const DEFAULT_BARS_SECTION_HEIGHT: f32 = 64.0;
const DEFAULT_PANEL_PADDING: f32 = 14.0;
const DEFAULT_SECTION_GAP: f32 = 10.0;

// =========================================================
// Resources
// =========================================================

/// Real-time profiler resource for Univis layout and rendering.
#[derive(Resource)]
pub struct LayoutProfiler {
    // System timings in milliseconds
    pub upward_pass_time: f64,
    pub downward_pass_time: f64,
    pub material_update_time: f64,

    // Node stats
    pub total_nodes: usize,
    pub dirty_nodes: usize,
    pub visible_nodes: usize,

    // Frame history
    pub frame_history: Vec<FrameStats>,
    pub max_history: usize,

    // Optimization stats
    pub materials_created: usize,
    pub materials_reused: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

#[derive(Debug, Clone, Copy, Default)]
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

    pub fn latest_frame(&self) -> Option<FrameStats> {
        self.frame_history.last().copied()
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
        if self.frame_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.frame_history.iter().map(|s| s.total_layout_ms).sum();
        sum / self.frame_history.len() as f64
    }

    pub fn average_fps(&self) -> f64 {
        if self.frame_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.frame_history.iter().map(|s| s.fps).sum();
        sum / self.frame_history.len() as f64
    }

    pub fn average_total_time_recent(&self, sample_count: usize) -> f64 {
        if self.frame_history.is_empty() || sample_count == 0 {
            return 0.0;
        }
        let take = sample_count.min(self.frame_history.len());
        let slice = &self.frame_history[self.frame_history.len() - take..];
        let sum: f64 = slice.iter().map(|s| s.total_layout_ms).sum();
        sum / take as f64
    }

    pub fn max_total_time(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .max_by(safe_partial_cmp)
            .unwrap_or(0.0)
    }

    pub fn min_total_time(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .min_by(safe_partial_cmp)
            .unwrap_or(0.0)
    }

    pub fn percentile_total_time(&self, percentile: f64) -> f64 {
        if self.frame_history.is_empty() {
            return 0.0;
        }

        let p = percentile.clamp(0.0, 100.0) / 100.0;
        let mut values: Vec<f64> = self.frame_history.iter().map(|s| s.total_layout_ms).collect();
        values.sort_by(safe_partial_cmp);

        let idx = ((values.len() - 1) as f64 * p).round() as usize;
        values[idx]
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        (self.cache_hits as f64 / total as f64) * 100.0
    }

    pub fn material_reuse_rate(&self) -> f64 {
        let total = self.materials_created + self.materials_reused;
        if total == 0 {
            return 0.0;
        }
        (self.materials_reused as f64 / total as f64) * 100.0
    }

    pub fn dirty_ratio(&self) -> f64 {
        if self.total_nodes == 0 {
            return 0.0;
        }
        self.dirty_nodes as f64 / self.total_nodes as f64
    }

    pub fn visible_ratio(&self) -> f64 {
        if self.total_nodes == 0 {
            return 0.0;
        }
        self.visible_nodes as f64 / self.total_nodes as f64
    }

    pub fn timing_share(&self) -> (f64, f64, f64) {
        let total = self.total_time();
        if total <= f64::EPSILON {
            return (0.0, 0.0, 0.0);
        }

        (
            (self.upward_pass_time / total) * 100.0,
            (self.downward_pass_time / total) * 100.0,
            (self.material_update_time / total) * 100.0,
        )
    }
}

/// Overlay and reporting settings.
#[derive(Resource)]
pub struct ProfilerSettings {
    pub enabled: bool,
    pub show_overlay: bool,
    pub show_graph: bool,
    pub log_interval: f32,
    pub overlay_position: OverlayPosition,
    pub target_fps: f32,
    pub panel_opacity: f32,
    pub graph_samples: usize,
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
            show_graph: true,
            log_interval: 5.0,
            overlay_position: OverlayPosition::TopLeft,
            target_fps: DEFAULT_TARGET_FPS,
            panel_opacity: 0.78,
            graph_samples: DEFAULT_GRAPH_SAMPLES,
        }
    }
}

// =========================================================
// Profiling Systems
// =========================================================

/// Legacy stop/start timer helper for upward pass.
pub fn profile_upward_pass(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else {
        profiler.upward_pass_time = start_time.expect("timer should exist").elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}

/// Legacy stop/start timer helper for downward pass.
pub fn profile_downward_pass(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else {
        profiler.downward_pass_time = start_time.expect("timer should exist").elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}

/// Legacy stop/start timer helper for material update.
pub fn profile_material_update(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else {
        profiler.material_update_time = start_time.expect("timer should exist").elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}

/// Collect node-level runtime statistics.
pub fn collect_node_stats(
    mut profiler: ResMut<LayoutProfiler>,
    cache: Option<Res<LayoutCache>>,
    nodes: Query<Entity, With<UNode>>,
    visible: Query<&Visibility, With<UNode>>,
) {
    profiler.total_nodes = nodes.iter().count();

    if let Some(cache) = cache {
        let dirty = cache.dirty_count();
        profiler.dirty_nodes = dirty;
        profiler.cache_misses = dirty;
        profiler.cache_hits = profiler.total_nodes.saturating_sub(dirty);
    } else {
        profiler.dirty_nodes = 0;
        profiler.cache_hits = 0;
        profiler.cache_misses = profiler.total_nodes;
    }

    profiler.visible_nodes = visible
        .iter()
        .filter(|v| **v != Visibility::Hidden)
        .count();
}

/// Record per-frame timing and FPS snapshots.
pub fn record_frame_stats(
    mut profiler: ResMut<LayoutProfiler>,
    diagnostics: Res<DiagnosticsStore>,
    mut frame_counter: Local<u64>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    *frame_counter += 1;
    profiler.record_frame(*frame_counter, fps);
}

/// Render the profiling panel and graph using gizmos.
pub fn display_profiler_overlay(
    mut gizmos: Gizmos,
    profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    windows: Query<&Window>,
) {
    if !settings.enabled || !settings.show_overlay {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let panel_size = overlay_panel_size(&settings);
    let center = overlay_center(window, settings.overlay_position, panel_size);
    let half = panel_size * 0.5;
    let left = center.x - half.x;
    let right = center.x + half.x;
    let top = center.y + half.y;
    let bottom = center.y - half.y;
    let padding = DEFAULT_PANEL_PADDING;

    let background = Color::srgba(0.02, 0.03, 0.05, settings.panel_opacity);
    let border = Color::srgba(0.56, 0.64, 0.79, 0.95);
    let separator = Color::srgba(0.28, 0.35, 0.47, 0.9);

    gizmos.rect_2d(
        Isometry2d::from_xy(center.x, center.y),
        panel_size,
        background,
    );

    // Outer border
    gizmos.line_2d(Vec2::new(left, top), Vec2::new(right, top), border);
    gizmos.line_2d(Vec2::new(right, top), Vec2::new(right, bottom), border);
    gizmos.line_2d(Vec2::new(right, bottom), Vec2::new(left, bottom), border);
    gizmos.line_2d(Vec2::new(left, bottom), Vec2::new(left, top), border);

    // Section layout
    let text_top = top - padding;
    let text_bottom = text_top - DEFAULT_TEXT_SECTION_HEIGHT;
    let bars_top = text_bottom - DEFAULT_SECTION_GAP;
    let bars_bottom = bars_top - DEFAULT_BARS_SECTION_HEIGHT;

    // Section separators
    gizmos.line_2d(
        Vec2::new(left + padding, text_bottom - (DEFAULT_SECTION_GAP * 0.5)),
        Vec2::new(right - padding, text_bottom - (DEFAULT_SECTION_GAP * 0.5)),
        separator,
    );
    if settings.show_graph {
        gizmos.line_2d(
            Vec2::new(left + padding, bars_bottom - (DEFAULT_SECTION_GAP * 0.5)),
            Vec2::new(right - padding, bars_bottom - (DEFAULT_SECTION_GAP * 0.5)),
            separator,
        );
    }

    // Timing bars
    let bar_left = left + padding;
    let bar_width = panel_size.x - (padding * 2.0);
    let bar_height = 10.0;
    let total_ms = profiler.total_time().max(0.0001);

    draw_horizontal_bar(
        &mut gizmos,
        bar_left,
        bars_top - 14.0,
        bar_width,
        bar_height,
        (profiler.upward_pass_time / total_ms) as f32,
        Color::srgb(0.26, 0.64, 1.0),
    );
    draw_horizontal_bar(
        &mut gizmos,
        bar_left,
        bars_top - 32.0,
        bar_width,
        bar_height,
        (profiler.downward_pass_time / total_ms) as f32,
        Color::srgb(0.2, 0.84, 0.45),
    );
    draw_horizontal_bar(
        &mut gizmos,
        bar_left,
        bars_top - 50.0,
        bar_width,
        bar_height,
        (profiler.material_update_time / total_ms) as f32,
        Color::srgb(1.0, 0.67, 0.21),
    );

    if settings.show_graph {
        let graph_width = panel_size.x - (padding * 2.0);
        let graph_left = left + padding;
        let graph_top = bars_bottom - DEFAULT_SECTION_GAP;
        let graph_bottom = bottom + padding;
        let graph_height = (graph_top - graph_bottom).max(10.0);

        gizmos.rect_2d(
            Isometry2d::from_xy(
                graph_left + graph_width * 0.5,
                graph_bottom + graph_height * 0.5,
            ),
            Vec2::new(graph_width, graph_height),
            Color::srgba(0.06, 0.08, 0.12, 0.92),
        );

        let grid_color = Color::srgba(0.2, 0.26, 0.34, 0.65);
        for row in 0..=4 {
            let t = row as f32 / 4.0;
            let y = graph_bottom + t * graph_height;
            gizmos.line_2d(
                Vec2::new(graph_left, y),
                Vec2::new(graph_left + graph_width, y),
                grid_color,
            );
        }

        let sample_count = settings.graph_samples.max(2).min(profiler.frame_history.len());
        if sample_count >= 2 {
            let slice = &profiler.frame_history[profiler.frame_history.len() - sample_count..];
            let frame_budget = frame_budget_ms(settings.target_fps);
            let max_ms = slice
                .iter()
                .map(|s| s.total_layout_ms)
                .fold(frame_budget, f64::max)
                .max(1.0);

            let budget_y = graph_bottom + ((frame_budget / max_ms) as f32).clamp(0.0, 1.0) * graph_height;
            gizmos.line_2d(
                Vec2::new(graph_left, budget_y),
                Vec2::new(graph_left + graph_width, budget_y),
                Color::srgba(0.96, 0.84, 0.28, 0.95),
            );

            for i in 1..slice.len() {
                let prev = slice[i - 1].total_layout_ms;
                let curr = slice[i].total_layout_ms;
                let x0 = graph_left + ((i - 1) as f32 / (slice.len() - 1) as f32) * graph_width;
                let x1 = graph_left + (i as f32 / (slice.len() - 1) as f32) * graph_width;
                let y0 = graph_bottom + ((prev / max_ms) as f32).clamp(0.0, 1.0) * graph_height;
                let y1 = graph_bottom + ((curr / max_ms) as f32).clamp(0.0, 1.0) * graph_height;
                gizmos.line_2d(
                    Vec2::new(x0, y0),
                    Vec2::new(x1, y1),
                    performance_color(curr, frame_budget),
                );
            }
        }
    }
}

/// Log periodic performance reports to the console.
pub fn log_performance_report(
    _profiler: Res<LayoutProfiler>,
    _settings: Res<ProfilerSettings>,
    _timer: Local<f32>,
    _time: Res<Time>,
) {
    // intentionally no terminal output
}

/// Keyboard controls for profiler visibility and panel behavior.
pub fn profiler_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<ProfilerSettings>,
) {
    // F10: profiler ON/OFF
    if keyboard.just_pressed(KeyCode::F10) {
        settings.enabled = !settings.enabled;
    }

    // F11: overlay ON/OFF
    if keyboard.just_pressed(KeyCode::F11) {
        settings.show_overlay = !settings.show_overlay;
    }

    // F9: graph ON/OFF
    if keyboard.just_pressed(KeyCode::F9) {
        settings.show_graph = !settings.show_graph;
    }

    // F12: cycle overlay position
    if keyboard.just_pressed(KeyCode::F12) {
        settings.overlay_position = match settings.overlay_position {
            OverlayPosition::TopLeft => OverlayPosition::TopRight,
            OverlayPosition::TopRight => OverlayPosition::BottomRight,
            OverlayPosition::BottomRight => OverlayPosition::BottomLeft,
            OverlayPosition::BottomLeft => OverlayPosition::TopLeft,
        };
    }
}

// =========================================================
// Text Overlay System
// =========================================================

#[derive(Component)]
struct ProfilerOverlayText;

fn setup_profiler_overlay(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Text2d::new("UNIVIS PROFILER"),
        TextLayout {
            justify: Justify::Left,
            linebreak: LineBreak::NoWrap,
            ..default()
        },
        TextFont {
            font: theme.text.font.inter_regular.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.94, 1.0)),
        Anchor::TOP_LEFT,
        Transform::from_xyz(0.0, 0.0, 50.0),
        Visibility::Hidden,
        ProfilerOverlayText,
    ));
}

fn update_profiler_overlay_text(
    profiler: Res<LayoutProfiler>,
    settings: Res<ProfilerSettings>,
    mut query: Query<(&mut Text2d, &mut Transform, &mut Visibility, &mut TextColor), With<ProfilerOverlayText>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let panel_size = overlay_panel_size(&settings);
    let center = overlay_center(window, settings.overlay_position, panel_size);
    let half = panel_size * 0.5;
    let panel_left = center.x - half.x;
    let panel_top = center.y + half.y;

    let latest = profiler.latest_frame().unwrap_or_default();
    let recent_avg = profiler.average_total_time_recent(30);
    let p95 = profiler.percentile_total_time(95.0);
    let frame_budget = frame_budget_ms(settings.target_fps);
    let budget_usage = if frame_budget > 0.0 {
        (profiler.total_time() / frame_budget) * 100.0
    } else {
        0.0
    };
    let (up_share, down_share, mat_share) = profiler.timing_share();
    let perf_state = performance_label(profiler.total_time(), frame_budget);

    let text_color_value = match perf_state {
        "GOOD" => Color::srgb(0.72, 0.96, 0.77),
        "WARN" => Color::srgb(1.0, 0.88, 0.62),
        _ => Color::srgb(1.0, 0.72, 0.72),
    };

    let overlay_text = format!(
        "UNIVIS PROFILER [{}]\n\
FPS now/avg: {:>5.1} / {:>5.1}   frame #{:>6}\n\
Frame ms now/avg/p95/max: {:>6.2} / {:>6.2} / {:>6.2} / {:>6.2}\n\
Budget @{:>3.0}fps: {:>6.2}ms  usage {:>6.1}%\n\
Breakdown up/down/mat: {:>4.1}% / {:>4.1}% / {:>4.1}%\n\
Nodes total/dirty/visible: {} / {} / {}\n\
Cache hit: {:>5.1}%   Material reuse: {:>5.1}%\n\
History: {:>3} frames | Recent30 avg: {:>6.2}ms | Graph: {}\n\
Keys: F10 profiler  F11 overlay  F9 graph  F12 move",
        perf_state,
        latest.fps,
        profiler.average_fps(),
        latest.frame,
        profiler.total_time(),
        profiler.average_total_time(),
        p95,
        profiler.max_total_time(),
        settings.target_fps,
        frame_budget,
        budget_usage,
        up_share,
        down_share,
        mat_share,
        latest.node_count,
        latest.dirty_count,
        profiler.visible_nodes,
        profiler.cache_hit_rate(),
        profiler.material_reuse_rate(),
        profiler.frame_history.len(),
        recent_avg,
        if settings.show_graph { "ON" } else { "OFF" },
    );

    for (mut text, mut transform, mut visibility, mut text_color) in query.iter_mut() {
        if !settings.enabled || !settings.show_overlay {
            *visibility = Visibility::Hidden;
            continue;
        }

        *visibility = Visibility::Visible;
        transform.translation = Vec3::new(
            panel_left + DEFAULT_PANEL_PADDING,
            panel_top - DEFAULT_PANEL_PADDING,
            50.0,
        );
        **text = overlay_text.clone();
        text_color.0 = text_color_value;
    }
}

// =========================================================
// Helper Functions
// =========================================================

fn safe_partial_cmp(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}

fn frame_budget_ms(target_fps: f32) -> f64 {
    let fps = target_fps.max(1.0) as f64;
    1000.0 / fps
}

fn overlay_panel_size(settings: &ProfilerSettings) -> Vec2 {
    let base_height = (DEFAULT_PANEL_PADDING * 2.0)
        + DEFAULT_TEXT_SECTION_HEIGHT
        + DEFAULT_SECTION_GAP
        + DEFAULT_BARS_SECTION_HEIGHT;

    if settings.show_graph {
        Vec2::new(
            DEFAULT_PANEL_WIDTH,
            base_height + DEFAULT_SECTION_GAP + DEFAULT_GRAPH_HEIGHT,
        )
    } else {
        Vec2::new(DEFAULT_PANEL_WIDTH, base_height)
    }
}

fn overlay_center(window: &Window, position: OverlayPosition, panel_size: Vec2) -> Vec2 {
    let margin = 14.0;
    let half_window = Vec2::new(window.width() * 0.5, window.height() * 0.5);
    let half_panel = panel_size * 0.5;

    match position {
        OverlayPosition::TopLeft => Vec2::new(
            -half_window.x + margin + half_panel.x,
            half_window.y - margin - half_panel.y,
        ),
        OverlayPosition::TopRight => Vec2::new(
            half_window.x - margin - half_panel.x,
            half_window.y - margin - half_panel.y,
        ),
        OverlayPosition::BottomLeft => Vec2::new(
            -half_window.x + margin + half_panel.x,
            -half_window.y + margin + half_panel.y,
        ),
        OverlayPosition::BottomRight => Vec2::new(
            half_window.x - margin - half_panel.x,
            -half_window.y + margin + half_panel.y,
        ),
    }
}

fn draw_horizontal_bar(
    gizmos: &mut Gizmos,
    left: f32,
    center_y: f32,
    width: f32,
    height: f32,
    ratio: f32,
    fill_color: Color,
) {
    let bg = Color::srgba(0.15, 0.19, 0.26, 0.92);
    gizmos.rect_2d(
        Isometry2d::from_xy(left + width * 0.5, center_y),
        Vec2::new(width, height),
        bg,
    );

    let fill_ratio = ratio.clamp(0.0, 1.0);
    if fill_ratio <= 0.0 {
        return;
    }

    let fill_width = width * fill_ratio;
    gizmos.rect_2d(
        Isometry2d::from_xy(left + fill_width * 0.5, center_y),
        Vec2::new(fill_width, height),
        fill_color,
    );
}

fn performance_label(total_ms: f64, frame_budget: f64) -> &'static str {
    if frame_budget <= f64::EPSILON {
        return "N/A";
    }
    if total_ms <= frame_budget * 0.7 {
        "GOOD"
    } else if total_ms <= frame_budget {
        "WARN"
    } else {
        "HOT"
    }
}

fn performance_color(total_ms: f64, frame_budget: f64) -> Color {
    match performance_label(total_ms, frame_budget) {
        "GOOD" => Color::srgb(0.24, 0.86, 0.49),
        "WARN" => Color::srgb(1.0, 0.74, 0.24),
        _ => Color::srgb(0.98, 0.33, 0.33),
    }
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
                ),
            );
    }
}
