# Changelog

All notable changes to this project will be documented in this file.

This changelog is intentionally concise.
For deeper release context, see:
- `RELEASE_NOTES.md`

Use this file for the dated chronological record.
Use `RELEASE_NOTES.md` for the current alpha release summary.
Use `MIGRATION.md` / `MIGRATION_AR.md` for upgrade guidance.

## [2026-05-18]

### Added

- Added `RELEASE_PREP_0.3.0.md` to record local verification evidence for the next `0.3.0` stabilization cut.

### Changed

- Updated the `0.3.0` roadmap and bilingual release-readiness docs with the current local gate status, while keeping CI confirmation, manual visual validation, and RC feedback closure explicit as pending release-process work.
- Documented the local ignored-lockfile refresh from yanked `fastrand 2.4.0` to `fastrand 2.4.1`, then reran the full local release rehearsal successfully.

## [2026-04-19]

### Added

- Added a live widget example set under `examples/widgets/` with explicit Cargo targets for `widgets_controls`, `widgets_inputs`, `widgets_display`, and `widgets_containers`.

### Changed

- Changed the examples catalog and widget docs in both English and Arabic so widget-related pages point to current live examples.

## [2026-04-08]

### Added

- Added public-surface and structure guardrail scripts, `scripts/check_public_api_surface.sh` and `scripts/check_structure_guardrails.sh`, so release validation now checks recommended imports, deprecated-wrapper exposure, file-size ceilings, wildcard-internal usage, and forbidden `Result<_, ()>` regressions.
- Added maintainer-oriented docs pages in both English and Arabic covering crate selection, architecture rules, error modeling, naming conventions, widget structure, layout-engine boundaries, maintainer ownership, and legacy-compatibility policy.
- Added focused engine submodules for layout plugin wiring, registration, settlement orchestration, profiling state/overlay/timers, root-model splitting, layout-cache invalidation/tests, and placement tests instead of keeping those concerns in a handful of oversized files.
- Added focused interaction/widget submodules for picking backend internals, panel runtime/visual/math paths, radio internals, text-field internals, and `text_label` measurement/render responsibilities.
- Added `ALPHA3_REPORT.md` as the root-level `alpha3` comparison report against the released `alpha2` baseline.

### Changed

- Changed the facade and engine import story so the recommended preludes stay centered on canonical APIs, while deprecated root wrappers remain available only on explicit paths.
- Changed the largest implementation hotspots across engine, interaction, and widgets from monolithic source files into responsibility-led module trees with tests kept close to the logic they verify.
- Changed text measurement and select runtime setup to use typed internal errors plus structured warnings instead of opaque `Result<_, ()>`-style failure paths.
- Changed the docs, migration notes, root landing pages, and compatibility wording so `alpha3` consistently describes the canonical API story, the explicit-only wrapper policy, and the next removal window for deprecated roots.
- Changed CI and review expectations so quality gates, public API checks, structure guardrails, docs updates, and package-aware example validation are all part of the standard release path.
- Changed the root crate and workspace crate versions from `0.3.0-alpha.2` to `0.3.0`.

### Removed

- Removed the old monolithic `crates/univis_ui_engine/src/layout/algorithms/places/display.rs` in favor of `placement.rs` plus focused helpers/tests.
- Removed the old monolithic `crates/univis_ui_engine/src/layout/layout_system/types.rs` in favor of the focused `roots.rs` and `legacy_compat.rs` split.

## [0.3.0] - 2026-04-08

### Changed

- Bumped the facade crate and all public workspace crates from `0.3.0-alpha.2` to `0.3.0`.
- Promoted the post-`alpha2` engineering-quality wave into the current alpha release, focusing on API curation, maintainability, diagnostics, and release-discipline rather than on introducing a new feature family.
- Finalized the current explicit-only compatibility story for `UScreenRoot` and `UWorldRoot` during `alpha3`, while deferring the actual removal decision to the first alpha after `0.3.0` if migration feedback stays clean.

### Notes

- `alpha3` keeps the `URootUi`-centered public model from `alpha2` intact while making the implementation and the release process substantially cleaner.
- The detailed diff between the released `alpha2` baseline and the current `alpha3` prep tree is summarized in `ALPHA3_REPORT.md`.

## [2026-04-06]

### Added

