# Frame-Zero Performance Roadmap

## Goal

This roadmap targets two outcomes:

- make Univis UI settle correctly on frame zero
- improve performance without disrupting public behavior or update ordering

In this document, `frame-zero` means:

- if UI entities exist before `app.update()`, then by the end of that same frame the system has valid root resolution, hierarchy depth, intrinsic measurements, solved transforms, render sync state, and initial hit state

In this document, `idle` means:

- a system stage is only idle when it has fully consumed the current UI mutation generation and has no downstream work left queued

## Current Architecture Snapshot

The current high-level stack is:

- facade plugin order: style -> engine -> interaction -> widgets
- widget logic mostly runs in `Update`
- picking currently runs in `PreUpdate`
- layout and render synchronization run in `PostUpdate`

Current strengths:

- the `PostUpdate` layout sets are already explicit and ordered
- the upward measurement pass already has a dirty/cache model
- runtime benchmark harnesses already exist for solver and runtime scenarios

Current frame-zero and performance risks:

1. The depth cache rebuild happens in `Update`, while `LayoutDepth` assignment happens later in `PostUpdate`.
2. The upward pass is dirty-aware, but the downward solve is still effectively full-tree.
3. Picking happens before the layout pipeline settles for the current frame.
4. Several complex widgets still construct runtime child trees in `Update` through command-buffered systems.
5. Render sync still recomputes context per entity and allocates rectangle meshes during sync.

## Target Architecture

The target frame pipeline should be:

1. `PreUpdate`
   - capture raw input only
2. `Update`
   - user/game logic mutates Univis components
   - widget logic mutates widget state, but does not leave hidden downstream work untracked
3. `PostUpdate`
   - `WidgetBuild`
   - `RootResolve`
   - `HierarchyRefresh`
   - `DirtyPropagate`
   - `Measure`
   - `Solve`
   - `RenderSync`
   - `PickSync`
   - `UiSettled`

The target completion model should be:

- a shared `UiWorkState` resource tracks the current mutation generation
- every mutating system increments the generation and marks downstream work
- each stage stamps the generation it has fully processed
- a stage is not idle until its processed generation matches the world generation and its work queue is empty
- if work remains after the configured settle loop limit, the UI is reported as not settled instead of silently rolling work into later frames

## Staged Roadmap

### Phase 0: Instrumentation and Invariants

Intent:

- establish proof for frame-zero correctness before changing architecture

Deliverables:

- add frame-zero benchmark scenarios to the existing runtime harness
- add first-frame regression tests for representative roots and widgets
- add settle diagnostics to the profiler

Work:

- add `frame_zero_bootstrap` runtime benchmark
- add `frame_zero_widget_spawn` runtime benchmark
- add `frame_zero_picking` runtime benchmark
- add `idle_drain_count` and `settle_iterations` metrics
- add tests that assert first-frame:
  - root resolution is valid
  - `LayoutDepth` exists
  - `ComputedSize` is non-zero where expected
  - render handles/materials are attached where expected
  - picking returns expected hits for ready UI

Acceptance criteria:

- benchmarks run through the existing perf script
- representative first-frame tests fail on regressions
- no behavior change yet

### Phase 1: Fix the First-Frame Layout Bootstrap

Intent:

- remove the current architectural split that can defer real layout work to the second frame

Deliverables:

- unify hierarchy refresh and depth-cache construction inside the main UI pipeline
- move dirty propagation into the same frame as layout settlement

Work:

- merge `update_layout_hierarchy` and depth-cache rebuild into one `PostUpdate` phase
- stop depending on `Update`-time `Added<LayoutDepth>` to prepare the current frame's measurement/solve work
- move `track_layout_changes` out of `Update` and into the ordered UI pipeline
- ensure the hierarchy/depth pass produces data that `Measure` and `Solve` can use immediately in the same frame

Acceptance criteria:

- a freshly spawned root tree measures and solves on the first frame
- no required second-frame bootstrap remains for layout depth indexing

### Phase 2: Introduce Explicit Settle and Idle Semantics

Intent:

- make "idle" a real contract rather than an inference from empty queries

Deliverables:

- stage-level work tracking
- bounded settle loop
- clear diagnostics when the UI does not settle

Work:

- add `UiWorkState` or equivalent generation-tracking resource
- add tracked downstream work categories:
  - `NeedsBuild`
  - `NeedsHierarchy`
  - `NeedsMeasure`
  - `NeedsSolve`
  - `NeedsRenderSync`
  - `NeedsPickSync`
- add a bounded settle loop inside the UI pipeline
- make the settle loop drain until queues are empty or the iteration budget is exhausted
- surface warnings/diagnostics if the settle budget is exhausted

