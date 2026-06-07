# Legacy Compatibility Status

This page explains which migration-era compatibility paths are still intentionally available, which ones are already de-emphasized, and which ones are expected to stay long-term.

## Public Classification

| Item | Current Status | Recommended Path | Planned Direction |
|---|---|---|---|
| `URootUi`, `URootUi::screen()`, `URootUi::world_2d(...)`, `URootUi::world_3d(...)` | Canonical public API | keep using them directly | keep long-term |
| `UiSpace`, `UiCameraRef`, `UiCanvasSize` | Canonical support types | keep using them directly | keep long-term |
| `UScreenRoot` | Deprecated explicit-only wrapper | `URootUi::screen()` | de-emphasize now; planned removal target is the first alpha after `0.3.0` that no longer needs wrapper-based migration support |
| `UWorldRoot` | Deprecated explicit-only wrapper | `URootUi::world_2d(size)` or `URootUi::world_3d(size)` | de-emphasize now; planned removal target is the first alpha after `0.3.0` that no longer needs wrapper-based migration support |
| `meters_per_unit = 1.0` on `URootUi` | Explicit compatibility knob on the canonical root API | keep it only when you need the historical world-root physical size | keep long-term as an opt-in control, not as the default story |

## Internal Compatibility Translation

These items are not the recommended public story, but they still exist internally while the alpha surface settles:

- legacy root wrappers are translated into the canonical root-resolution input before camera and canvas resolution run
- legacy scalar alignment fallback is translated toward the ext alignment model before placement decisions
- legacy main-axis flex fallback remains an internal compatibility path while canonical grow/shrink fields stay preferred

## Maintainer Rules

- do not add `UScreenRoot` or `UWorldRoot` to recommended preludes
- do not add new examples that depend on deprecated root wrappers
- keep migration docs and README wording aligned with this page before each alpha cut

## Planned Removal Window

- current target: remove `UScreenRoot` and `UWorldRoot` in the first alpha after `0.3.0` if migration feedback stays clean
- reevaluate that target before the cut if published examples, migration guides, or user reports still depend on the wrappers

## Related

- [Root API Migration to `URootUi`](root-api.md)
- [Roots and Spaces](../layout/roots.md)
- [Current Limitations](../development/current-limitations.md)