- Added `examples/visual_regression_lab.rs` as a focused visual regression scene that stresses nested cards, dynamic text, select/toggle controls, meters, and width changes in one place.
- Added per-node stage generations and per-root settlement state so measure, solve, and render completion can be tracked explicitly instead of inferred from broad dirty scans.
- Added reusable layout scratch instrumentation and picking candidate counters to the profiler overlay so scratch growth, peak queue sizes, and prepared picking buckets are visible during debugging.
- Added regression coverage for settled-node stage generations, full-scan rollout fallback paths, autosize text stabilization, child-size propagation, and cached-camera picking refreshes so rollout validation exercises the current incremental settlement rules directly.

### Changed

- Changed layout invalidation and settlement to operate through scoped measure/solve/render frontiers, per-root generations, and dependency-aware propagation instead of repeatedly walking unchanged subtrees.
- Changed the upward and downward layout passes to reuse scratch buffers, avoid rebuilding temporary child vectors and solver-ref vectors per container, and keep solve ordering tied to the dirty frontier.
- Changed picking to prepare camera-scoped candidate buckets before pointer evaluation, reducing repeated root-resolution and candidate-preparation work for the same frame.
- Changed text measurement to cache parent bounds alongside label metrics so constrained labels can detect width and height changes coming from their container, not only from direct label mutations.
- Changed cached root-context refresh to stay scoped to real descendant-facing root, clip, and hierarchy deltas, so root-stack write noise and child/clip removals no longer force unnecessary broad refreshes.
- Changed `UiRolloutConfig` to gate incremental measure and render frontiers alongside solve, keeping the full-scan fallback paths available while rollout parity is still being verified.
- Changed the maintainer docs, release-readiness notes, migration docs, and root notes to point at the current incremental-settlement rollout and verification gate.

### Fixed

- Fixed write-noise across root stacking, solve output, and text measurement so unchanged values no longer trigger broad downstream work through avoidable component rewrites.
- Fixed a depth-order regression where tiny root-capsule `z` steps could be skipped by epsilon checks, causing transient visual stacking glitches even when size and position stayed correct.
- Fixed nested container settlement after root canvas changes so child containers are requeued with the active solve generation instead of being left on stale solve state.
- Fixed `UTextLabel` remeasurement under changing parent bounds, resolving visual mix-ups where select rows, toggle rows, and clipped labels could keep stale truncation or stale inner widths.
- Fixed incremental text-label invalidation so same-size text edits no longer dirty parent solve work, while real intrinsic-size deltas still queue one deduped solve pass for the affected label and ancestor chain.
- Fixed post-settle picking cache refresh after root camera changes so descendants pick against the updated root context instead of keeping stale cached camera ancestry.
- Fixed startup ordering in several widgets so child entities are spawned through the same chained parent command path, preventing hierarchy warnings caused by partially initialized parent entities.
- Fixed the benchmark/example compatibility fallout from the solver scratch refactor by updating the solver benchmark harness, the complex dashboard example, and the representative rollout checks to the current pipeline.

## [2026-04-05]

### Added

- Added a frozen performance baseline capture under `perf_baselines/current_max/2026-04-05/` with human-readable notes, a machine-readable manifest, and raw solver/runtime benchmark outputs for the pre-optimization maximum built-in load.
- Added explicit rollout controls and validation state through `UiRolloutConfig`, `UiValidationMode`, `UiValidationState`, and the interaction-owned `PickingValidationState`, so cached ancestry, incremental solve, mesh reuse, and post-settle picking can be validated before defaults are tightened.
- Added `scripts/verify_frame_zero_rollout.sh` to run the frame-zero rollout gate in one command across library tests, representative release-mode examples, and benchmark budget checks.
- Added a stress-only runtime benchmark workload, `roots_10k_nodes_1m`, covering `10,000` roots and `1,000,000` nodes without folding that load into the default baseline pass.

### Changed

