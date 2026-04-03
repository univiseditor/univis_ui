# Release Notes

This file replaces the version-specific `RELEASE_NOTES_0.2.0-alpha.1.md` root note and keeps the current alpha release story in one stable location.

## Where To Read What

The intended long-term root file set is:

- `README.md`
- `README_AR.md`
- `MIGRATION.md`
- `MIGRATION_AR.md`
- `RELEASE_NOTES.md`
- `changelog.md`

Use them like this:

- `README.md`: landing story and first stop for GitHub visitors
- `README_AR.md`: Arabic landing story
- `MIGRATION.md`: migration path from older docs, examples, and assumptions
- `MIGRATION_AR.md`: Arabic migration path
- `RELEASE_NOTES.md`: current alpha release-scale summary
- `changelog.md`: dated historical record

`ALPHA2_STATUS.md` and `ALPHA2_STATUS_AR.md` remain temporary stabilization notes for the current alpha line rather than long-term root release files.

## Current Release Story

Current target release: `0.2.0-alpha.2`

The current `alpha2` line should be understood as a structural continuation of the `0.2.0-alpha.1` transition, not as a small patch over the old `0.1.x` line.

Public docs URL:

```text
https://univiseditor.github.io/univis_ui/
```

The project has changed substantially in these areas:

- packaging moved from a monolithic crate to a layered workspace
- the public root model now centers on `URootUi`
- examples are distributed by owning crate
- documentation now lives in one bilingual `mdBook` under `docs/`
- generated `API Docs` are part of the intended learning path
- release validation now includes explicit sequential checks for docs, examples, and public crates

For `0.2.0-alpha.2`, the intended message is:

- the `URootUi` migration baseline is in place
- the docs surface, migration surface, and release surface now have clearer roles
- the example catalog has matured into a gallery plus a package-aware reference index
- the `alpha2` line is considered complete enough to cut as a coherent milestone

## Main Upgrade Themes

### Architecture

- `univis_ui` remains the facade crate
- `univis_ui_engine`, `univis_ui_style`, `univis_ui_interaction`, and `univis_ui_widgets` now represent clearer public subsystem boundaries
- low-level users should depend on the crate that owns the capability they need

### Roots And Layout

- `URootUi` is the public root model
- `screen`, `world_2d`, and `world_3d` are the canonical root spaces
- content-sized world roots are supported through `UiCanvasSize::FitContent { min, max }`
- root capsules prevent cross-root stacking leaks between separate UI trees

### Docs And Examples

- `README.md` and `README_AR.md` are the GitHub landing pages
- `docs/src/index.md` is the docs landing point
- `https://univiseditor.github.io/univis_ui/` is the intended hosted docs URL
- examples now live next to the crate they primarily represent
- package-aware example commands are now the default

### Migration

If you are upgrading from older docs, older example paths, or older root wrappers, start here:

- `MIGRATION.md`
- `MIGRATION_AR.md`
- `ALPHA2_STATUS.md`
- `ALPHA2_STATUS_AR.md`
- `docs/src/en/migration/`
- `docs/src/ar/migration/`

## Docs Publishing

- local preview stays centered on `mdbook build docs` and `mdbook serve docs`
- the hosted site is built from the same `docs/` tree
- `.github/workflows/docs_publish.yml` is the only Pages publishing workflow
- `.github/workflows/docs_examples_api.yml` handles quality, docs, example, and API-doc validation

## Canonical Commands

Generate API docs:

```bash
cargo doc --no-deps -p univis_ui
```

Build the docs site:

```bash
mdbook build docs
```

Run representative examples:

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui_engine --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui_interaction --example interaction
```

## Where To Look Next

- release history and implementation highlights: `changelog.md`
- migration summary: `MIGRATION.md`
- Arabic migration summary: `MIGRATION_AR.md`
- alpha2 stability note: `ALPHA2_STATUS.md`
- docs home: `docs/src/index.md`

## Notes

- the completed `URootUi` migration baseline from `Phase 0` through `Phase 10` remains the foundation for the current alpha line
- the old version-specific release note file was removed to keep one stable root-level release note entry point
- this file should stay focused on release-scale summary rather than acting as a duplicate of `README` or `MIGRATION`
- `0.2.0-alpha.2` is the point where the current docs/examples/release cleanup wave is considered complete enough for a release cut
