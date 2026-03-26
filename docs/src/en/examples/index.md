# Examples

Examples now live next to the crate they best represent. Run them with:

```bash
cargo run -p <package> --example <name>
```

## `univis_ui` facade examples

- `hello_world`: the minimal starting point
- `complex_dashboard`: integrated widget-heavy dashboard
- `card_profile`: polished facade-level profile card
- `sci_fi`: larger full-stack sci-fi HUD

## `univis_ui_engine` examples

- `alignment`
- `border_light_3d`
- `ex_node`
- `layout_cache`
- `layout_case_alignment_overflow`
- `layout_case_flex_wrap`
- `layout_case_grid_auto_flow`
- `layout_case_grid_tracks`
- `layout_case_masonry_ext`
- `layout_case_radial`
- `layout_case_stack`
- `layout_solver_no_widgets`
- `layout_solver_ultra_complex`
- `masonry`
- `root_screen_hud`
- `root_world_scale`
- `root_capsule_overlap`
- `root_fit_content`
- `texture`

## `univis_ui_widgets` examples

- `drag_value`
- `panel_divider`
- `panel_window`
- `radio`
- `scroll_view`
- `seekbar`
- `select`
- `text_field`
- `text_label`
- `text_label_zoom`
- `toggle`
- `widgets`

## `univis_ui_interaction` examples

- `interaction`

## Suggested commands

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui_engine --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui_interaction --example interaction
```

## Suggested Learning Order

1. start with `hello_world`
2. move to `text_label`, `widgets`, and `interaction`
3. explore the `layout_case_*` and root examples for engine semantics
4. finish with `sci_fi` or `card_profile` for larger compositions

## Related Reports

- [Example Validation Report](../development/example-validation.md)
