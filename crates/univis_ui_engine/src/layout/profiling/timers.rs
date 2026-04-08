use super::*;

/// Legacy stop/start timer helper for upward pass.
pub fn profile_upward_pass(
    mut profiler: ResMut<LayoutProfiler>,
    mut start_time: Local<Option<Instant>>,
) {
    if start_time.is_none() {
        *start_time = Some(Instant::now());
    } else if let Some(start) = *start_time {
        profiler.upward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
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
    } else if let Some(start) = *start_time {
        profiler.downward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
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
    } else if let Some(start) = *start_time {
        profiler.material_update_time = start.elapsed().as_secs_f64() * 1000.0;
        *start_time = None;
    }
}
