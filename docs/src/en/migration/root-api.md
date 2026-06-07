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

- `UScreenRoot` and `UWorldRoot` remain deprecated compatibility wrappers.
- They are explicit-only migration paths and are no longer part of the recommended prelude story.
- The current removal target is the first alpha after `0.3.0` that no longer needs wrapper-based migration support.
- Set `meters_per_unit` explicitly when you need a specific physical world-root size.

## Related Guides

- [Legacy Compatibility Status](legacy-compatibility.md)
- [Roots and Spaces](../layout/roots.md)
- [Quick Start](../quick-start.md)
- [Examples](../examples/index.md)
