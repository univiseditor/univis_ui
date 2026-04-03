# Migration Summary

This file replaces the temporary docs/examples/API roadmap files now that the migration work has been applied to the repository.

## When To Read This File

Read `MIGRATION.md` when you are coming from older docs, older example paths, or older root assumptions.

Use the other root files for different goals:

- `README.md`: landing story and fastest overview
- `README_AR.md`: Arabic landing story
- `RELEASE_NOTES.md`: current alpha release summary
- `changelog.md`: chronological change history

## What Changed

- documentation now lives under one bilingual `mdBook` in `docs/`
- English and Arabic chapters are mirrored under one shared `docs/src/SUMMARY.md`
- examples now live next to the crate that primarily owns them
- package-aware example commands are now the default
- generated `API Docs` are now part of the documented learning path
- the public root model centers on `URootUi`
- layout sizing now has explicit `min_width/max_width/min_height/max_height`, `MinContent`, and `MaxContent`
- `UVal::Auto` is no longer treated as the same thing as `Content`

## Sizing Migration

If you built layouts against older alpha behavior, recheck these assumptions:

- `UVal::Content` now means explicit max-content sizing.
- `UVal::Auto` now means contextual sizing instead of silently collapsing into `Content`.
- `UImage` resolves `Auto`, `Content`, `MinContent`, and `MaxContent` to the native texture size when the asset becomes available.

Start from these pages:

- `docs/src/en/layout/sizing-semantics.md`
- `docs/src/en/layout/node-model.md`
- `docs/src/en/examples/index.md`

## New Discovery Path

Use this order when navigating the project:

1. `README.md` or `README_AR.md`
2. `docs/src/index.md`
3. the relevant guide chapter
4. `docs/src/en/examples/index.md` or `docs/src/ar/examples/index.md`
5. generated `cargo doc --no-deps -p univis_ui`

## Example Command Migration

Older commands often assumed the workspace root package:

```bash
cargo run --example hello_world
```

Use package-aware commands now:

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui_engine --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui_interaction --example interaction
```

## Root And Docs Migration

If you learned the project through older docs or older root wrappers, start here:

- `docs/src/en/migration/docs-and-examples.md`
- `docs/src/en/migration/root-api.md`
- `docs/src/en/migration/example-paths.md`

Arabic mirrors:

- `docs/src/ar/migration/docs-and-examples.md`
- `docs/src/ar/migration/root-api.md`
- `docs/src/ar/migration/example-paths.md`

## Source Of Truth

- project story and crate map: `README.md`
- Arabic landing page: `README_AR.md`
- alpha2 stability note: `ALPHA2_STATUS.md`
- guides: `docs/src/en/*` and `docs/src/ar/*`
- example catalog: `docs/src/en/examples/index.md` and `docs/src/ar/examples/index.md`
- API reference: generated `cargo doc --no-deps -p univis_ui`

## Why The Roadmap Files Were Removed

The roadmap checklists were useful while the work was in progress, but they are now historical implementation notes. The repository now keeps a migration summary instead of open-ended planning files at the root.
