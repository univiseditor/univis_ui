# Changelog

All notable changes to this project will be documented in this file.

This changelog is intentionally concise.
For deeper release context, see:
- `RELEASE_NOTES_0.2.0-alpha.1.md`
- `RELEASE_NOTES_2026-03-06.md`

## [2026-03-25]

### Changed

- Started the `alpha2` `URootUi` root-system migration and documented the frozen Phase 0 decisions in the roadmap:
  - unify public roots around `URootUi`
  - keep `UScreenRoot` and `UWorldRoot` as compatibility wrappers during `alpha2`
  - freeze the default `meters_per_unit` and `UiCameraRef::Auto` behavior
- Completed Phase 1 of the `URootUi` refactor by introducing shared root resolution through `URootUi`, `UiSpace`, `UiCanvasSize`, `UiCameraRef`, and internal `ResolvedRootUi`.
- Completed Phase 2 of the `URootUi` refactor by making screen roots behave as real HUD roots tied to the resolved camera and projection instead of acting like ordinary window-sized world canvases.
- Completed Phase 3 of the `URootUi` refactor by removing hardcoded `Camera2d` assumptions from picking and panel resize paths and resolving interaction cameras from each root.
- Completed Phase 4 of the `URootUi` refactor by making the render path derive from `ResolvedRootUi.space`, mapping `World3d` to the 3D material path and `Screen` / `World2d` to the 2D material path.

### Notes

- The `URootUi` migration is still in progress for `alpha2`; additional breaking changes are expected in later phases.
- This changelog entry backfills the previously undocumented completed phases: `Phase 0`, `Phase 1`, `Phase 2`, `Phase 3`, and `Phase 4`.

## [2026-03-16]

### Added

- Added `text_label_zoom` example to inspect `UTextLabel` sharpness while changing camera zoom with the mouse wheel, `+`, `-`, and `0`.

### Changed

- Reduced default `bevy` surface across the workspace by switching to a shared minimal feature set instead of full default features.
- Moved all internal crates to `bevy = { workspace = true }` so Bevy feature management is centralized in the workspace root.
- Gated bloom-heavy examples behind the optional `example_bloom` feature:
  - `border_light_3d`
  - `card_profile`
  - `sci_fi`
- Reworked `UTextLabel` so layout is driven by Bevy's `TextPipeline`/`ComputedTextBlock` path while rendering is handled by a dedicated SDF mesh/material path instead of a `Text2d` child.

### Fixed

- Eliminated the close-range pixelation in `UTextLabel` by replacing the temporary raster child rendering path with shared SDF glyph atlases.
- Kept `UTextLabel` autosizing aligned with the final text layout after the renderer migration.
- Ensured `UTextLabel` ellipsis layout and rendered glyph meshes stay in sync by remeasuring the final displayed text before mesh generation.
- Allowed autosized `UTextLabel` nodes to shrink back correctly when their text becomes empty.
- Stopped `UTextLabel` from retrying failed text measurement every frame when a font handle is unresolved or not ready yet.
- Improved `UDragValue` precision on large ranges by adapting drag sensitivity to the widget resolution, making values like `23` practical to hit even across spans such as `1..1000`.

## [0.2.0-alpha.1] - 2026-03-07

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
