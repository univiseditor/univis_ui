# Alpha2 Stability

This page freezes the current documentation-facing interpretation of the `alpha2` line.

## Stable Enough To Build On

- `URootUi` is the canonical root API.
- `screen`, `world_2d`, and `world_3d` are the intended public root spaces.
- `UiCanvasSize::FitContent { min, max }` is part of the intended world-root sizing model.
- screen roots are real viewport-fixed HUD roots.
- root capsules are the intended cross-root stacking rule.
- examples are expected to use crate-owned paths and package-aware commands.

## Transitional But Still Supported During Alpha2

- `UScreenRoot` and `UWorldRoot` remain available as deprecated compatibility wrappers.
- compatibility notes around `meters_per_unit: 1.0` remain relevant only for preserving the historical size of older world-space examples.
- migration pages may still mention the wrappers because older users need a clear translation path.

## Explicit Decision

The compatibility wrappers stay through the rest of `alpha2`. They are not meant for new examples, new guide snippets, or new user-facing patterns, but they are not scheduled for removal before the next alpha cut.

## Still Experimental

- fit-content roots under strongly circular sizing relationships
- some of the more polished `World3d` showcase defaults
- any future feature that would intentionally escape the closed root-capsule model

## Example Audit Result

The current shipped examples were re-audited for this phase. The intended example path now uses `URootUi` directly; the wrapper types are part of migration guidance, not the canonical teaching story.

## Related Pages

- [Migration and Limitations](index.md)
- [Root API Migration to `URootUi`](root-api.md)
- [Examples](../examples/index.md)
- [Example Gallery](../examples/gallery.md)
