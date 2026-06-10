mod overlay;
mod state;
mod systems;
mod timers;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::cmp::Ordering;
use std::time::Instant;
use univis_ui_style::style::Theme;

use crate::layout::core::layout_cache::LayoutCache;
use crate::layout::univis_node::UNode;

pub use self::overlay::display_profiler_overlay;
pub use self::state::{
    FrameStats, LAYOUT_DOWNWARD_TIME, LAYOUT_TOTAL_TIME, LAYOUT_UPWARD_TIME, LayoutProfiler,
    MATERIAL_UPDATE_TIME, OverlayPosition, ProfilerSettings,
};
pub use self::systems::{
    collect_node_stats, log_performance_report, profiler_controls, record_frame_stats,
};
pub use self::timers::{profile_downward_pass, profile_material_update, profile_upward_pass};

use self::overlay::{setup_profiler_overlay, update_profiler_overlay_text};

const DEFAULT_TARGET_FPS: f32 = 60.0;
const DEFAULT_PANEL_WIDTH: f32 = 520.0;
const DEFAULT_GRAPH_HEIGHT: f32 = 92.0;
const DEFAULT_GRAPH_SAMPLES: usize = 120;
const DEFAULT_TEXT_SECTION_HEIGHT: f32 = 152.0;
const DEFAULT_BARS_SECTION_HEIGHT: f32 = 64.0;
const DEFAULT_PANEL_PADDING: f32 = 14.0;
const DEFAULT_SECTION_GAP: f32 = 10.0;

/// Bevy plugin that enables visual profiling for the UI layout engine.
pub struct UnivisLayoutProfilingPlugin;

impl Plugin for UnivisLayoutProfilingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LayoutProfiler>()
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

/// Legacy alias for the profiling plugin.
#[deprecated(note = "Use `UnivisLayoutProfilingPlugin` instead.")]
pub type LayoutProfilingPlugin = UnivisLayoutProfilingPlugin;
