# Alpha2 Status

This short note is for users who read the repository root files first and want a fast answer to one question: what should be treated as stable enough to build on during the current `alpha2` line?

This file is an operational alpha-line note. It is useful during the stabilization wave, but it is not part of the intended long-term root release file set.

## Where This File Fits

Use this file when you want the shortest stability-oriented answer.

Use the other root files for different goals:

- `README.md`: landing story and fastest overview
- `MIGRATION.md`: upgrade path from older docs, examples, and root assumptions
- `RELEASE_NOTES.md`: current alpha release-scale summary
- `changelog.md`: dated historical record
- `docs/src/index.md`: docs home and language entry point

## Intended Stable Surface For The Rest Of Alpha2

- `URootUi` is the canonical public root model.
- `URootUi::screen()`, `URootUi::world_2d(...)`, and `URootUi::world_3d(...)` are the intended public entry points.
- `UiSpace`, `UiCameraRef`, and `UiCanvasSize`, including `UiCanvasSize::FitContent { min, max }`, are part of the intended root story.
- screen roots behave as real HUD roots
- world roots use logical UI units plus `meters_per_unit`
- root capsules are part of the intended behavior, so separate roots do not visually or interactively leak over each other
- examples are expected to use package-aware commands and crate-owned example locations

## Transitional Surface

- `UScreenRoot` and `UWorldRoot` remain deprecated compatibility wrappers through the rest of `alpha2`
- legacy world-size parity via `meters_per_unit: 1.0` remains a compatibility tactic, not the preferred mental model
- some docs pages still mention compatibility wrappers for migration purposes, but they are no longer the primary example path

## Explicit Decision For This Alpha Line

- the deprecated root wrappers stay available for the remainder of `alpha2`
- they should not be used in new examples or new guide snippets
- they should not be removed immediately after the next alpha cut
- removal should be reconsidered only after the next alpha cut ships and one more stabilization review is complete

## Still Experimental Or Still Settling

- fit-content world roots under heavy `%` sizing or strongly root-relative flex layouts
- the more polished visual defaults of `World3d` showcase scenes
- any future escape hatch beyond closed root capsules

## Example Audit Snapshot

- the shipped runnable examples were re-audited for this stabilization pass
- the intended example path now centers on `URootUi`
- the root wrappers are documented for migration, not presented as the canonical route for new users

## Read Next

- migration summary: `MIGRATION.md`
- release notes: `RELEASE_NOTES.md`
- docs home: `docs/src/index.md`
- docs migration page: `docs/src/en/migration/alpha2-stability.md`
