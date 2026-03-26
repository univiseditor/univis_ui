# Root API Migration to `URootUi`

`URootUi` is now the canonical public root API.

## Old to New Mapping

- `UScreenRoot` -> `URootUi::screen()`
- `UWorldRoot { size, is_3d: false }` -> `URootUi::world_2d(size)`
- `UWorldRoot { size, is_3d: true }` -> `URootUi::world_3d(size)`

## Important Semantic Changes

- `URootUi::screen()` is a real viewport-fixed HUD root.
- `URootUi` roots are closed stacking capsules.
- `UVal::Px` means logical UI units, not literal monitor pixels.
- world-space physical size is controlled by `meters_per_unit`.

## Legacy Compatibility Notes

- `UScreenRoot` and `UWorldRoot` remain deprecated compatibility wrappers during `alpha2`.
- If you need the historical world-root size from older examples, set `meters_per_unit: 1.0` explicitly.

## Related Guides

- [Roots and Spaces](../layout/roots.md)
- [Quick Start](../quick-start.md)
- [Examples](../examples/index.md#featured-learning-path)