- Reworked the UI update architecture around an explicit bounded settlement loop in `PostUpdate`, with tracked work generations and verified idle semantics instead of inferring settlement from empty queries.
- Moved hierarchy refresh, cached ancestry sync, depth-cache rebuild, dirty propagation, measurement, solve, render sync, and post-settle picking into one ordered current-frame settlement path so freshly spawned UI can finish in the same frame.
- Split stateful widget runtime work into dedicated `Build`, `Logic`, `Visual`, and `Events` sets, and migrated the complex built-in widgets and text-label sync paths onto that structure.
- Changed layout solve to preserve and drain dirty solve frontiers, reuse cached root context when enabled, and avoid treating the downward pass as a full-tree walk by default.
- Reduced render-sync overhead by caching root/clip ancestry for downstream systems and reusing rectangle meshes by logical size instead of allocating a fresh mesh on every eligible sync.
- Changed picking so the compatibility `PreUpdate` path remains available while a post-settle refresh can resolve against current-frame geometry and compare cached-vs-legacy hit resolution in validation mode.
- Extended the runtime benchmark harness with scenario filtering and stress-only workload selection so very large stress runs can be executed on demand without distorting the default perf gate.
- Updated `README.md` to point maintainers at the frame-zero rollout gate and the validation resources that now protect behavior during optimization work.

### Fixed

- Fixed the widget embedded-asset registration path so the SDF text-label shader is registered from the widget plugin instead of relying on a missing runtime asset path lookup.

## [2026-04-04]

### Added

- Added `TECH_DEBT_INVENTORY.md` as a maintainer-facing backlog for the current large-file refactor wave, covering `text_label.rs`, `layout_system.rs`, `solver.rs`, and `select.rs` with proposed module splits and execution order.
- Added general `release-readiness` docs pages in English and Arabic so release prep guidance no longer depends on the removed alpha2-specific naming.
- Added CLI performance harnesses for the layout solver and runtime paths, covering dense flex, wrapped-card, grid-dashboard, root resolution, text measurement, picking, widget-heavy panel, and world-3d panel scenarios.
- Added `scripts/run_perf_baselines.sh` plus bilingual benchmark docs so the current solver and runtime baselines can be run and reviewed from one documented entry point.

### Changed

- Rewrote the root docs and release notes around the current alpha surface, explicitly separating stable public APIs, deprecated compatibility wrappers, and still-settling areas.
- Updated the root discovery path so `README.md`, `README_AR.md`, `MIGRATION.md`, `MIGRATION_AR.md`, and `RELEASE_NOTES.md` now point at the current migration story without the removed alpha2-specific root notes.
- Tightened pre-merge validation expectations in `.github/PULL_REQUEST_TEMPLATE.md` and the bilingual docs review checklists so `check_quality`, representative examples, and docs build requirements are visible at review time.
- Replaced the four large target files with dedicated folder-based module trees while preserving behavior:
  - `crates/univis_ui_engine/src/layout/layout_system/`
  - `crates/univis_ui_engine/src/layout/core/solver/`
  - `crates/univis_ui_widgets/src/widget/select/`
  - `crates/univis_ui_widgets/src/widget/text_label/`
- Split the root system into focused modules for root types, stacking, and cached `UI3d` sync while keeping `root_resolution.rs` and `screen_transform.rs` as the public-facing resolution flow.
- Split the layout solver into focused helper, type, translation, and absolute-positioning modules without changing the existing solve entry points.
- Split the `select` widget into model, runtime, interaction, visuals, and event modules so widget state, entity-tree wiring, and visual updates are no longer concentrated in one file.
- Split the `text_label` widget into model, measurement, and render modules so text sizing, bidi/truncation logic, and SDF rendering no longer live in one monolithic implementation file.
- Recorded the current structure in `TECH_DEBT_INVENTORY.md`, then revalidated the refactor with `cargo check --workspace`, `./scripts/check_quality.sh`, and `./scripts/check_representative_examples.sh`.
- Updated the performance overview, testing docs, and benchmark docs in both languages so the repository baseline flow now includes solver and runtime harnesses plus the shared `--check` gate.

### Removed

- Removed the alpha2-specific root planning/status files and their paired docs pages after the project moved on to the current generic release-readiness flow.

## [2026-04-03]

### Added

