# Release Notes - 0.2.0-alpha.1

Date: March 7, 2026

## Scope

This document compares the currently prepared `0.2.0-alpha.1` working tree against the previously published `0.1.1` release of `univis_ui`.

Comparison basis:
- Published crate page: `https://crates.io/crates/univis_ui/0.1.1`
- API/docs snapshot for the old release: `https://docs.rs/crate/univis_ui/0.1.1`
- Local source tag for the old release: `v0.1.1`
- Current local target release: `0.2.0-alpha.1`

Important note:
- This comparison targets the current alpha working tree, not a previously published `0.2.0` crate.
- The tracked diff from `v0.1.1` to the current tree already exceeds `13k+` insertions and `3k+` deletions, and the active alpha prep adds more new files on top of that.

## Executive Summary

`0.2.0-alpha.1` is not a small follow-up to `0.1.1`. It is a structural release:
- the old monolithic crate has been reorganized into a layered workspace
- the widget surface has expanded from a small base set into a much broader built-in UI kit
- layout and rendering now expose a more serious engine surface with style/theme separation
- documentation, examples, and release validation have been expanded substantially

In practice, `0.2.0-alpha.1` should be treated as a new alpha line, not as a minor patch over `0.1.1`.

## High-Level Delta

| Area | `0.1.1` | `0.2.0-alpha.1` |
| --- | --- | --- |
| Packaging | Single public crate | Workspace with `univis_ui_engine`, `univis_ui_style`, `univis_ui_interaction`, `univis_ui_widgets`, plus `univis_ui` facade |
| Plugin composition | Direct monolith plugin wiring | Layered facade: `style -> engine -> interaction -> widgets` |
| Widget surface | `button`, `text_label`, `badge`, `progress` | 16 public widget modules, including forms and advanced controls |
| Examples shipped | 3 Rust examples | 31 Rust examples plus HTML visual reference files |
| Docs | Mostly README-centric | README, AI integration manual, and EN/AR mdBooks |
| Release tooling | Minimal | Serial release verification, alpha package rehearsal, and explicit publish checklist |

## Major Changes Since 0.1.1

### 1. Architecture and Packaging

`0.1.1` was a monolithic crate with a direct `bevy` dependency and all systems under `src/layout`, `src/widget`, `src/interaction`, and `src/univis_debug`.

`0.2.0-alpha.1` is now organized around explicit public crates:
- `univis_ui_engine`
- `univis_ui_style`
- `univis_ui_interaction`
- `univis_ui_widgets`
- `univis_ui` facade

This is the largest breaking change in the release. The public design now reflects the actual subsystem boundaries instead of exposing one flat internal tree.

### 2. Plugin Model

Old `UnivisUiPlugin` in `0.1.1` assembled:
- `UnivisInteractionPlugin`
- `MeshPickingPlugin`
- `UnivisNodePlugin`
- `UnivisLayoutPlugin`
- `UnivisWidgetPlugin`

Current `UnivisUiPlugin` assembles:
- `UnivisUiStylePlugin`
- `UnivisEnginePlugin`
- `UnivisInteractionPlugin`
- `UnivisWidgetPlugin`

The new `UnivisEnginePlugin` is the low-level bundle that owns node setup, layout solving, and render sync. This is a much cleaner public story than exposing raw engine pieces directly from the facade.

### 3. Engine Surface Expansion

Compared to `0.1.1`, the current engine adds or formalizes several low-level capabilities:
- separated style/theme/fonts/icons support
- `UWorldRoot { is_3d, resolution_scale, size }`
- `UPbr` for lit 3D UI control
- `UClip`
- `UImage`
- `UGridAutoFlow`
- `ULayoutContainerExt`
- `ULayoutItemExt`
- layout cache and cache-aware solve path
- dedicated layout profiling systems and overlay controls

