use bevy::{
    ecs::schedule::{ScheduleLabel, SystemSet},
    prelude::{Res, ResMut, Resource},
};

/// Internal settlement schedule executed from `PostUpdate`.
///
/// The layout plugin drains this schedule until the tracked UI work reaches a
/// fixed point or the configured iteration budget is exhausted.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UiSettlementSchedule;

/// Shared `PostUpdate` schedule sets used across the Univis UI workspace.
///
/// These sets make ordering between widgets, root resolution, layout, and
/// render synchronization explicit and reusable.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UnivisPostUpdateSet {
    /// External systems that prepare layout-affecting state before root resolution.
    ExternalPrepare,
    /// Root resolution, camera binding, and root stacking capsule updates.
    RootResolve,
    /// Hierarchy analysis and cached parent/child relationships.
    LayoutHierarchy,
    /// Intrinsic measurement and fit-content evaluation.
    LayoutMeasure,
    /// Final downward layout solve and transform placement.
    LayoutSolve,
    /// Render-side synchronization after layout is final.
    RenderSync,
    /// External synchronization that runs after solve against settled geometry.
    ExternalPostSolve,
    /// Final bookkeeping after the UI pipeline has finished for the frame.
    UiSettled,
}

/// High-level settlement stages tracked for the current UI mutation generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiWorkStage {
    /// Root resolution stage.
    RootResolve,
    /// Hierarchy analysis stage.
    Hierarchy,
    /// Intrinsic measurement and fit-content evaluation stage.
    Measure,
    /// Final layout downward solving and positioning.
    Solve,
    /// Render-side synchronization stage.
    Render,
}

/// Tracks which high-level settlement stages are pending execution.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UiPendingStages {
    /// True if root resolution is pending.
    pub root_resolve: bool,
    /// True if hierarchy analysis is pending.
    pub hierarchy: bool,
    /// True if intrinsic measurement is pending.
    pub measure: bool,
    /// True if downward layout solving is pending.
    pub solve: bool,
    /// True if render-side synchronization is pending.
    pub render: bool,
}

impl UiPendingStages {
    /// Returns `true` if any stage is pending.
    pub fn any(self) -> bool {
        self.root_resolve || self.hierarchy || self.measure || self.solve || self.render
    }

    /// Merges another `UiPendingStages` into this one, setting each flag to true if it is true in either instance.
    pub fn merge(&mut self, other: Self) {
        self.root_resolve |= other.root_resolve;
        self.hierarchy |= other.hierarchy;
        self.measure |= other.measure;
        self.solve |= other.solve;
        self.render |= other.render;
    }
}

/// Configuration for the bounded UI settlement loop.
#[derive(Resource, Debug, Clone)]
pub struct UiSettlementConfig {
    /// Maximum number of layout iterations allowed per frame.
    pub max_iterations: u32,
}

impl Default for UiSettlementConfig {
    fn default() -> Self {
        Self { max_iterations: 8 }
    }
}

/// Validation mode for rollout shadowing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UiValidationMode {
    /// Validation is disabled.
    #[default]
    Disabled,
    /// Run shadow checks and log warnings when mismatches are detected.
    LogWarnings,
}

/// Rollout switches for the frame-zero pipeline.
///
/// These gates let applications and validation harnesses keep the public API
/// stable while selectively enabling the new settlement behavior.
#[derive(Resource, Debug, Clone)]
pub struct UiRolloutConfig {
    /// Whether to use the cached UI context optimization.
    pub use_cached_ui_context: bool,
    /// Whether to perform incremental measurement (skipping unchanged subtrees).
    pub use_incremental_measure: bool,
    /// Whether to perform incremental layout solving.
    pub use_incremental_solve: bool,
    /// Whether to perform incremental render synchronization.
    pub use_incremental_render: bool,
    /// Whether to use the mesh cache to reuse geometry meshes when possible.
    pub use_mesh_cache: bool,
    /// Whether to enable picking tests after settlement finishes.
    pub use_post_settle_picking: bool,
    /// The validation mode for checking layout mismatches.
    pub validation: UiValidationMode,
}

impl Default for UiRolloutConfig {
    fn default() -> Self {
        Self {
            use_cached_ui_context: true,
            use_incremental_measure: true,
            use_incremental_solve: true,
            // Default off: the incremental render frontier flip-flops in
            // static scenes, redrawing the whole UI tree every other frame
            // (observed as full-panel flicker). Full sync is cheap at common
            // UI scales; revisit once frontier generation bookkeeping is fixed.
            use_incremental_render: false,
            use_mesh_cache: true,
            use_post_settle_picking: true,
            validation: UiValidationMode::Disabled,
        }
    }
}

