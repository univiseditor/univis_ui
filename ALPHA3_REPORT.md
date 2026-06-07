# Alpha 3 Report

## Scope

- This report compares the released `0.3.0-alpha.2` baseline to the current `0.3.0` preparation state.
- It focuses on engineering quality, structure, public API clarity, documentation, and release validation.
- It does not try to restate every historical `alpha2` migration detail that is already captured in `changelog.md`.

## Executive Summary

- `alpha3` is not a feature-expansion release.
- `alpha3` is a cleanup, curation, and maintainability release on top of the `alpha2` public baseline.
- The root model, crate layout, and example ownership story from `alpha2` stay in place.
- The biggest changes are:
  - a narrower and clearer recommended import story
  - decomposition of the largest internal hotspots
  - stronger release guardrails
  - typed diagnostics in high-value runtime paths
  - clearer maintainer documentation and compatibility policy

## What Was Added

### Root-Level Release And Quality Files

- `ALPHA3_REPORT.md`

### New Validation And Guardrail Scripts

- `scripts/check_public_api_surface.sh`
- `scripts/check_structure_guardrails.sh`

These additions make `alpha3` validate more than formatting/tests/examples. They also validate:

- the recommended prelude surface
- deprecated-wrapper isolation
- source hotspot growth limits
- accidental wildcard-internal spread
- forbidden `Result<_, ()>` regressions

### New Maintainer Docs

English and Arabic both gained new pages for:

- crate selection
- architecture rules
- error model
- layout-engine boundaries
- maintainer map
- naming conventions
- widget structure
- legacy compatibility status

Concrete additions include:

- `docs/src/en/api/crate-map.md`
- `docs/src/ar/api/crate-map.md`
- `docs/src/en/development/architecture-rules.md`
- `docs/src/ar/development/architecture-rules.md`
- `docs/src/en/development/error-model.md`
- `docs/src/ar/development/error-model.md`
- `docs/src/en/development/widget-structure.md`
- `docs/src/ar/development/widget-structure.md`
- `docs/src/en/migration/legacy-compatibility.md`
- `docs/src/ar/migration/legacy-compatibility.md`

### New Module Trees

The release adds focused module trees that did not exist in the `alpha2` baseline:

- engine layout split helpers:
  - `layout/plugin.rs`
  - `layout/registration.rs`
  - `layout/settlement_loop.rs`
  - `layout/tests.rs`
- root-system split helpers:
  - `layout/layout_system/roots.rs`
  - `layout/layout_system/legacy_compat.rs`
- profiling split helpers:
  - `layout/profiling/state.rs`
  - `layout/profiling/overlay.rs`
  - `layout/profiling/systems.rs`
  - `layout/profiling/timers.rs`
- picking split helpers:
  - `interaction/picking/backend.rs`
  - `interaction/picking/clip.rs`
  - `interaction/picking/hit_math.rs`
  - `interaction/picking/root_lookup.rs`
  - `interaction/picking/validation.rs`
  - `interaction/picking/tests.rs`
- widget split helpers:
  - `widget/panel/*`
  - `widget/radio/*`
  - `widget/text_field/*`
  - `widget/text_label/measure/*`
  - `widget/text_label/render/*`

## What Changed

### 1. Public API And Prelude Story

`alpha2` already had the right high-level crate architecture, but the recommended import story was still broader than it needed to be.

In `alpha3`:

- the facade still offers one main plugin and one main prelude
- deprecated wrappers are now explicitly documented as explicit-path-only migration tools
- the engine prelude no longer treats deprecated wrappers as part of the recommended default story
- new API validation scripts compile positive and negative cases to keep this contract stable

Key references:

- `src/lib.rs`
- `crates/univis_ui_engine/src/lib.rs`
- `docs/src/en/api/crate-map.md`
- `scripts/check_public_api_surface.sh`

### 2. Engine Structure

The engine saw the biggest structural cleanup.

Large responsibilities that were previously concentrated in very large files were split into focused pieces:

- layout plugin wiring
- registration
- settlement orchestration
- root types and legacy translation
- placement helpers/tests
- profiling state/overlay/systems/timers
- layout-cache invalidation/tests

This did not change the top-level crate story, but it changed reviewability and ownership a lot.

