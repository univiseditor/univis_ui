# Migration Summary

This file explains the current repository layout after the docs and examples cleanup.

## Current Branch Status

- documentation lives in one bilingual `mdBook` under `docs/`
- the public root model centers on `URootUi`
- the examples catalog lists only source files that exist in this branch
- workspace examples live under `examples/` and are registered in `Cargo.toml` when needed
- the standalone Android-style demo package is `android/android_phone_app`
- static visual references for archived showcases live under `docs/src/assets/visual-references/`

## Current Example Entry Points

Treat outdated example commands from older docs as historical only in this branch.

Run the Android-style demo:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

Check every workspace example known to Cargo:

```bash
cargo check --workspace --examples
```

Browse the current example list:

- `docs/src/en/examples/index.md`
- `docs/src/ar/examples/index.md`
- `docs/src/en/examples/gallery.md`
- `docs/src/ar/examples/gallery.md`

## Canonical Public Surface

Use these as the current public assumptions:

- canonical roots: `URootUi`, `URootUi::screen()`, `URootUi::world_2d(...)`, `URootUi::world_3d(...)`
- canonical root support types: `UiSpace`, `UiCameraRef`, `UiCanvasSize`
- deprecated compatibility wrappers: `UScreenRoot`, `UWorldRoot` on explicit paths only
- physical world sizing is controlled by `meters_per_unit` on `URootUi`

Still worth rechecking during migration:

- fit-content world roots under heavy relative sizing
- visual behavior in `World3d`-style scenes
- rendering-, layout-, or picking-heavy changes that still need manual validation

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
- current example list: `docs/src/en/examples/index.md` and `docs/src/ar/examples/index.md`
- API reference: generated `cargo doc --no-deps -p univis_ui`
- release-readiness roadmap for the stable `0.3.x` line: `ROADMAP.md`