impl UiRolloutConfig {
    /// Returns whether post-settle picking should emit hits from settled UI geometry.
    pub fn post_settle_picking_enabled(&self) -> bool {
        self.use_post_settle_picking
    }

    /// Returns whether picking-side validation should run for the current rollout.
    pub fn picking_validation_enabled(&self) -> bool {
        self.validation != UiValidationMode::Disabled
    }
}

/// Narrow settlement-facing runtime snapshot derived from the engine work state.
///
/// Examples and diagnostics can depend on this resource without reading the
/// full internal settlement tracker directly.
#[derive(Resource, Debug, Clone)]
pub struct UiSettlementRuntimeState {
    current_generation: u64,
    settled: bool,
    last_frame_iterations: u32,
    budget_exhausted: bool,
}

impl Default for UiSettlementRuntimeState {
    fn default() -> Self {
        Self {
            current_generation: 0,
            settled: true,
            last_frame_iterations: 0,
            budget_exhausted: false,
        }
    }
}

impl UiSettlementRuntimeState {
    /// Returns the latest mutation generation observed by the UI pipeline.
    pub fn current_generation(&self) -> u64 {
        self.current_generation
    }

    /// Returns whether the tracked UI stages are fully settled for the frame.
    pub fn is_settled(&self) -> bool {
        self.settled
    }

    /// Returns the number of settlement iterations executed during the latest
    /// frame.
    pub fn last_frame_iterations(&self) -> u32 {
        self.last_frame_iterations
    }

    /// Returns whether the latest frame exhausted its settlement budget.
    pub fn budget_exhausted(&self) -> bool {
        self.budget_exhausted
    }
}

/// Refreshes the public settlement runtime snapshot from engine-owned work state.
pub fn sync_settlement_runtime_state(
    work_state: Option<Res<UiWorkState>>,
    mut settlement_runtime: ResMut<UiSettlementRuntimeState>,
) {
    let current_generation = work_state
        .as_ref()
        .map_or(0, |value| value.current_generation());
    let settled = work_state.as_ref().map_or(true, |value| value.is_settled());
    let last_frame_iterations = work_state
        .as_ref()
        .map_or(0, |value| value.last_frame_iterations());
    let budget_exhausted = work_state
        .as_ref()
        .is_some_and(|value| value.budget_exhausted());

    *settlement_runtime = UiSettlementRuntimeState {
        current_generation,
        settled,
        last_frame_iterations,
        budget_exhausted,
    };
}

/// Narrow picking-facing runtime snapshot derived from settlement state.
///
/// Interaction systems can depend on this resource without reading the full
/// rollout or work-state resources directly.
#[derive(Resource, Debug, Clone, Default)]
pub struct UiPickingRuntimeState {
    current_ui_generation: u64,
    geometry_pending: bool,
    post_settle_picking_enabled: bool,
    validation_enabled: bool,
}

impl UiPickingRuntimeState {
    /// Returns the latest UI mutation generation observed by the engine.
    pub fn current_ui_generation(&self) -> u64 {
        self.current_ui_generation
    }

    /// Returns whether layout geometry is still settling for the current generation.
    pub fn geometry_pending(&self) -> bool {
        self.geometry_pending
    }

    /// Returns whether settled geometry should emit post-settle pointer hits.
    pub fn post_settle_picking_enabled(&self) -> bool {
        self.post_settle_picking_enabled
    }

    /// Returns whether picking validation is enabled for the current rollout.
    pub fn validation_enabled(&self) -> bool {
        self.validation_enabled
    }
}

/// Refreshes the public picking runtime snapshot from engine-owned rollout and work state.
pub fn sync_picking_runtime_state(
    rollout: Option<Res<UiRolloutConfig>>,
    work_state: Option<Res<UiWorkState>>,
    mut picking_runtime: ResMut<UiPickingRuntimeState>,
) {
    let rollout = rollout
        .as_deref()
        .cloned()
        .unwrap_or_else(UiRolloutConfig::default);
    let current_ui_generation = work_state
        .as_ref()
        .map_or(0, |value| value.current_generation());
    let geometry_pending = work_state
        .as_ref()
        .is_some_and(|value| value.geometry_pending_for_picking());

    *picking_runtime = UiPickingRuntimeState {
        current_ui_generation,
        geometry_pending,
        post_settle_picking_enabled: rollout.post_settle_picking_enabled(),
        validation_enabled: rollout.picking_validation_enabled(),
    };
}