### 3. Interaction Structure

The picking backend is no longer one giant file carrying all of:

- hit math
- clip checks
- root lookup
- backend emission
- rollout validation
- tests

Instead, `alpha3` keeps `picking.rs` as the entry module and pushes details into focused neighbors.

### 4. Widget Structure

Several stateful widgets now follow clearer internal patterns:

- `panel` split into model/cursor/math/resize/visuals/tests
- `radio` split into model/runtime/visuals/events
- `text_field` split into model/runtime/visuals/events/tests
- `text_label` split more deeply into measurement/render subtrees

This makes the widget layer more uniform and closer to a documented convention rather than a collection of one-off implementations.

### 5. Error Handling

`alpha2` still had some opaque runtime failure paths.

`alpha3` improves this by introducing typed local diagnostics in hot-value areas such as:

- `TextMeasureError` for text measurement
- `SelectRuntimeError` for generated select child-tree issues

It also documents the warning shape and when typed errors should exist.

### 6. Release Discipline

Release preparation in `alpha3` now includes:

- quality gates
- public API surface checks
- structure guardrails
- mdBook build
- library tests
- package-aware example validation

This is a meaningful difference from `alpha2`: the release process now protects architecture quality directly.

## What Was Removed Or Replaced

### Removed Files

- `crates/univis_ui_engine/src/layout/algorithms/places/display.rs`
- `crates/univis_ui_engine/src/layout/layout_system/types.rs`

### Replaced By

- `display.rs` was replaced by `placement.rs` plus focused helpers/tests
- `types.rs` was replaced by `roots.rs` and `legacy_compat.rs`

### Practical Meaning

- less “everything file” pressure
- clearer reasons to change per file
- easier localized testing

## Before / After Snapshot

| Area | `alpha2` / pre-cleanup baseline | `alpha3` prep state |
|---|---|---|
| Facade import story | recommended surface still leaked more migration detail than ideal | curated import story, deprecated wrappers documented as explicit-only |
| Public API validation | mostly docs/tests/examples based | dedicated public API compile checks |
| Structure guardrails | no dedicated LOC/API guard scripts | LOC/API/result-type guardrails added |
| Engine layout entry | `layout/mod.rs` was a major hotspot (`1247` LOC baseline) | `layout/mod.rs` is now a small entry module (`56` LOC) |
| Picking entry | `interaction/picking.rs` was `1355` LOC | `interaction/picking.rs` is now `347` LOC plus focused submodules |
| Text measurement entry | `text_label/measure.rs` was `1133` LOC | `text_label/measure.rs` is now `244` LOC plus focused submodules |
| Panel entry | `widget/panel.rs` was `997` LOC | `widget/panel.rs` is now `70` LOC plus focused submodules |
| Text field entry | `widget/text_field.rs` was `619` LOC | `widget/text_field.rs` is now `61` LOC plus focused submodules |
| Radio entry | `widget/radio.rs` was `611` LOC | `widget/radio.rs` is now `41` LOC plus focused submodules |
| Error model | opaque runtime failure paths still existed in key areas | typed diagnostics documented and used in key hotspots |

## What Was Added For Maintainers Specifically

- a written crate-selection story
- a written architecture rule set
- a written error-model policy
- a written widget-structure convention
- a written legacy compatibility policy

This is one of the biggest `alpha3` upgrades even though it is not a “feature” in the usual sense.

## Recommended Alpha 3 Positioning

Use this release story:

- `alpha2` established the new public baseline
- `alpha3` hardens that baseline
- users upgrading from `alpha2` should expect a cleaner default import story, stronger compatibility guidance, and a more maintainable internal codebase
- maintainers should expect clearer module ownership, better diagnostics, and stronger release automation

## Verification Snapshot

The current `alpha3` prep state was checked with:

- `cargo fmt --all --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace --lib`
- `mdbook build docs`
- `./scripts/check_structure_guardrails.sh`
- `CARGO_NET_OFFLINE=true ./scripts/check_public_api_surface.sh`

## Short Release Summary

If you need one short sentence for the team:

`0.3.0` keeps the `alpha2` public model, but makes the library significantly cleaner to consume, maintain, and validate.
