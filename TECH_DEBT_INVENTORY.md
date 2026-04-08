# Technical Debt Inventory

This file started as the maintainer-facing backlog for the large-file refactor wave.

Current status:

- completed on `2026-04-04`
- the four target areas were moved into dedicated folders with `mod.rs` entry points
- validation passed with:
  - `cargo check --workspace`
  - `./scripts/check_quality.sh`
  - `./scripts/check_representative_examples.sh`

It focused on the four highest-impact files identified during the post-review pass:

- `crates/univis_ui_widgets/src/widget/text_label.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/layout/core/solver.rs`
- `crates/univis_ui_widgets/src/widget/select.rs`

The goal was not to redesign behavior. The goal was to reduce maintenance cost, isolate risk, and make future bugs easier to diagnose.

## Delivered Structure

### `text_label`

- current entry point: `crates/univis_ui_widgets/src/widget/text_label/mod.rs`
- delivered split:
  - `model.rs`
  - `measure.rs`
  - `render.rs`

### `layout_system`

- current entry point: `crates/univis_ui_engine/src/layout/layout_system/mod.rs`
- delivered split:
  - `types.rs`
  - `root_stacking.rs`
  - `ui3d_sync.rs`
  - existing `root_resolution.rs`
  - existing `screen_transform.rs`

### `solver`

- current entry point: `crates/univis_ui_engine/src/layout/core/solver/mod.rs`
- delivered split:
  - `types.rs`
  - `helpers.rs`
  - `translate.rs`
  - `absolute.rs`

### `select`

- current entry point: `crates/univis_ui_widgets/src/widget/select/mod.rs`
- delivered split:
  - `model.rs`
  - `runtime.rs`
  - `interaction.rs`
  - `visuals.rs`
  - `events.rs`

## Refactor Principles

- preserve public behavior unless a bug fix is intentional
- prefer module extraction over semantic rewrites
- keep tests close to the logic they verify when possible
- give each major model / feature family its own folder so related files stay grouped
- avoid dumping many sibling files into one already-busy parent directory
- validate each refactor with:
  - `./scripts/check_quality.sh`
  - `./scripts/check_representative_examples.sh`
  - `mdbook build docs` when public docs or examples changed

## Priority Order

1. `text_label.rs`
2. `layout_system.rs`
3. `solver.rs`
4. `select.rs`

This order is based on cross-cutting risk, file size, and how often future feature work is likely to touch the file.

## 1) `text_label.rs`

Path:

- `crates/univis_ui_widgets/src/widget/text_label.rs`

Why it is high priority:

- it mixes public widget API, text measurement, bidi/truncation logic, glyph atlas generation, SDF image construction, mesh building, clip handling, runtime sync, and tests in one file
- the file is large enough that rendering bugs, autosize bugs, and clipping bugs are difficult to isolate quickly
- the widget touches both layout behavior and visual output, so regressions are costly

Current responsibility clusters already visible in the file:

- widget types and layout cache
- text measurement and bounds resolution
- bidi-aware truncation and ellipsis logic
- SDF glyph/image generation
- mesh/material building
- clip/material sync
- dirty tracking
- plugin wiring
- tests

Recommended target split:

- `widget/text_label/`
  - `mod.rs`
  - `model.rs`
  - `measure.rs`
  - `truncation.rs`
  - `sdf.rs`
  - `mesh.rs`
  - `clip.rs`
  - `runtime.rs`
  - `tests.rs`

Refactor notes:

- keep public types such as `UTextLabel`, `UTextOverflow`, and `UTextTruncateSide` easy to discover from the top-level module
- avoid changing measurement semantics and ellipsis behavior in the same step as file extraction
- preserve the current test coverage around grapheme truncation, clipping, and autosize

Primary risk if left untouched:

- future fixes in text rendering will keep coupling layout, rendering, and bidi logic together, slowing reviews and raising regression risk

## 2) `layout_system.rs`

Path:

- `crates/univis_ui_engine/src/layout/layout_system.rs`

Why it is high priority:

- it combines public root API types, legacy compatibility wrappers, resolved runtime state, root stacking math, cached `UI3d` sync, and a large test block
- the file sits on a critical path for roots, world-vs-screen behavior, and cross-root ordering
- small root-behavior edits become harder to reason about when state definitions and runtime systems live together

