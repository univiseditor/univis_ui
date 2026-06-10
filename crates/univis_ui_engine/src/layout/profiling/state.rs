use super::*;

/// Metric key for upward pass layout time.
pub const LAYOUT_UPWARD_TIME: &str = "layout_upward_ms";
/// Metric key for downward pass layout time.
pub const LAYOUT_DOWNWARD_TIME: &str = "layout_downward_ms";
/// Metric key for material update time.
pub const MATERIAL_UPDATE_TIME: &str = "material_update_ms";
/// Metric key for total layout time.
pub const LAYOUT_TOTAL_TIME: &str = "layout_total_ms";

/// Real-time profiler resource for Univis layout and rendering.
#[derive(Resource)]
pub struct LayoutProfiler {
    /// Time spent in the upward intrinsic measurement pass.
    pub upward_pass_time: f64,
    /// Time spent in the downward explicit sizing pass.
    pub downward_pass_time: f64,
    /// Time spent updating UI materials and meshes.
    pub material_update_time: f64,
    /// Total number of UI nodes in the active hierarchy.
    pub total_nodes: usize,
    /// Number of UI nodes marked dirty for layout.
    pub dirty_nodes: usize,
    /// Number of UI nodes successfully solved this frame.
    pub solved_nodes: usize,
    /// Number of UI nodes physically rendered.
    pub visible_nodes: usize,
    /// Historical frame data for charting and rolling averages.
    pub frame_history: Vec<FrameStats>,
    /// Maximum number of historical frames to retain.
    pub max_history: usize,
    /// Number of new UI materials created this frame.
    pub materials_created: usize,
    /// Number of existing UI materials reused.
    pub materials_reused: usize,
    /// Number of new UI meshes created this frame.
    pub meshes_created: usize,
    /// Number of existing UI meshes reused.
    pub meshes_reused: usize,
    /// Number of successful layout cache hits.
    pub cache_hits: usize,
    /// Number of layout cache misses.
    pub cache_misses: usize,
    /// Reallocations during measure scratch space.
    pub measure_scratch_alloc_grows: usize,
    /// Reallocations during solve scratch space.
    pub solve_scratch_alloc_grows: usize,
    /// Reallocations during solver references tracking.
    pub solve_ref_alloc_grows: usize,
    /// Peak items tracked in measure scratch space.
    pub measure_scratch_peak: usize,
    /// Peak items tracked in solve scratch space.
    pub solve_scratch_peak: usize,
    /// Peak items tracked in solve reference space.
    pub solve_ref_peak: usize,
    /// Number of buckets in the picking spatial hash.
    pub picking_bucket_count: usize,
    /// Number of candidate nodes matched during picking.
    pub picking_candidate_count: usize,
}

/// Snapshot of performance statistics for a single frame.
#[derive(Debug, Clone, Copy, Default)]
pub struct FrameStats {
    /// The frame number.
    pub frame: u64,
    /// Time spent in the upward intrinsic measurement pass.
    pub upward_ms: f64,
    /// Time spent in the downward explicit sizing pass.
    pub downward_ms: f64,
    /// Time spent updating UI materials and meshes.
    pub material_ms: f64,
    /// Total time spent across all layout and render update passes.
    pub total_layout_ms: f64,
    /// The overall frames-per-second recorded at this layout frame.
    pub fps: f64,
    /// Total active UI nodes in this frame.
    pub node_count: usize,
    /// Number of UI nodes invalidated/dirty in this frame.
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
    /// Resets transient per-frame counters at the start of a frame.
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

    /// Returns the combined time spent on upward, downward, and material updates.
    pub fn total_time(&self) -> f64 {
        self.upward_pass_time + self.downward_pass_time + self.material_update_time
    }

    /// Retrieves the most recent frame statistics, if available.
    pub fn latest_frame(&self) -> Option<FrameStats> {
        self.frame_history.last().copied()
    }

