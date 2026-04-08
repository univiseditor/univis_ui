use super::*;

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

    profiler.visible_nodes = visible.iter().filter(|v| **v != Visibility::Hidden).count();
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
    if keyboard.just_pressed(KeyCode::F10) {
        settings.enabled = !settings.enabled;
    }

    if keyboard.just_pressed(KeyCode::F11) {
        settings.show_overlay = !settings.show_overlay;
    }

    if keyboard.just_pressed(KeyCode::F9) {
        settings.show_graph = !settings.show_graph;
    }

    if keyboard.just_pressed(KeyCode::F12) {
        settings.overlay_position = match settings.overlay_position {
            OverlayPosition::TopLeft => OverlayPosition::TopRight,
            OverlayPosition::TopRight => OverlayPosition::BottomRight,
            OverlayPosition::BottomRight => OverlayPosition::BottomLeft,
            OverlayPosition::BottomLeft => OverlayPosition::TopLeft,
        };
    }
}
