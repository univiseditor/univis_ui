# Example Validation Report

## Validation Date

- March 6, 2026

## Validation Mode

- Command style: sequential, release profile
- Resource guard: `CARGO_BUILD_JOBS=1`
- Command used:

```bash
./scripts/check_examples_serial_release.sh
```

## Result

- Total examples checked: 28
- Passed: 28
- Failed: 0

## Checked Examples

- `alignment`
- `border_light_3d`
- `card_profile`
- `drag_value`
- `ex_node`
- `hello_world`
- `interaction`
- `layout_cache`
- `layout_case_alignment_overflow`
- `layout_case_flex_wrap`
- `layout_case_grid_auto_flow`
- `layout_case_grid_tracks`
- `layout_case_masonry_ext`
- `layout_case_radial`
- `layout_case_stack`
- `masonry`
- `panel_divider`
- `panel_window`
- `radio`
- `sci_fi`
- `scroll_view`
- `seekbar`
- `select`
- `text_field`
- `text_label`
- `texture`
- `toggle`
- `widgets`

## Notes

- This report confirms compile viability via `cargo check --release --example ...`.
- Runtime behavior still requires manual windowed smoke checks for interaction and visual correctness.

## Runtime Smoke Status

- March 6, 2026: Deferred/skipped for this cycle by request (machine/resource constraints).
- Planned smoke command set remains documented in [Smoke Test Plan](smoke-test-plan.md).