Acceptance criteria:

- the engine can report whether the UI settled in the current frame
- systems are only considered idle after downstream work is drained

### Phase 3: Re-architect Widget Runtime Phases

Intent:

- stop leaving widget initialization and child-tree construction scattered across general `Update`

Deliverables:

- dedicated widget subphases
- clearer separation of runtime build vs logic vs visuals vs events

Work:

- create `WidgetBuild`, `WidgetLogic`, `WidgetVisual`, and `WidgetEvents` sets
- migrate `Added<T>` widget child-tree construction into `WidgetBuild`
- keep pure state mutations in `WidgetLogic`
- keep visual-only sync in `WidgetVisual`
- keep message emission in `WidgetEvents`
- migrate complex widgets first:
  - `USelect`
  - `UTextField`
  - `UPanelWindow`

Acceptance criteria:

- a new complex widget is structurally complete and measurable in one frame
- widget systems no longer rely on implicit multi-frame follow-up to finish setup

### Phase 4: Make Layout Incremental End-to-End

Intent:

- align solve cost with the actual dirty area instead of the whole tree

Deliverables:

- subtree-scoped solve
- cheaper per-frame traversal

Work:

- keep dirty-aware upward measurement
- stop treating downward solve as full-tree by default
- solve only dirty roots or dirty subtrees
- preserve dirty frontier until solve completes
- cache root context needed by solve instead of repeatedly walking parent chains
- avoid unnecessary cloning/temporary allocations in hot layout paths where possible

Acceptance criteria:

- isolated subtree edits scale with subtree size
- benchmark wins appear in widget-heavy and root-heavy scenes

### Phase 5: Reduce Render Sync Overhead

Intent:

- remove avoidable per-entity work in the render phase

Deliverables:

- reusable render context data
- mesh reuse or mesh caching

Work:

- cache resolved root/clip context once per entity or subtree
- reuse that context in render and picking
- replace per-sync rectangle mesh allocation with:
  - a reusable unit quad plus scale, or
  - a mesh cache keyed by size if transform-only scaling is not viable
- add mesh reuse stats alongside material reuse stats

Acceptance criteria:

- render sync stops allocating a new mesh for every eligible size change by default
- context walks are reduced measurably in profiling

### Phase 6: Add After-Settle Picking

Intent:

- make hit testing reflect settled current-frame geometry

Deliverables:

- `PickSync` stage after solve/render
- compatibility-preserving rollout for input behavior

Work:

- keep current `PreUpdate` picking initially for compatibility
- add a post-settle picking refresh when UI generation or pointer generation changes
- cache root ancestry and clip ancestry used by hit tests
- build root-local interaction indices so picking is not forced into repeated full scans forever
- validate parity before reducing `PreUpdate` picking responsibility

Acceptance criteria:

- first-frame and mutation-frame picking can resolve against settled geometry
- no unexpected input behavior regressions during rollout

### Phase 7: Rollout, Shadowing, and Safety Gates

Intent:

- land the architecture change without destabilizing behavior

Deliverables:

- gated rollout plan
- side-by-side validation

Work:

- hide new pipeline behavior behind a feature flag or internal config switch first
- run old and new geometry/hit computations side by side in debug or validation mode
- compare representative examples for:
  - layout
  - picking
  - text widgets
  - panel windows
  - world-space roots
- gate rollout on:
  - unit/integration tests
  - representative example checks
  - runtime benchmark budgets

Acceptance criteria:

- no public API break is required for the first rollout
- regressions are caught before the new path becomes default

## Implementation Tickets

### Ticket 1: Add Frame-Zero Benchmark Scenarios

Scope:

- extend the runtime benchmark harness with frame-zero-specific scenarios and settle metrics

Files likely involved:

- `crates/univis_ui_engine/examples/runtime_benchmarks.rs`
- `scripts/run_perf_baselines.sh`

Definition of done:

- new scenarios run from the existing perf script
- output includes settle metrics

### Ticket 2: Add First-Frame Integration Tests

Scope:

- add tests for first-frame root settlement, widget settlement, and picking settlement

Files likely involved:

- `crates/univis_ui_engine/src/layout/mod.rs`
- `crates/univis_ui_engine/src/layout/layout_system/`
- `crates/univis_ui_interaction/src/interaction/picking.rs`
- widget modules with runtime child construction

Definition of done:

- representative first-frame tests pass
- tests fail if settlement requires an extra frame

### Ticket 3: Unify Hierarchy Refresh and Depth Cache

Scope:

- remove the split between hierarchy marking and depth-cache readiness

Files likely involved:

- `crates/univis_ui_engine/src/layout/core/hierarchy.rs`
- `crates/univis_ui_engine/src/layout/core/layout_cache.rs`
- `crates/univis_ui_engine/src/layout/mod.rs`