/// Validation diagnostics produced by rollout shadow checks.
#[derive(Resource, Debug, Clone, Default)]
pub struct UiValidationState {
    /// The generation of the UI context that was validated.
    pub cached_context_generation: u64,
    /// The number of mismatches detected during validation.
    pub cached_context_mismatches: usize,
    /// The generation that triggered a warning, if any.
    pub cached_context_warning_generation: Option<u64>,
}

impl UiValidationState {
    /// Records the result of a cached context check.
    pub fn record_cached_context_check(&mut self, generation: u64, mismatches: usize) {
        self.cached_context_generation = generation;
        self.cached_context_mismatches = mismatches;
    }
}

/// Tracks the current UI mutation generation and which settlement stages have
/// fully processed it.
#[derive(Resource, Debug, Default, Clone)]
pub struct UiWorkState {
    current_generation: u64,
    pending: UiPendingStages,
    root_resolve_generation: u64,
    hierarchy_generation: u64,
    measure_generation: u64,
    solve_generation: u64,
    render_generation: u64,
    last_frame_iterations: u32,
    budget_exhausted: bool,
}

impl UiWorkState {
    /// Clears per-frame diagnostics before the settlement loop starts.
    pub fn begin_frame(&mut self) {
        self.last_frame_iterations = 0;
        self.budget_exhausted = false;
    }

    /// Starts a new settlement generation when `pending` contains any work.
    ///
    /// If a previous generation is still unresolved, the newly detected work is
    /// merged into it instead of skipping ahead to a new generation.
    pub fn begin_generation(&mut self, pending: UiPendingStages) -> bool {
        if !pending.any() {
            return false;
        }

        if self.pending.any() {
            self.pending.merge(pending);
            return false;
        }

        self.current_generation += 1;
        self.pending = pending;
        true
    }

    /// Returns the latest mutation generation observed by the UI pipeline.
    pub fn current_generation(&self) -> u64 {
        self.current_generation
    }

    /// Returns the generation fully processed by `stage`.
    pub fn completed_generation(&self, stage: UiWorkStage) -> u64 {
        match stage {
            UiWorkStage::RootResolve => self.root_resolve_generation,
            UiWorkStage::Hierarchy => self.hierarchy_generation,
            UiWorkStage::Measure => self.measure_generation,
            UiWorkStage::Solve => self.solve_generation,
            UiWorkStage::Render => self.render_generation,
        }
    }

    /// Returns the currently pending settlement stages.
    pub fn pending(&self) -> UiPendingStages {
        self.pending
    }

    /// Returns `true` when pre-render geometry stages are still catching up.
    ///
    /// Post-settle picking can run once root resolution, hierarchy, measure,
    /// and solve are complete, even if render synchronization is still pending.
    pub fn geometry_pending_for_picking(&self) -> bool {
        self.pending.root_resolve
            || self.pending.hierarchy
            || self.pending.measure
            || self.pending.solve
    }

    /// Returns the number of settlement iterations executed during the latest
    /// frame.
    pub fn last_frame_iterations(&self) -> u32 {
        self.last_frame_iterations
    }

    /// Returns `true` when the latest frame exhausted the settlement budget
    /// before reaching a verified fixed point.
    pub fn budget_exhausted(&self) -> bool {
        self.budget_exhausted
    }

    /// Returns `true` when the tracked settlement stages have caught up with
    /// the current mutation generation.
    pub fn is_settled(&self) -> bool {
        !self.pending.any() && !self.budget_exhausted
    }

    /// Records one bounded-loop iteration for the current frame.
    pub fn record_iteration(&mut self) {
        self.last_frame_iterations += 1;
    }

    /// Marks the latest frame as budget-exhausted.
    pub fn mark_budget_exhausted(&mut self) {
        self.budget_exhausted = true;
    }

