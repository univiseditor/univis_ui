# Example Path Migration

Examples now live next to the crate they primarily represent.

## Command Migration

Older commands often looked like this:

```bash
cargo run --example root_fit_content
```

Use package-aware commands now:

```bash
cargo run -p univis_ui --example root_fit_content
```

## Package Ownership Rules

- `examples/` at the workspace root holds facade demos plus cross-crate layout/root examples that need widgets or interaction
- `crates/univis_ui_engine/examples/` holds engine, layout, and rendering-focused examples that stay pure to the engine boundary
- `crates/univis_ui_widgets/examples/` holds widget-focused examples
- `examples/interaction.rs` in the root package holds the interaction-focused example

## Common Migrations

- `root_*` examples -> `univis_ui`
- `layout_case_*` examples -> `univis_ui`
- `alignment`, `ex_node`, `layout_cache`, `layout_sizing_semantics`, `runtime_benchmarks` -> `univis_ui`
- `text_*`, `panel_*`, `scroll_view`, `select`, `toggle` -> `univis_ui_widgets`
- `interaction` -> `univis_ui_interaction`
- `hello_world`, `card_profile`, `sci_fi`, `complex_dashboard` -> `univis_ui`

## Related Guides

- [Examples](../examples/index.md)
- [Testing and Validation](../development/testing.md)
- [Smoke Test Plan](../development/smoke-test-plan.md)
