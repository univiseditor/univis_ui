# Docs and Examples Surface Migration

This page is for users who previously learned the project through older README-only material, split books, or root-level example assumptions.

## What Changed

- the docs now live in one bilingual mdBook under `docs/`
- English and Arabic chapters are mirrored under one shared `SUMMARY.md`
- examples now live next to the crate they primarily represent
- package-aware example commands are now the default
- root guidance now centers on `URootUi`

## Old Discovery Path vs New Discovery Path

Old flow:

- README
- scattered examples
- split `book_en` / `book_ar`
- broad assumptions about internal modules

New flow:

1. `README.md` or `README_AR.md`
2. `docs/src/index.md`
3. the relevant guide chapter
4. the canonical example entry in `docs/src/*/examples/index.md`
5. generated `rustdoc` for exact type paths and signatures

## Example Command Migration

Older example commands often assumed the workspace root package:

```bash
cargo run --example hello_world
```

Now examples should be run through the owning package:

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui --example interaction
```

## Root Docs Migration

If you previously learned the root model through `UScreenRoot` or `UWorldRoot`, use these pages now:

- [Root API Migration to `URootUi`](root-api.md)
- [Roots and Spaces](../layout/roots.md)

## Canonical Sources Now

- landing story: `README.md` / `README_AR.md`
- guides: `docs/src/en/*` and `docs/src/ar/*`
- example catalog: `docs/src/en/examples/index.md` and `docs/src/ar/examples/index.md`
- API reference: generated `cargo doc --no-deps -p univis_ui`