    /// Records the current layout metrics as a new frame in the history log.
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

    /// Calculates the average total layout time across all recorded historical frames.
    pub fn average_total_time(&self) -> f64 {
        if self.frame_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.frame_history.iter().map(|s| s.total_layout_ms).sum();
        sum / self.frame_history.len() as f64
    }

    /// Calculates the average FPS across all recorded historical frames.
    pub fn average_fps(&self) -> f64 {
        if self.frame_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.frame_history.iter().map(|s| s.fps).sum();
        sum / self.frame_history.len() as f64
    }

    /// Calculates the average total layout time over the most recent `sample_count` frames.
    pub fn average_total_time_recent(&self, sample_count: usize) -> f64 {
        if self.frame_history.is_empty() || sample_count == 0 {
            return 0.0;
        }
        let take = sample_count.min(self.frame_history.len());
        let slice = &self.frame_history[self.frame_history.len() - take..];
        let sum: f64 = slice.iter().map(|s| s.total_layout_ms).sum();
        sum / take as f64
    }

    /// Finds the maximum layout time recorded in the frame history.
    pub fn max_total_time(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .max_by(safe_partial_cmp)
            .unwrap_or(0.0)
    }

    /// Finds the minimum layout time recorded in the frame history.
    pub fn min_total_time(&self) -> f64 {
        self.frame_history
            .iter()
            .map(|s| s.total_layout_ms)
            .min_by(safe_partial_cmp)
            .unwrap_or(0.0)
    }

    /// Retrieves the layout time at the given percentile (e.g., 99.0 for 99th percentile).
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

    /// Computes the percentage of cache requests that were fulfilled without recalculation.
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        (self.cache_hits as f64 / total as f64) * 100.0
    }

    /// Computes the percentage of materials that were reused instead of re-instantiated.
    pub fn material_reuse_rate(&self) -> f64 {
        let total = self.materials_created + self.materials_reused;
        if total == 0 {
            return 0.0;
        }
        (self.materials_reused as f64 / total as f64) * 100.0
    }

    /// Computes the percentage of meshes that were reused instead of regenerated.
    pub fn mesh_reuse_rate(&self) -> f64 {
        let total = self.meshes_created + self.meshes_reused;
        if total == 0 {
            return 0.0;
        }
        (self.meshes_reused as f64 / total as f64) * 100.0
    }

    /// Returns the fraction of nodes that required layout updates this frame.
    pub fn dirty_ratio(&self) -> f64 {
        if self.total_nodes == 0 {
            return 0.0;
        }
        self.dirty_nodes as f64 / self.total_nodes as f64
    }

    /// Returns the fraction of layout nodes that were ultimately visible to the renderer.
    pub fn visible_ratio(&self) -> f64 {
        if self.total_nodes == 0 {
            return 0.0;
        }
        self.visible_nodes as f64 / self.total_nodes as f64
    }

    /// Returns the relative percentage breakdowns of layout time: (upward, downward, material).
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
    /// Enable or disable profiling globally.
    pub enabled: bool,
    /// Whether to draw the on-screen profiler overlay.
    pub show_overlay: bool,
    /// Whether to display historical graphs in the overlay.
    pub show_graph: bool,
    /// Periodic terminal log interval in seconds.
    pub log_interval: f32,
    /// Which corner of the screen the overlay is anchored to.
    pub overlay_position: OverlayPosition,
    /// Target frame rate used to scale the performance graph.
    pub target_fps: f32,
    /// Opacity factor for the overlay panel background.
    pub panel_opacity: f32,
    /// Number of history data points rendered in the performance graph.
    pub graph_samples: usize,
}

/// Screen anchor position for the profiler overlay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverlayPosition {
    /// Top left corner of the screen.
    TopLeft,
    /// Top right corner of the screen.
    TopRight,
    /// Bottom left corner of the screen.
    BottomLeft,
    /// Bottom right corner of the screen.
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