Definition of done:

- current-frame hierarchy output is immediately usable by measure/solve

### Ticket 4: Move Dirty Propagation Into the Ordered UI Pipeline

Scope:

- stop relying on `Update`-time dirty propagation for `PostUpdate` settlement

Files likely involved:

- `crates/univis_ui_engine/src/layout/core/layout_cache.rs`
- `crates/univis_ui_engine/src/layout/mod.rs`

Definition of done:

- dirty propagation sees current-frame widget and layout changes

### Ticket 5: Add `UiWorkState` and Generation Tracking

Scope:

- introduce explicit work and completion tracking for UI settlement

Files likely involved:

- new resource/module under `crates/univis_ui_engine/src/layout/`
- `crates/univis_ui_engine/src/layout/mod.rs`
- `crates/univis_ui_engine/src/layout/profiling.rs`

Definition of done:

- each stage can report whether it is caught up with the current mutation generation

### Ticket 6: Add Bounded Settle Loop

Scope:

- drain queued UI work inside the current frame while preserving deterministic limits

Files likely involved:

- `crates/univis_ui_engine/src/layout/mod.rs`
- stage systems that participate in settlement

Definition of done:

- settlement drains in-frame for normal cases
- over-budget cases are visible and diagnosable

### Ticket 7: Introduce Widget Subphase Sets

Scope:

- separate widget build, logic, visuals, and events

Files likely involved:

- `crates/univis_ui_widgets/src/widget/mod.rs`
- `crates/univis_ui_widgets/src/widget/select/`
- `crates/univis_ui_widgets/src/widget/text_field.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`

Definition of done:

- widget runtime work is scheduled explicitly and predictably

### Ticket 8: Migrate `USelect` to `WidgetBuild` / `WidgetLogic` / `WidgetVisual`

Scope:

- make `USelect` the first complex widget migrated to the new scheduling model

Definition of done:

- first-frame select build is complete
- select dropdown/runtime behavior matches old behavior

### Ticket 9: Migrate `UTextField` to the New Widget Phases

Scope:

- move text field build and cursor/text runtime updates into explicit phases

Definition of done:

- text field child structure and visuals settle in one frame

### Ticket 10: Migrate `UPanelWindow` Runtime Handle Setup

Scope:

- move resize-handle initialization and related runtime sync into explicit widget phases

Definition of done:

- resize handles exist and are sized correctly on the first frame

### Ticket 11: Make Downward Solve Dirty-Scoped

Scope:

- reduce solve cost from full-tree to dirty-root or dirty-subtree traversal

Files likely involved:

- `crates/univis_ui_engine/src/layout/core/pass_down.rs`
- related cache/work-state modules

Definition of done:

- unchanged subtrees are not solved unnecessarily

### Ticket 12: Cache Root and Clip Context for Solve/Render/Picking

Scope:

- avoid repeated ancestry walks in hot paths

Files likely involved:

- `crates/univis_ui_engine/src/layout/render/system.rs`
- `crates/univis_ui_interaction/src/interaction/picking.rs`
- new shared cached-context module if needed

Definition of done:

- root/clip ancestry work is amortized instead of recomputed repeatedly

### Ticket 13: Add Mesh Reuse Strategy

Scope:

- remove default per-sync rectangle mesh allocation behavior

Files likely involved:

- `crates/univis_ui_engine/src/layout/render/system.rs`

Definition of done:

- mesh reuse stats exist
- benchmarked sync path shows fewer allocations

### Ticket 14: Add Post-Settle Picking Sync

Scope:

- resolve picking against settled current-frame geometry without immediately removing compatibility paths

Files likely involved:

- `crates/univis_ui_interaction/src/interaction/mod.rs`
- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_engine/src/layout/mod.rs`

Definition of done:

- picking can refresh after layout settlement in the same frame

### Ticket 15: Add Shadow Validation Mode

Scope:

- compare old and new paths safely during rollout

Files likely involved:

- validation/debug-only modules around layout and picking
- representative example validation scripts if needed

Definition of done:

- mismatches are reported before the new path becomes default

## Recommended Execution Order

Implement in this order:

1. Ticket 1
2. Ticket 2
3. Ticket 3
4. Ticket 4
5. Ticket 5
6. Ticket 6
7. Ticket 7
8. Ticket 8
9. Ticket 9
10. Ticket 10
11. Ticket 11
12. Ticket 12
13. Ticket 13
14. Ticket 14
15. Ticket 15

This order fixes the first-frame architectural gap first, then adds explicit completion semantics, then reworks widgets, and only after that optimizes render and picking internals.
