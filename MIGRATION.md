# Migration Summary

This file explains how to navigate the repository after the docs/examples reshuffle.

## Current Branch Status

- documentation lives in one bilingual `mdBook` under `docs/`
- the public root model centers on `URootUi`
- most of the older workspace example source files are archived and are no longer shipped under
  root `examples/` or `crates/*/examples/`
- the current live demo package is `android/android_phone_app`
- the current static visual references live under `docs/src/assets/visual-references/`

## If You Are Coming From Older Docs

Treat older example commands as historical only in this branch.

Examples such as:

- `cargo run --example hello_world`
- `cargo run -p univis_ui --example root_fit_content`
- `cargo run -p univis_ui_widgets --example text_field`

describe earlier repository layouts, not the current working tree.

Use these entry points instead:

1. `README.md` or `README_AR.md`
2. `docs/src/index.md`
3. the relevant guide chapter
4. `docs/src/en/examples/index.md` or `docs/src/ar/examples/index.md` for ownership/history
5. `cargo run --manifest-path android/android_phone_app/Cargo.toml` for the current live mobile-style demo

## Canonical Public Surface

Use these as the current public assumptions:

- canonical roots: `URootUi`, `URootUi::screen()`, `URootUi::world_2d(...)`, `URootUi::world_3d(...)`
- canonical root support types: `UiSpace`, `UiCameraRef`, `UiCanvasSize`
- deprecated compatibility wrappers: `UScreenRoot`, `UWorldRoot` on explicit paths only
- long-term physical-size compatibility knob: `meters_per_unit: 1.0` on `URootUi`

Still worth rechecking during migration:

- fit-content world roots under heavy relative sizing
- visual behavior in `World3d`-style scenes
- any rendering-, layout-, or picking-heavy change that still needs manual validation

## Example Availability

The examples catalog now distinguishes three states:

- `Live package`: source still ships in the current branch and can be run directly
- `Static reference`: an HTML or image reference is kept for visual comparison
- `Archived source`: the historical example name is still documented, but the source file is not shipped in this branch

See:

- `docs/src/en/examples/index.md`
- `docs/src/ar/examples/index.md`
- `docs/src/en/examples/gallery.md`
- `docs/src/ar/examples/gallery.md`

## Related Migration Pages

- `docs/src/en/migration/docs-and-examples.md`
- `docs/src/en/migration/root-api.md`
- `docs/src/en/migration/legacy-compatibility.md`
- `docs/src/en/migration/example-paths.md`

Arabic mirrors:

- `docs/src/ar/migration/docs-and-examples.md`
- `docs/src/ar/migration/root-api.md`
- `docs/src/ar/migration/legacy-compatibility.md`
- `docs/src/ar/migration/example-paths.md`

## Source Of Truth

- project story and crate map: `README.md`
- Arabic landing page: `README_AR.md`
- guides: `docs/src/en/*` and `docs/src/ar/*`
- example availability map: `docs/src/en/examples/index.md` and `docs/src/ar/examples/index.md`
- API reference: generated `cargo doc --no-deps -p univis_ui`