    /// Marks `stage` complete for the current generation.
    pub fn complete_stage(&mut self, stage: UiWorkStage) {
        if self.current_generation == 0 {
            return;
        }

        match stage {
            UiWorkStage::RootResolve => {
                if !self.pending.root_resolve {
                    return;
                }
                self.root_resolve_generation = self.current_generation;
                self.pending.root_resolve = false;
            }
            UiWorkStage::Hierarchy => {
                if !self.pending.hierarchy {
                    return;
                }
                self.hierarchy_generation = self.current_generation;
                self.pending.hierarchy = false;
            }
            UiWorkStage::Measure => {
                if !self.pending.measure {
                    return;
                }
                self.measure_generation = self.current_generation;
                self.pending.measure = false;
            }
            UiWorkStage::Solve => {
                if !self.pending.solve {
                    return;
                }
                self.solve_generation = self.current_generation;
                self.pending.solve = false;
            }
            UiWorkStage::Render => {
                if !self.pending.render {
                    return;
                }
                self.render_generation = self.current_generation;
                self.pending.render = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_settlement_runtime_state_defaults_to_idle() {
        let state = UiSettlementRuntimeState::default();

        assert_eq!(state.current_generation(), 0);
        assert!(state.is_settled());
        assert_eq!(state.last_frame_iterations(), 0);
        assert!(!state.budget_exhausted());
    }

    #[test]
    fn ui_work_state_tracks_stage_completion_per_generation() {
        let mut state = UiWorkState::default();
        let pending = UiPendingStages {
            root_resolve: true,
            hierarchy: true,
            measure: true,
            solve: true,
            render: true,
        };

        assert!(state.begin_generation(pending));
        assert_eq!(state.current_generation(), 1);
        assert!(!state.is_settled());

        state.complete_stage(UiWorkStage::RootResolve);
        state.complete_stage(UiWorkStage::Hierarchy);
        state.complete_stage(UiWorkStage::Measure);
        state.complete_stage(UiWorkStage::Solve);
        state.complete_stage(UiWorkStage::Render);

        assert!(state.is_settled());
        assert_eq!(state.completed_generation(UiWorkStage::Render), 1);
    }

    #[test]
    fn ui_work_state_merges_new_work_into_unfinished_generation() {
        let mut state = UiWorkState::default();

        assert!(state.begin_generation(UiPendingStages {
            hierarchy: true,
            measure: true,
            solve: true,
            render: true,
            ..Default::default()
        }));
        assert_eq!(state.current_generation(), 1);

        assert!(!state.begin_generation(UiPendingStages {
            root_resolve: true,
            ..Default::default()
        }));
        assert_eq!(state.current_generation(), 1);
        assert!(state.pending().root_resolve);
        assert!(state.pending().hierarchy);
    }

    #[test]
    fn ui_work_state_budget_exhaustion_keeps_frame_non_idle() {
        let mut state = UiWorkState::default();

        state.begin_generation(UiPendingStages {
            measure: true,
            ..Default::default()
        });
        state.complete_stage(UiWorkStage::Measure);
        assert!(state.is_settled());

        state.mark_budget_exhausted();
        assert!(!state.is_settled());
        assert!(state.budget_exhausted());

        state.begin_frame();
        assert!(state.is_settled());
        assert!(!state.budget_exhausted());
    }

    #[test]
    fn ui_rollout_config_defaults_enable_the_new_pipeline_without_validation() {
        let config = UiRolloutConfig::default();

        assert!(config.use_cached_ui_context);
        assert!(config.use_incremental_measure);
        assert!(config.use_incremental_solve);
        assert!(config.use_incremental_render);
        assert!(config.use_mesh_cache);
        assert!(config.use_post_settle_picking);
        assert_eq!(config.validation, UiValidationMode::Disabled);
        assert!(config.post_settle_picking_enabled());
        assert!(!config.picking_validation_enabled());
    }

    #[test]
    fn ui_work_state_ignores_empty_generations() {
        let mut state = UiWorkState::default();

        assert!(!state.begin_generation(UiPendingStages::default()));
        assert_eq!(state.current_generation(), 0);
        assert!(state.is_settled());
    }

    #[test]
    fn ui_validation_state_tracks_cached_context_check_results() {
        let mut state = UiValidationState::default();

        state.record_cached_context_check(3, 2);

        assert_eq!(state.cached_context_generation, 3);
        assert_eq!(state.cached_context_mismatches, 2);
    }

    #[test]
    fn ui_work_state_reports_geometry_pending_for_post_settle_picking() {
        let mut state = UiWorkState::default();
        state.begin_generation(UiPendingStages {
            solve: true,
            render: true,
            ..Default::default()
        });

        assert!(state.geometry_pending_for_picking());

        state.complete_stage(UiWorkStage::Solve);
        assert!(!state.geometry_pending_for_picking());
    }
}