- Added `scripts/check_quality.sh` and `scripts/check_representative_examples.sh` to centralize formatting, linting, workspace checks, library tests, and a curated release-mode example pass around the facade path, root modes, interaction, panel resize, text input, and `World3d`.
- Added a dedicated representative-examples GitHub Actions job and folded the same checks into the alpha release verification path.
- Added task-oriented `first-steps` docs pages in English and Arabic so the recommended plugin setup and best-first example path now live in one short onboarding page per language.
- Added focused text-overflow widget examples for mixed Arabic + Latin labels, truncation sides, autosize clamping, and edge cases.
- Added embedded `NotoSansArabic-Regular.ttf` and `FreeSerif.otf` theme font handles so the shipped text examples can render Arabic and Latin coverage from repo-owned assets.
- Added regression tests around grapheme-safe truncation, justify anchoring, local clip behavior, and autosize measurement under parent constraints in `UTextLabel`.
- Added public `min_width`, `max_width`, `min_height`, and `max_height` controls to `UNode`, plus explicit `UVal::MinContent` and `UVal::MaxContent` sizing modes.
- Added a dedicated sizing-semantics docs note in both English and Arabic to freeze the alpha-line meaning of `Auto`, `Content`, `MinContent`, `MaxContent`, and `min/max` constraints.
- Added focused sizing examples for contextual `Auto` sizing, explicit intrinsic modes, min/max-aware flex redistribution, fixed cards, wrap breakpoints, text sizing, and `UImage` native-size behavior.

### Changed

- Consolidated docs publishing to one GitHub Pages path and aligned the validation workflow around quality gates, docs build, API docs, representative examples, and package-by-package example checks.
- Refactored `layout_system.rs` by extracting root-resolution and screen-transform helpers into focused submodules, reducing the size of the main root-system file without changing its external behavior.
- Made `UnivisWidgetPlugin` include `UnivisTextFieldPlugin` and `UnivisBadgePlugin` by default, while keeping the dedicated plugins safe to add manually for narrower custom composition.
- Removed the empty internal `widget/menu.rs` placeholder and rewrote the bilingual plugin/setup docs so the truth tables, quick starts, compatibility notes, and widget pages all match the current runtime surface.
- Extended `UTextLabel` ellipsis handling with grapheme-aware truncation, bidi-aware segment isolation, and the new `UTextTruncateSide` API (`Auto`, `Start`, `End`, `Middle`).
- Updated the English and Arabic text-rendering docs to describe the current clipping model: local glyph clipping in `sync_text_label_meshes` plus ancestor `UClip` material clipping in `sync_text_clipper_materials`.
- Refreshed the widget and example docs in both languages so the text examples, clipping behavior, and troubleshooting guidance match the current implementation.
- Taught the layout solver to clamp solved sizes against explicit node `min/max` bounds and to respect those bounds during flex grow/shrink redistribution.
- Split `Auto` from `Content` inside the layout solver so contextual auto sizing no longer collapses into explicit intrinsic sizing.
- Kept `UVal::Content` as a backward-compatible public alias for max-content semantics while making implicit stretch behavior depend on `Auto` instead of all intrinsic modes.
- Updated `UImage` sizing so `Auto`, `Content`, `MinContent`, and `MaxContent` all resolve to the native texture size once the image asset becomes available.

### Fixed

- Fixed the default widget facade so `UTextField` and `UBadge` runtime behavior are available through `UnivisUiPlugin` / `UnivisWidgetPlugin` without extra manual plugin registration.
- Fixed duplicate-registration friction by making `UnivisTextFieldPlugin` and `UnivisBadgePlugin` idempotent when applications still add them explicitly after the default widget surface.
- Fixed validation drift by documenting and checking a stable representative-example set instead of relying only on full package sweeps or ad-hoc manual selection.
- Fixed `UTextLabel` autosize so constrained parents cap the measured size instead of letting labels grow past their available bounds.
- Fixed clipped single-line labels so `Justify::Left` and `Justify::Right` anchor to the expected edge instead of exposing the middle of the text.
- Fixed local text clipping so `Clip` and `Ellipsis` still apply when `autosize` is enabled, while `Visible` keeps overflow unclipped.
- Fixed the mixed-text demo card layouts by opting fixed-width flex items out of shrink, preventing overlap in wrapped example grids.
- Fixed grid and flex stretch behavior so `Auto` items can still fill contextual space while `MinContent` and `MaxContent` keep their intrinsic size unless alignment explicitly overrides them.

## [0.3.0-alpha.2] - 2026-03-27

### Changed

