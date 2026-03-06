# Release Notes - 2026-03-06

## Change Range

- Base commit: `9b7416d`
- Head commit: `919cc77`
- Commits in range: `9`
- Files changed: `39`
- Diff summary: `712 insertions`, `27 deletions`

## Scope

- Documentation alignment and consistency updates.
- EN/AR book coverage improvements.
- Small refactors to reduce API noise and naming issues.
- Runtime warnings for optional widget plugin misconfiguration.
- Serial release validation tooling for low-resource machines.
- Workspace split into public crates with facade compatibility.

## Migration Notes

- Project now uses a workspace with public crates:
- `univis_ui_core`
- `univis_ui_layout`
- `univis_ui_render`
- `univis_ui_interaction`
- `univis_ui_widgets`
- `univis_ui` remains a facade crate and still exposes `UnivisUiPlugin` and `univis_ui::prelude::*`.
- Existing facade-based apps should continue to work without import changes.
- Advanced users can compose plugins directly from split crates.
- Suggested publish order: `core` -> `layout` -> `render` -> `interaction` -> `widgets` -> `univis_ui`.

## Highlights

### 1) P0 Docs Alignment (Code Truth)

- Aligned `README.md`, `readme_for_ai.md`, and related book pages with current code behavior.
- Clarified plugin truth:
- `UnivisUiPlugin` includes `UnivisWidgetPlugin`.
- `UnivisWidgetPlugin` does not auto-register `UnivisTextFieldPlugin`.
- `UnivisWidgetPlugin` does not auto-register `UnivisBadgePlugin`.
- Corrected layout schedule docs to match the 3-step `PostUpdate` chain with no duplicate downward pass.
- Corrected book path references to `book_ar/` and `book_en/`.
- Documented current interaction camera assumption (`Camera2d` path).

### 2) Book Documentation Expansion (EN+AR)

- Added plugin truth table pages.
- Added interaction support matrix pages.
- Added current limitations pages.
- Added example validation reports.
- Added compatibility matrix pages.
- Added smoke test plan pages.
- Updated book summaries/navigation links accordingly.

### 3) Refactors and API Surface Cleanup

- Renamed `src/layout/geomerty.rs` to `src/layout/geometry.rs`.
- Reduced broad prelude re-exports in layout modules.
- Kept `widget/menu.rs` internal placeholder to reduce public API noise.

### 4) Optional Plugin Safety Feedback

- Added runtime warnings when optional systems are missing:
- `UTextField` without `UnivisTextFieldPlugin`.
- `UBadge` without `UnivisBadgePlugin`.
- `UTag` usage warning (runtime system limitations).
- Added installed-marker resources for TextField and Badge plugins.

### 5) Validation Tooling for Weak Machines

- Added serial release scripts:
- `scripts/test_lib_serial_release.sh`
- `scripts/check_examples_serial_release.sh`
- `scripts/verify_serial_release.sh`
- Documentation updated to recommend sequential checks with low build pressure.

## Validation Summary

- `cargo check --quiet`: pass.
- `mdbook build book_en`: pass.
- `mdbook build book_ar`: pass.
- Serial lib tests in `--release`: pass (`37/37`).
- Serial example compile checks in `--release`: pass (`28/28`).

## Deferred in This Cycle

- Manual runtime smoke runs (`cargo run --release --example ...`) were intentionally deferred/skipped due to machine/resource constraints.
- Deferred status is documented in:
- `book_en/src/development/example-validation.md`
- `book_ar/src/development/example-validation.md`
- `book_en/src/development/compatibility-matrix.md`
- `book_ar/src/development/compatibility-matrix.md`

## Impact Notes

- No intentional runtime behavior changes in the P0 docs-alignment portion.
- Public API cleanup may require import adjustments for consumers relying on removed wildcard prelude re-exports.
- Widget runtime warnings improve misconfiguration visibility without changing feature registration defaults.

## Commits Included

- `d3db281` docs: align P0 docs with code truth (plugins, layout schedule, book paths, Camera2d limits)
- `68180cb` docs(book): add plugin truth tables, interaction matrix, and limitations pages
- `84ba7ab` refactor(layout): rename geomerty module to geometry
- `68af63e` refactor(api): narrow layout prelude re-exports
- `58b414e` feat(widget): warn when optional widget plugins are missing
- `907c204` chore(testing): add serial release verification scripts
- `7d4ed82` refactor(widget): keep menu placeholder internal to reduce API noise
- `d411f48` docs(validation): add example report, compatibility matrix, and smoke plan
- `919cc77` docs(validation): mark runtime smoke phase as deferred in EN/AR reports
