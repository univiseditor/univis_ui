# Release Notes

This file replaces the version-specific `RELEASE_NOTES_0.3.0-alpha.1.md` root note and keeps the current alpha release story in one stable location.

## Where To Read What

The intended long-term root file set is:

- `README.md`
- `README_AR.md`
- `ROADMAP.md`
- `MIGRATION.md`
- `MIGRATION_AR.md`
- `RELEASE_NOTES.md`
- `RELEASE_PREP_0.3.0.md`
- `changelog.md`

Use them like this:

- `README.md`: landing story and first stop for GitHub visitors
- `README_AR.md`: Arabic landing story
- `ROADMAP.md`: forward-looking execution plan from alpha cleanup to beta readiness
- `MIGRATION.md`: migration path from older docs and assumptions
- `MIGRATION_AR.md`: Arabic migration path
- `RELEASE_NOTES.md`: current alpha release-scale summary
- `RELEASE_PREP_0.3.0.md`: local evidence for the next `0.3.0` stabilization cut
- `changelog.md`: dated historical record

## Current Release Story

Latest published release tracked here: `0.3.0`

This release should be understood as an engineering-quality and maintainability follow-up to `0.3.0-alpha.2`, not as a reset of the `alpha2` public API baseline. The root model, workspace shape, and crate-owned example layout from `alpha2` remain in place, while `alpha3` focuses on making that baseline cleaner, easier to maintain, and harder to regress.

Public docs URL:

```text
https://univiseditor.github.io/univis_ui/
```

The project has changed substantially in these areas:

- the public import story is now more curated, with deprecated root wrappers kept off recommended preludes
- the largest engine, interaction, and widget hotspots were split into focused module trees
- release validation now covers public API exposure and structural guardrails in addition to docs/examples/test passes
- maintainers now have explicit docs for crate boundaries, naming rules, widget structure, error modeling, and legacy compatibility policy
- several opaque runtime failure paths were replaced with typed diagnostics and consistent warning messages

## Current `0.3.0` Preparation Status

Local release-prep evidence for the next `0.3.0` stabilization cut is recorded in `RELEASE_PREP_0.3.0.md`.

As of 2026-05-18:

- local `./scripts/verify_alpha_release.sh` passed
- local `mdbook build docs` passed
- the ignored local lockfile was refreshed from yanked `fastrand 2.4.0` to `fastrand 2.4.1` before the final package rehearsal
- GitHub Actions confirmation, manual visual validation, and RC feedback closure are still pending

For `0.3.0`, the intended message is:

- the `alpha2` runtime and migration baseline is still the public foundation
- the default user-facing import path is clearer and exposes less accidental surface area
- the internal implementation is substantially easier to review, extend, and test
- the release process now protects structure and public-surface quality, not only functional behavior

## Main Upgrade Themes

### Architecture

- `univis_ui` remains the facade crate
- `univis_ui_engine`, `univis_ui_style`, `univis_ui_interaction`, and `univis_ui_widgets` now represent clearer public subsystem boundaries
- low-level users should depend on the crate that owns the capability they need
- major engine, interaction, and widget hotspots were split into responsibility-led module trees instead of staying concentrated in a few very large files

### Public Surface And Compatibility

- `URootUi` is the public root model
- `screen`, `world_2d`, and `world_3d` remain the canonical root spaces
- deprecated wrappers such as `UScreenRoot` and `UWorldRoot` are now an explicit-only migration path instead of part of the recommended prelude story
- content-sized world roots remain supported through `UiCanvasSize::FitContent { min, max }`
- root capsules continue to prevent cross-root stacking leaks between separate UI trees

### Quality And Validation

- `check_quality.sh` now includes public API surface and structure guardrails
- dedicated scripts now check that deprecated wrappers stay out of the default import story and that source hotspots do not silently grow back
- typed diagnostics now cover high-value runtime paths such as text measurement and select runtime tree setup

## Current Public Surface

### Stable Enough To Build On

- `URootUi` as the canonical root API
- `URootUi::screen()`, `URootUi::world_2d(...)`, and `URootUi::world_3d(...)`
- `UiSpace`, `UiCameraRef`, and `UiCanvasSize`, including fit-content world roots
- crate-owned example locations plus package-aware example commands
- curated facade and engine preludes that keep migration wrappers off the default import surface

### Deprecated But Still Supported

- `UScreenRoot` on explicit paths only
- `UWorldRoot` on explicit paths only
- explicit `meters_per_unit` when controlling the physical size of world roots

### Still Settling

- fit-content world roots under heavy `%` sizing or strongly root-relative flex layouts
- some `World3d` showcase polish defaults
- manual visual validation remains part of release prep for rendering-heavy changes
- the final removal timing for deprecated root wrappers still depends on migration feedback after `alpha3`

### Docs And Examples

- `README.md` and `README_AR.md` are the GitHub landing pages
- `docs/src/index.md` is the docs landing point
- `https://univiseditor.github.io/univis_ui/` is the intended hosted docs URL
- examples now live next to the crate they primarily represent
- package-aware example commands are now the default
- `docs/src/en/api/crate-map.md` and `docs/src/ar/api/crate-map.md` explain which crate surface to depend on
- maintainer docs now include architecture rules, widget structure, naming conventions, error-model guidance, and legacy compatibility status

### Migration

If you are upgrading from older docs or older root wrappers, start here:

- `MIGRATION.md`
- `MIGRATION_AR.md`
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

Run the structure/public-surface guards:

```bash
./scripts/check_public_api_surface.sh
./scripts/check_structure_guardrails.sh
```

Run representative examples:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
cargo run --example responsive_layout_test
cargo run --example widgets_controls
cargo run --example widgets_inputs
```

## Where To Look Next

- release history and implementation highlights: `changelog.md`
- stable-readiness plan for the `0.2.x` line: `ROADMAP.md`
- local release-prep evidence: `RELEASE_PREP_0.3.0.md`
- alpha3 diff report: `ALPHA3_REPORT.md`
- migration summary: `MIGRATION.md`
- Arabic migration summary: `MIGRATION_AR.md`
- current large-file refactor backlog: `TECH_DEBT_INVENTORY.md`
- docs home: `docs/src/index.md`

## Notes

- the completed `URootUi` migration baseline remains the foundation for the current alpha line
- `0.3.0` is the release that turns the post-`alpha2` cleanup wave into a documented public-surface and maintainability milestone
- the old version-specific release note file was removed to keep one stable root-level release note entry point
- this file should stay focused on release-scale summary rather than acting as a duplicate of `README` or `MIGRATION`
- `0.3.0-alpha.2` remains the release that closed the previous docs/examples/release cleanup wave
