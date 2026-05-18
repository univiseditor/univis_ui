# Docs and Examples Surface Migration

This page explains the current documentation and example layout.

## What Changed

- the docs live in one bilingual mdBook under `docs/`
- English and Arabic chapters are mirrored under one shared `SUMMARY.md`
- the examples catalog lists only source files that exist in this branch
- the standalone demo package is `android/android_phone_app`
- root guidance centers on `URootUi`

## Current Discovery Path

1. `README.md` or `README_AR.md`
2. `docs/src/index.md`
3. the relevant guide chapter
4. the current example list in `docs/src/*/examples/index.md`
5. generated `rustdoc` for exact type paths and signatures

## Example Commands In This Branch

Use these commands for the current working tree:

- `cargo run --manifest-path android/android_phone_app/Cargo.toml`
- `cargo check --workspace --examples`
- `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets`

For a task-oriented view, see:

- [Examples](../examples/index.md)
- [Example Gallery](../examples/gallery.md)

## Root Docs Migration

If you previously learned the root model through `UScreenRoot` or `UWorldRoot`, use these pages now:

- [Root API Migration to `URootUi`](root-api.md)
- [Roots and Spaces](../layout/roots.md)

## Canonical Sources Now

- landing story: `README.md` / `README_AR.md`
- guides: `docs/src/en/*` and `docs/src/ar/*`
- current examples: `docs/src/en/examples/index.md` and `docs/src/ar/examples/index.md`
- API reference: generated `cargo doc --no-deps -p univis_ui`