Current responsibility clusters already visible in the file:

- root public enums and components
- `URootUi` constructors and defaults
- resolved runtime state
- legacy wrappers
- root capsule stacking helpers
- cached `UI3d` synchronization
- tests

Recommended target split:

- `layout/layout_system/`
  - `mod.rs`
  - `types.rs`
  - `runtime_state.rs`
  - `legacy_wrappers.rs`
  - `root_stacking.rs`
  - `ui3d_sync.rs`
  - existing `root_resolution.rs`
  - existing `screen_transform.rs`
  - `tests.rs`

Refactor notes:

- keep `URootUi`, `UiSpace`, `UiCanvasSize`, and `UiCameraRef` easy to find from one public entry module
- do not mix legacy-wrapper cleanup with root-stacking behavior changes
- treat `root_resolution.rs` and `screen_transform.rs` as the extraction style to continue

Primary risk if left untouched:

- root-behavior regressions will remain expensive to audit because type definitions and runtime mutation paths are still too concentrated

## 3) `solver.rs`

Path:

- `crates/univis_ui_engine/src/layout/core/solver.rs`

Why it is high priority:

- it concentrates sizing helpers, flex/grid decisions, explicit-vs-implicit stretch behavior, absolute positioning, and translation from `UNode` to solver specs
- it is one of the most sensitive files for layout correctness and future performance work
- performance tuning will be harder while the core layout flow remains in one file

Current responsibility clusters already visible in the file:

- solver data structures
- alignment mapping helpers
- size bound helpers
- main solve routine
- `translate_spec`
- absolute box solving
- tests

Recommended target split:

- `layout/core/solver/`
  - `mod.rs`
  - `types.rs`
  - `helpers.rs`
  - `flex.rs`
  - `grid.rs`
  - `alignment.rs`
  - `absolute.rs`
  - `translate.rs`
  - `tests.rs`

Refactor notes:

- keep the public `solve_flex_layout` entry point stable while moving helpers underneath it
- if grid behavior is still routed through shared flow, split by responsibility first and optimize later
- do not pair structural extraction with new layout semantics in the same pass

Primary risk if left untouched:

- any future min/max, intrinsic sizing, or wrap fix will continue to land inside a single high-pressure file with too many reasons to change

## 4) `select.rs`

Path:

- `crates/univis_ui_widgets/src/widget/select.rs`

Why it is high priority:

- it mixes public widget API, runtime entity graph management, trigger/option interaction, keyboard navigation, visuals, event emission, helper utilities, and tests
- select widgets are behavior-heavy and easy to regress when visuals and input paths evolve together
- this file is likely to grow again if forms/widgets expand

Current responsibility clusters already visible in the file:

- public plugin and widget model
- runtime marker components
- invariant enforcement
- trigger interaction
- option interaction
- keyboard handling
- outside-click close behavior
- dropdown tree sync and spawning
- visual updates
- event emission
- tests

Recommended target split:

- `widget/select/`
  - `mod.rs`
  - `model.rs`
  - `runtime.rs`
  - `tree.rs`
  - `interaction.rs`
  - `keyboard.rs`
  - `visuals.rs`
  - `events.rs`
  - `tests.rs`

Refactor notes:

- keep `USelect` and `USelectOption` near the top-level public module
- avoid changing keyboard behavior while moving tree-sync logic
- keep helper functions such as `sanitize_select` and enabled-index traversal close to model invariants

Primary risk if left untouched:

- future behavior fixes in forms/widgets will keep touching one file that mixes state invariants with rendering tree concerns

## Suggested Execution Order

Recommended order:

1. extract `text_label.rs` into submodules without semantic changes
2. finish splitting `layout_system.rs` around types, stacking, and ui3d sync
3. separate solver helpers/types from the main solve routine
4. split `select.rs` once the earlier engine-facing work is stable

Recommended PR shape:

- one file family per PR
- one behavior-preserving extraction goal per PR
- add or refresh tests before changing module layout if the current area is under-tested
- when an area is split, create one dedicated folder for it instead of scattering many files into the parent folder

## Exit Criteria For This Inventory

This inventory can be considered complete when:

- every target file has a documented reason to change
- every target file has a proposed module split
- the refactor order is explicit
- validation expectations are named before follow-up execution begins
