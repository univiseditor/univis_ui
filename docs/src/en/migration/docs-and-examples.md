# Docs and Examples Surface Migration

This page is for users who previously learned the project through older README-only material,
split books, or runnable workspace examples that are no longer shipped in this branch.

## What Changed

- the docs now live in one bilingual mdBook under `docs/`
- English and Arabic chapters are mirrored under one shared `SUMMARY.md`
- the examples catalog now tracks availability, not only ownership
- the current live demo package is `android/android_phone_app`
- root guidance now centers on `URootUi`

## Old Discovery Path vs New Discovery Path

Old flow:

- README
- scattered runnable examples
- split `book_en` / `book_ar`
- broad assumptions about internal modules

New flow:

1. `README.md` or `README_AR.md`
2. `docs/src/index.md`
3. the relevant guide chapter
4. the canonical availability entry in `docs/src/*/examples/index.md`
5. generated `rustdoc` for exact type paths and signatures

## Example Commands In This Branch

Older commands such as `cargo run --example hello_world` or
`cargo run -p univis_ui --example root_fit_content` are historical only here.

Use these instead:

- `cargo run --manifest-path android/android_phone_app/Cargo.toml` for the current live Android-style demo
- [Examples](../examples/index.md) for ownership/history and archived names
- [Example Gallery](../examples/gallery.md) for curated live/static/archived status

## Root Docs Migration

If you previously learned the root model through `UScreenRoot` or `UWorldRoot`, use these pages now:

- [Root API Migration to `URootUi`](root-api.md)
- [Roots and Spaces](../layout/roots.md)

## Canonical Sources Now

- landing story: `README.md` / `README_AR.md`
- guides: `docs/src/en/*` and `docs/src/ar/*`
- example availability map: `docs/src/en/examples/index.md` and `docs/src/ar/examples/index.md`
- API reference: generated `cargo doc --no-deps -p univis_ui`
