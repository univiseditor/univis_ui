# Maintainer Map

This page is the short maintainer-facing map for the workspace: which crate owns what, where major module boundaries live, and which hotspots need careful review.

## Workspace Crate Map

- `univis_ui`: facade crate for the recommended application-facing import story and the full stack plugin
- `univis_ui_style`: shared theme, font, and icon resources
- `univis_ui_engine`: root model, layout solver, settlement schedule, and render sync
- `univis_ui_interaction`: picking, interaction state, and pointer-driven validation
- `univis_ui_widgets`: higher-level controls built on top of engine and interaction crates

## Engine Module Map

- `layout::layout_system`: root resolution, root stacking capsules, screen transforms, and per-root settlement state
- `layout::univis_node`: authored node, layout, and local-positioning components
- `layout::geometry`: logical UI units, sides, constraints, and axis helpers
- `layout::core`: hierarchy tracking, cache invalidation, measurement, solving, and solver helpers
- `layout::render`: mesh/material synchronization after layout settles
- `layout::profiling`: optional runtime diagnostics and overlay helpers
- `layout::registration` and `layout::settlement_loop`: pipeline wiring and bounded `PostUpdate` execution

## Widget Module Map

- single-file widgets stay flat when their runtime is small
- stateful widgets split by responsibility:
  - `model`
  - `runtime`
  - `interaction`
  - `visuals`
  - `events`
- `text_label` is the main exception because it owns both `measure/` and `render/` subtrees

## Hotspot Ownership

- engine layout hotspots: `crates/univis_ui_engine/src/layout/**`
- interaction hotspots: `crates/univis_ui_interaction/src/interaction/**`
- widget hotspots: `crates/univis_ui_widgets/src/widget/**`
- facade/docs/import story: root `src/`, `README*`, and `docs/src/**`

## Review Priority Areas

- root resolution and root stacking changes can affect layout, rendering, and picking at once
- cache and settlement loop changes can create broad regressions even when local tests pass
- text-label measurement/render changes need both layout and rendering validation
- widget runtime refactors should preserve the `Build -> Logic -> Visual -> Events` schedule story