- Bumped the facade crate and all public workspace crates from `0.3.0-alpha.1` to `0.3.0-alpha.2`.
- Promoted the current `alpha2` line from an in-progress stabilization wave to a release-ready milestone after finishing the `URootUi` baseline, the docs and API-doc cleanup, the crate-owned example migration, and the alpha2 stabilization work.
- Clarified the long-term root-level file set around `README.md`, `README_AR.md`, `MIGRATION.md`, `MIGRATION_AR.md`, `RELEASE_NOTES.md`, and `changelog.md`, while keeping `ALPHA2_STATUS.md` / `ALPHA2_STATUS_AR.md` as temporary stabilization notes for the current alpha line.
- Finalized the release communication path so the hosted docs URL, example gallery, migration story, and release-readiness checklist all point at the same `0.3.0-alpha.2` public surface.

### Notes

- The detailed implementation log that led into this release remains captured under the dated entries for `2026-03-25` and `2026-03-26`.
- This release still builds on the completed `URootUi` migration baseline captured in the dated entries below.

## [2026-03-26]

### Added

- Added a visual fit-content root example for `URootUi::world_2d_fit_content()` and `UiCanvasSize::FitContent { min, max }` with `World3d` roots.
- Added `ALPHA2_STATUS.md` and `ALPHA2_STATUS_AR.md` as short root-level stability notes that freeze what is stable, transitional, and still experimental in the current `alpha2` line.
- Added dedicated docs pages for `alpha2` stability, docs publishing, and a curated example gallery in both English and Arabic.
- Added a GitHub Pages publishing workflow for the unified `docs/` book and bundled the example gallery with static visual references under `docs/src/assets/`.
- Added dedicated visual-validation and alpha2-release-readiness pages in both English and Arabic, including a final pre-release checklist and an explicit wrapper-removal decision.

### Changed

- Reorganized the documentation into a single `docs/` mdBook with `docs/src/ar` and `docs/src/en`, a shared `SUMMARY.md`, and a runtime language switcher at the top of mirrored pages, replacing the old split `book_ar` / `book_en` layout.
- Normalized language separation inside the unified docs so English chapters no longer contain Arabic prose, Arabic chapters no longer contain stray English descriptive terms, and only code identifiers remain shared across both trees.
- Rewrote `README.md` to focus on the library's practical value, strengths, use cases, quick start, and example entry points instead of spreading low-level reference detail across the landing page.
- Added `README_AR.md` as a dedicated Arabic landing page and linked it from the main `README.md`.
- Added repository-level migration summaries in `MIGRATION.md` and `MIGRATION_AR.md`, then removed the temporary docs/examples/API tracking files from the root.
- Replaced the version-specific `RELEASE_NOTES_0.3.0-alpha.1.md` root note with a stable root-level `RELEASE_NOTES.md`.
- Expanded the generated Rust API docs across the facade crate and core workspace crates by adding crate-level and module-level `rustdoc`, documenting `URootUi`, layout primitives, common widgets, interaction state, and theme resources, and hiding several internal scheduling helpers from the public documentation surface.
- Redistributed examples so each one now lives under the crate it primarily represents (`univis_ui`, `univis_ui_engine`, `univis_ui_interaction`, or `univis_ui_widgets`), then updated example manifests, package-specific example commands, example indexes, smoke-test docs, and release-validation scripts to follow the new package-aware layout.
- Froze the bilingual `docs/` information architecture around a mirrored English/Arabic navigation model, added dedicated migration pages, and introduced editorial rules for language separation, naming, and example-link conventions.
- Turned the examples chapter into a canonical package-aware catalog with purpose statements, learning/showcase/smoke tags, guide-to-example links, and reverse links from representative example source files back to their related docs pages.
- Audited the public API surface again, expanded curated `rustdoc` on high-value entry points such as `UnivisUiPlugin`, `URootUi`, `UNode`, `UInteractionColors`, `Theme`, `UTextLabel`, and `UButton`, and hid several engine-internal modules from the generated documentation story.
- Wired `README`, the docs landing pages, and the key guide chapters to their canonical examples, migration pages, and fully qualified public API entry points.
- Added a dedicated docs authoring workflow, a docs review checklist, and a pull-request template that codifies bilingual docs updates, crate-owned examples, and public `rustdoc` expectations.
- Added a GitHub Actions workflow for `mdbook build docs`, sequential package-by-package example validation, sequential public-crate `cargo doc --no-deps`, and explicit pre-alpha documentation validation requirements.
- Refreshed the release notes around the unified `docs` book and crate-owned example layout, added a dedicated migration page for users coming from previous documentation assumptions, tightened the GitHub discovery path across `README -> docs -> examples -> API docs`, and performed a final consistency pass on names, commands, and package-aware example invocations.
- Froze the current `URootUi`-centered public surface, explicitly kept `UScreenRoot` and `UWorldRoot` only as deprecated wrappers for the remainder of `alpha2`, re-audited the shipped examples, and published root-level plus in-book stability notes.
- Wired a GitHub Pages docs publishing workflow, defined the hosted docs URL, documented how the published build is produced from `docs/`, and kept the local `mdbook build docs` path as the canonical contributor flow.
- Added a curated example gallery, a best-first example list, purpose-driven grouped example sections, and static visual references for representative showcase and solver scenes.
- Decided the long-term root-level release file set, clarified the unique role of each root file, added “where to read what” guidance, and tightened `RELEASE_NOTES.md` so it stays release-focused instead of duplicating the landing or migration files.
- Hardened docs/example validation around clearer CI job boundaries, added a lightweight visual-validation checklist, and explicitly kept screenshots as manual release-prep material rather than a required CI artifact.
- Rechecked the canonical root examples, confirmed that docs/migration/release notes describe the same current reality, added a final alpha2 release-readiness checklist, and decided not to remove deprecated root wrappers immediately after the next alpha cut.