`0.1.1` already had the core display modes (`Flex`, `Grid`, `Stack`, `Radial`, `Masonry`), but the new line is much closer to an actual UI engine instead of a promising prototype.

### 4. Widgets: 4 Modules to a Real UI Kit

The old published crate exposed 4 public widget modules:
- `button`
- `text_label`
- `badge`
- `progress`

The current alpha exposes a much larger built-in widget set:
- `button`
- `text_label`
- `badge`
- `progress`
- `image`
- `seekbar`
- `checkbox`
- `icon_btn`
- `toggle`
- `radio`
- `text_field`
- `scroll_view`
- `divider`
- `panel`
- `drag_value`
- `select`

This is one of the most visible user-facing differences between the two lines.

### 5. Interaction Improvements

Interaction remains built around pointer observers and a custom picking backend, but the current alpha line improves structure and behavior:
- interaction is now its own public crate
- `on_pointer_click` is part of the interaction plugin registration
- picking and feedback code now live behind clearer crate boundaries
- optional-widget misconfiguration warnings were added for cases like `UTextField` and `UBadge`

### 6. Examples and Visual References

The package grew from 3 Rust examples in `0.1.1` to 31 Rust examples in the current alpha tree.

The new tree also adds visual reference HTML files for layout comparison, including:
- `layout_solver_no_widgets.html`
- `layout_solver_ultra_complex.html`
- `complex_dashboard.html`

This matters because the project is no longer shipping only demos; it now ships reference material for solver correctness and visual parity checks.

### 7. Documentation Maturity

`0.1.1` was largely README-driven.

`0.2.0-alpha.1` now ships:
- a rewritten README aligned with the actual codebase
- `readme_for_ai.md`
- Arabic mdBook
- English mdBook
- architecture, API, interaction, layout, rendering, performance, widget, and troubleshooting pages

This is a large jump in project maturity and onboarding quality.

### 8. Release and Validation Tooling

`0.1.1` did not ship a serious release-validation story.

The current alpha line adds:
- `scripts/test_lib_serial_release.sh`
- `scripts/check_examples_serial_release.sh`
- `scripts/verify_serial_release.sh`
- `scripts/package_alpha_serial.sh`
- `scripts/verify_alpha_release.sh`
- `ALPHA_RELEASE_CHECKLIST.md`

This is especially relevant for low-resource machines and for staged publishing of a multi-crate workspace.

## Breaking Changes / Migration Notes

Consumers moving from `0.1.1` should expect breaking changes.

### Packaging and imports
- Internal monolithic module assumptions no longer hold.
- Advanced users should now import from `univis_ui_engine`, `univis_ui_style`, `univis_ui_interaction`, and `univis_ui_widgets` as needed.
- Default users should continue to prefer `univis_ui::prelude::*`.

### Plugin composition
- Style initialization is now a first-class concern via `UnivisUiStylePlugin`.
- Low-level composition should use `UnivisEnginePlugin` instead of hand-assembling layout/render/node pieces from old internal paths.

### Internal API visibility
- More engine-only types are intentionally kept out of public preludes.
- Consumers relying on broad internal re-exports from the old tree should expect import adjustments.

### Docs and examples
- Old README installation snippets were outdated in places.
- The current docs and examples are the new source of truth for how the crate should be consumed.

## Validation Snapshot for the Alpha Line

As of March 7, 2026, the current alpha preparation passes:
- `cargo check --workspace`
- serial release lib tests for the public crates
- release example checks across the example set
- package rehearsal for `univis_ui_style`
- package file-list audit for dependent crates in offline mode

That is enough to justify an alpha line, but it should still be presented as an alpha and not as a stable production release.

## Bottom Line

`0.2.0-alpha.1` is a substantial step forward from `0.1.1` in every meaningful axis:
- architecture
- public API shape
- widgets
- examples
- docs
- release discipline

It is also a breaking transition. Users should read it as the start of a new alpha generation of Univis UI, not as a conservative incremental upgrade.
