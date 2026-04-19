# Example Path Migration

This page is now mostly historical.

Earlier alpha branches moved examples next to the crate that primarily owned them. The current
branch no longer ships most of those source trees, so use this page as a compatibility glossary
for older issues, PRs, and docs references.

## Current Branch Reality

- live demo package: `android/android_phone_app`
- static references: `docs/src/assets/visual-references/`
- archived ownership map: [Examples](../examples/index.md)

## Historical Ownership Rules

- root `examples/` used to hold facade demos plus cross-crate layout/root scenes
- `crates/univis_ui_engine/examples/` used to hold engine/layout/render-focused demos
- `crates/univis_ui_widgets/examples/` used to hold widget-focused demos
- `examples/interaction.rs` used to hold the interaction-focused scene

## Historical Mappings

- `root_*` examples -> `univis_ui`
- `layout_case_*` examples -> `univis_ui`
- `alignment`, `ex_node`, `layout_cache`, `layout_sizing_semantics`, `runtime_benchmarks` -> `univis_ui`
- `text_*`, `panel_*`, `scroll_view`, `select`, `toggle` -> `univis_ui_widgets`
- `interaction` -> `univis_ui_interaction`
- `hello_world`, `card_profile`, `sci_fi`, `complex_dashboard` -> `univis_ui`

Treat the mapping above as historical context, not as a guarantee that the source files are still present.

## Related Guides

- [Examples](../examples/index.md)
- [Testing and Validation](../development/testing.md)
- [Smoke Test Plan](../development/smoke-test-plan.md)