### Notes

- The completed `URootUi` migration baseline remains the foundation for this follow-up example work.

## [2026-03-25]

### Changed

- Started the `alpha2` `URootUi` root-system migration by unifying public roots around `URootUi`, keeping `UScreenRoot` and `UWorldRoot` as compatibility wrappers during `alpha2`, and freezing the default `meters_per_unit` and `UiCameraRef::Auto` behavior.
- Introduced shared root resolution through `URootUi`, `UiSpace`, `UiCanvasSize`, `UiCameraRef`, and internal `ResolvedRootUi`.
- Made screen roots behave as real HUD roots tied to the resolved camera and projection instead of acting like ordinary window-sized world canvases.
- Removed hardcoded `Camera2d` assumptions from picking and panel resize paths and resolved interaction cameras from each root.
- Made the render path derive from `ResolvedRootUi.space`, mapping `World3d` to the 3D material path and `Screen` / `World2d` to the 2D material path.
- Replaced `UI3d` propagation with derived cached state from each node's resolved root, including stale-state removal when roots switch mode and correct inheritance for newly spawned children.
- Normalized the unit model around logical UI units, applied `meters_per_unit` consistently to world-space mesh sizing and transforms, and kept `resolution_scale` separate from physical world size.
- Promoted `URootUi` as the public root API, documented `screen/world_2d/world_3d` constructors, and marked `UScreenRoot` / `UWorldRoot` as deprecated compatibility wrappers with migration notes for `alpha2`.
- Migrated all shipped examples to the `URootUi` API so the example surface now demonstrates `screen()`, `world_2d(...)`, and `world_3d(...)` directly instead of the deprecated root wrappers.
- Finished the example migration, added explicit `screen` HUD and world-scale showcase examples, and rechecked example semantics around the new root model.
- Rewrote root docs, quick-start snippets, and interaction support docs around `URootUi`, logical UI units, viewport semantics, and explicit world scaling.
- Verified the migration by running `cargo check --workspace`, sequential targeted tests for `univis_ui_engine`, `univis_ui_widgets`, and `univis_ui_interaction`, refreshed smoke checks for screen, world, and 3D examples, and final manual visual validation of screen HUD and world-scale behavior, while also adding direct verification tests for root resolution, root-bound layout sizing, `Screen`/`World2d`/`World3d` picking paths, panel root-camera resize behavior, world-root attachment, and `meters_per_unit` physical scaling.
- Made `URootUi` behave as a closed stacking capsule across layout, rendering, text, and picking so descendants never interleave above another root automatically; root-vs-root order is now resolved first, and same-`z` roots break ties by spawn order.

### Added

- Added screen-HUD, world-scale, and root-capsule overlap visual examples for `URootUi`.
- Added content-driven world-root sizing through `UiCanvasSize::FitContent { min, max }` plus `URootUi::world_2d_fit_content()` and `URootUi::world_3d_fit_content()`, allowing world roots to derive their logical canvas from measured content and optionally clamp it.

### Fixed

