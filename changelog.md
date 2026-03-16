# Changelog

All notable changes to this project will be documented in this file.

This changelog is intentionally concise.
For deeper release context, see:
- `RELEASE_NOTES_0.2.0-alpha.1.md`
- `RELEASE_NOTES_2026-03-06.md`

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
