# Example Path Migration

Examples now live next to the crate they primarily represent.

## Command Migration

Older commands often looked like this:

```bash
cargo run --example root_fit_content
```

Use package-aware commands now:

```bash
cargo run -p univis_ui_engine --example root_fit_content
```

## Package Ownership Rules

- `examples/` at the workspace root is reserved for facade-level integrated demos
- `crates/univis_ui_engine/examples/` holds engine, root, layout, and rendering-focused examples
- `crates/univis_ui_widgets/examples/` holds widget-focused examples
- `crates/univis_ui_interaction/examples/` holds interaction-focused examples

## Common Migrations

- `root_*` examples -> `univis_ui_engine`
- `layout_case_*` examples -> `univis_ui_engine`
- `text_*`, `panel_*`, `scroll_view`, `select`, `toggle` -> `univis_ui_widgets`
- `interaction` -> `univis_ui_interaction`
- `hello_world`, `card_profile`, `sci_fi`, `complex_dashboard` -> `univis_ui`

## Related Guides

- [Examples](../examples/index.md)
- [Testing and Validation](../development/testing.md)
- [Smoke Test Plan](../development/smoke-test-plan.md)
