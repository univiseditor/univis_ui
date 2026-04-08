use super::*;

pub const LAYOUT_UPWARD_TIME: &str = "layout_upward_ms";
pub const LAYOUT_DOWNWARD_TIME: &str = "layout_downward_ms";
pub const MATERIAL_UPDATE_TIME: &str = "material_update_ms";
pub const LAYOUT_TOTAL_TIME: &str = "layout_total_ms";

/// Real-time profiler resource for Univis layout and rendering.
#[derive(Resource)]
pub struct LayoutProfiler {
    pub upward_pass_time: f64,
    pub downward_pass_time: f64,
    pub material_update_time: f64,
    pub total_nodes: usize,
    pub dirty_nodes: usize,
    pub solved_nodes: usize,
    pub visible_nodes: usize,
    pub frame_history: Vec<FrameStats>,
    pub max_history: usize,
    pub materials_created: usize,
    pub materials_reused: usize,
    pub meshes_created: usize,
    pub meshes_reused: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub measure_scratch_alloc_grows: usize,
    pub solve_scratch_alloc_grows: usize,
    pub solve_ref_alloc_grows: usize,
    pub measure_scratch_peak: usize,
    pub solve_scratch_peak: usize,
    pub solve_ref_peak: usize,
    pub picking_bucket_count: usize,
    pub picking_candidate_count: usize,
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
            solved_nodes: 0,
            visible_nodes: 0,
            frame_history: Vec::new(),
            max_history: 300,
            materials_created: 0,
            materials_reused: 0,
            meshes_created: 0,
            meshes_reused: 0,
            cache_hits: 0,
            cache_misses: 0,
            measure_scratch_alloc_grows: 0,
            solve_scratch_alloc_grows: 0,
            solve_ref_alloc_grows: 0,
            measure_scratch_peak: 0,
            solve_scratch_peak: 0,
            solve_ref_peak: 0,
            picking_bucket_count: 0,
            picking_candidate_count: 0,
        }
    }
}

impl LayoutProfiler {
    pub fn begin_frame(&mut self) {
        self.solved_nodes = 0;
        self.measure_scratch_alloc_grows = 0;
        self.solve_scratch_alloc_grows = 0;
        self.solve_ref_alloc_grows = 0;
        self.measure_scratch_peak = 0;
        self.solve_scratch_peak = 0;
        self.solve_ref_peak = 0;
        self.picking_bucket_count = 0;
        self.picking_candidate_count = 0;
    }

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
        let mut values: Vec<f64> = self
            .frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .collect();
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

    pub fn mesh_reuse_rate(&self) -> f64 {
        let total = self.meshes_created + self.meshes_reused;
        if total == 0 {
            return 0.0;
        }
        (self.meshes_reused as f64 / total as f64) * 100.0
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

fn safe_partial_cmp(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}