- Restored legacy `UWorldRoot` example visibility during `alpha2` by preserving its historical `1 UI unit = 1 world unit` behavior as a compatibility path while the newer `URootUi` scale model continues to use `meters_per_unit`.
- Fixed the `UTextLabel` SDF edge filtering regression that made some examples render text with an exaggerated halo after the unit-model changes.
- Fixed bloom-enabled examples by pinning camera tonemapping to a LUT-free mode instead of relying on the default `TonyMcMapFace` path, which can fail at runtime when Bevy's `tonemapping_luts` feature is not enabled.
- Fixed a screen-HUD example after verification by updating its camera query to the current Bevy `single_mut()` result-based API, allowing the updated example set to compile cleanly.
- Fixed cross-root stacking leaks where a child such as a port or button could visually or interactively rise above another `URootUi` root even though its own root was below that other root.
- Fixed the world-root sizing gap where `World2d` and `World3d` roots had to be declared with fixed canvas sizes even when the desired behavior was to wrap measured content.

### Notes

- The `URootUi` migration is still in progress for `alpha2`; additional breaking changes are expected in later iterations.
- Visual validation confirmed that `URootUi::screen()` stays viewport-fixed while the world canvas continues to move, rotate, and scale with the camera, and that screen HUD behavior stays separate from world-root physical sizing.
- The new fit-content world-root mode works best when the root content has intrinsic or fixed sizing; heavy `%` or root-relative flex can still create circular sizing expectations.
- This changelog entry backfills the previously undocumented `URootUi` migration work that led into `alpha2`.

## [2026-03-16]

### Added

- Added a text zoom example to inspect `UTextLabel` sharpness while changing camera zoom with the mouse wheel, `+`, `-`, and `0`.

### Changed

- Reduced default `bevy` surface across the workspace by switching to a shared minimal feature set instead of full default features.
- Moved all internal crates to `bevy = { workspace = true }` so Bevy feature management is centralized in the workspace root.
- Gated bloom-heavy examples behind the optional `example_bloom` feature.
- Reworked `UTextLabel` so layout is driven by Bevy's `TextPipeline`/`ComputedTextBlock` path while rendering is handled by a dedicated SDF mesh/material path instead of a `Text2d` child.

### Fixed

- Eliminated the close-range pixelation in `UTextLabel` by replacing the temporary raster child rendering path with shared SDF glyph atlases.
- Kept `UTextLabel` autosizing aligned with the final text layout after the renderer migration.
- Ensured `UTextLabel` ellipsis layout and rendered glyph meshes stay in sync by remeasuring the final displayed text before mesh generation.
- Allowed autosized `UTextLabel` nodes to shrink back correctly when their text becomes empty.
- Stopped `UTextLabel` from retrying failed text measurement every frame when a font handle is unresolved or not ready yet.
- Improved `UDragValue` precision on large ranges by adapting drag sensitivity to the widget resolution, making values like `23` practical to hit even across spans such as `1..1000`.

## [0.3.0-alpha.1] - 2026-03-07

### Added

- Workspace split into public crates:
  - `univis_ui_engine`
  - `univis_ui_style`
  - `univis_ui_interaction`
  - `univis_ui_widgets`
  - `univis_ui` facade
- Expanded built-in widget surface to include forms and advanced controls such as `seekbar`, `checkbox`, `toggle`, `radio`, `text_field`, `scroll_view`, `divider`, `panel`, `drag_value`, and `select`.
- Added stronger release tooling and serial validation scripts for low-resource machines.
- Expanded documentation with EN/AR books, compatibility notes, and validation guidance.

### Changed

- Reorganized the library from a monolithic crate into a layered workspace with a facade that preserves `UnivisUiPlugin` and the prelude UX.
- Refined plugin composition around `style -> engine -> interaction -> widgets`.
- Improved examples and visual references substantially compared with the `0.1.x` line.

### Notes

- This release should be treated as a new alpha line rather than a small follow-up to `0.1.1`.
- Consumers relying on older internal paths or broad re-exports may need import adjustments.

## [2026-03-06]

### Changed

- Aligned docs and books with actual plugin behavior, layout schedule, and current project structure.
- Reduced API noise by narrowing some re-exports and keeping menu placeholders internal.
- Added runtime warnings for optional widget plugins that are not registered.
- Added serial release verification scripts and documented a sequential validation path for weaker machines.
