# Roadmap - URootUi Refactor

## Goal

Replace the current split root model (`UScreenRoot` / `UWorldRoot`) with a single public root API,
make screen UI a real HUD, and separate UI layout units from world-space size.

## Why This Refactor Is Needed

The current design has several structural issues:

- `UScreenRoot` is described as screen-fixed HUD, but it currently behaves like a window-sized world canvas.
- `UWorldRoot` mixes root-space concerns with render-mode concerns through `is_3d`.
- `UVal::Px` and `ComputedSize` are documented in pixel terms, while world/3D rendering consumes the same values as world geometry size.
- Picking and some widget interaction paths are hardcoded around `Camera2d`.
- `UI3d` is propagated as mutable state instead of being derived from a resolved root context.

## Target Public API

```rust
#[derive(Component, Clone, Reflect)]
#[require(UNode)]
pub struct URootUi {
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}

#[derive(Clone, Copy, Reflect, PartialEq, Eq)]
pub enum UiSpace {
    Screen,
    World2d,
    World3d,
}

#[derive(Clone, Copy, Reflect)]
pub enum UiCanvasSize {
    Viewport,
    Fixed(Vec2),
}

#[derive(Clone, Copy, Reflect)]
pub enum UiCameraRef {
    Auto,
    Entity(Entity),
}
```

## Semantic Rules

- `UVal` stays a layout-unit type for the UI tree only.
- `URootUi.canvas` defines the logical canvas used by layout.
- `UiCanvasSize::Viewport` means "follow the target camera viewport".
- `UiCanvasSize::Fixed(Vec2)` means a fixed logical UI canvas.
- `meters_per_unit` defines world scaling only for world roots.
- `resolution_scale` affects visual quality, not physical world size.
- `Screen` means real HUD behavior.
- `World2d` means a world-space canvas drawn in 2D mode.
- `World3d` means a world-space canvas drawn in 3D mode.

## Non-Goals

- Do not redesign the whole layout solver in this task.
- Do not change widget APIs unless they depend on root semantics.
- Do not overload `UVal` to mean meters.
- Do not keep hidden behavior where "screen" still means "world canvas with window size".

## Phase 0 - Freeze The Design

- [x] Finalize `URootUi`, `UiSpace`, `UiCanvasSize`, and `UiCameraRef`.
- [x] Decide whether old roots stay temporarily as deprecated compatibility wrappers or are removed immediately in `alpha2`.
- [x] Decide the default `meters_per_unit` value for world roots.
- [x] Decide the `UiCameraRef::Auto` resolution rule when multiple cameras exist.

### Phase 0 Decisions

- [x] Freeze the public root shape exactly as documented in `Target Public API`.
- [x] Freeze the canonical constructors as:
  - `URootUi::screen()` -> `space: Screen`, `canvas: Viewport`, `camera: Auto`, `meters_per_unit: 0.001`, `resolution_scale: 1.0`
  - `URootUi::world_2d(size)` -> `space: World2d`, `canvas: Fixed(size)`, `camera: Auto`, `meters_per_unit: 0.001`, `resolution_scale: 1.0`
  - `URootUi::world_3d(size)` -> `space: World3d`, `canvas: Fixed(size)`, `camera: Auto`, `meters_per_unit: 0.001`, `resolution_scale: 1.0`
- [x] Keep `UScreenRoot` and `UWorldRoot` through the `alpha2` cycle as deprecated compatibility wrappers to reduce migration breakage.
- [x] Do not remove legacy roots before examples, docs, and migration notes have been updated to `URootUi`.
- [x] Freeze the default `meters_per_unit` value to `0.001` for world roots.
- [x] Freeze the meaning of `meters_per_unit` as world scaling only. It does not change logical layout size and is ignored by pure screen-space behavior.
- [x] Freeze `UiCameraRef::Auto` to a fail-closed rule:
  - resolve only when exactly one compatible active camera exists for the root
  - if no compatible camera exists, keep the root unresolved and emit a warning
  - if more than one compatible camera exists, keep the root unresolved and require `UiCameraRef::Entity`
- [x] Freeze multiple-camera support policy:
  - simple apps can rely on `Auto`
  - multi-camera apps must bind explicitly with `UiCameraRef::Entity`

## Phase 1 - Introduce Internal Root Resolution

Add an internal resolved root model so layout, render, and interaction all consume the same truth.

Suggested internal shape:

```rust
pub struct ResolvedRootUi {
    pub root_entity: Entity,
    pub space: UiSpace,
    pub canvas_size: Vec2,
    pub camera_entity: Option<Entity>,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}
```

- [x] Add `URootUi` to the engine public prelude.
- [x] Add an internal resolver system that computes `ResolvedRootUi`.
- [x] Resolve `canvas_size` from either viewport or fixed canvas settings.
- [x] Resolve the camera from `UiCameraRef`.
- [x] Make the resolved model the shared input for later systems.

Primary files:

- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/lib.rs`
- `crates/univis_ui_engine/src/layout/mod.rs`

## Phase 2 - Make Screen UI Real HUD

`Screen` roots must no longer behave like ordinary world-space roots with window-sized bounds.

- [x] Replace raw `Window.width()/height()` fallback for screen roots with viewport-aware size resolution.
- [x] Bind screen roots to the resolved target camera.
- [x] Ensure camera movement does not visually move screen UI.
- [x] Ensure camera zoom does not scale screen UI unexpectedly.
- [x] Ensure camera rotation does not rotate screen UI unexpectedly.
- [x] Define the transform policy for screen roots clearly and keep it stable.

Primary files:

- `crates/univis_ui_engine/src/layout/core/pass_down.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`

## Phase 3 - Remove Hardcoded Camera2d Assumptions

Interaction must use the root's resolved camera, not a global `Camera2d` assumption.

- [x] Refactor the picking backend to resolve the relevant camera from the target root.
- [x] Stop querying `With<Camera2d>` as the only picking source.
- [x] Update panel resize handling to use the resolved root camera.
- [x] Review text field and scroll interaction paths for the same issue.
- [x] Support multiple cameras without undefined selection order.

Primary files:

- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`
- Any widget/input systems that currently depend on `Camera2d`

## Phase 4 - Separate Space From Render Mode

The root type should describe where the UI lives. Rendering should follow from resolved space, not from `UWorldRoot.is_3d`.

- [x] Remove `is_3d` from the public root story.
- [x] Map `UiSpace::World3d` to the 3D material/render path.
- [x] Map `UiSpace::Screen` and `UiSpace::World2d` to the 2D material/render path unless explicitly expanded later.
- [x] Update render systems to use resolved root context instead of ad-hoc root queries.

Primary files:

- `crates/univis_ui_engine/src/layout/render/system.rs`
- `crates/univis_ui_engine/src/layout/render/mod.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`

## Phase 5 - Replace UI3d Propagation With Derived State

The engine should not rely on one-way propagation of `UI3d` from parents to children as the source of truth.

- [x] Audit every current use of `UI3d`.
- [x] Decide whether `UI3d` becomes a purely internal cached marker or a fully derived transient detail.
- [x] Ensure switching between `World2d` and `World3d` removes stale 3D state correctly.
- [x] Ensure newly spawned children inherit the correct resolved mode without relying on delayed propagation quirks.

Primary files:

- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/layout/render/system.rs`
- `crates/univis_ui_engine/src/layout/components.rs`

## Phase 6 - Normalize The Unit Model

Clarify that layout uses logical UI units, while world scaling is controlled separately.

- [x] Update `UVal` docs so `Px` means logical UI units, not literal display pixels.
- [x] Update `ComputedSize` docs to the same meaning.
- [x] Define how logical canvas size maps to world size as `world_size = canvas_size * meters_per_unit`.
- [x] Ensure mesh sizing and transforms use the resolved world scale in world modes.
- [x] Ensure `resolution_scale` stays independent from physical world size.

Primary files:

- `crates/univis_ui_engine/src/layout/geometry.rs`
- `crates/univis_ui_engine/src/layout/render/system.rs`
- Any text/image sizing systems that assume pixel semantics

## Phase 7 - Public API Migration

Introduce `URootUi` cleanly and remove ambiguity from public API surface.

- [x] Export `URootUi` from the public prelude.
- [x] Add `URootUi::screen()`.
- [x] Add `URootUi::world_2d(size)`.
- [x] Add `URootUi::world_3d(size)`.
- [x] Keep `UScreenRoot` as a deprecated compatibility wrapper for `URootUi::screen()` during `alpha2`.
- [x] Keep `UWorldRoot { size, is_3d: false }` as a deprecated compatibility wrapper for `URootUi::world_2d(size)` during `alpha2`.
- [x] Keep `UWorldRoot { size, is_3d: true }` as a deprecated compatibility wrapper for `URootUi::world_3d(size)` during `alpha2`.
- [x] Mark old roots deprecated with migration notes if kept temporarily.
- [x] Remove root-specific wording that no longer reflects behavior.

Primary files:

- `crates/univis_ui_engine/src/lib.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `README.md`

## Phase 8 - Update Examples

Every example should use the new root API and demonstrate the correct semantics.

- [x] Replace `UScreenRoot` usage with `URootUi::screen()`.
- [x] Replace `UWorldRoot` usage with `URootUi::world_2d(...)` or `URootUi::world_3d(...)`.
- [x] Add at least one example that proves real screen-fixed HUD behavior while the camera moves.
- [x] Add at least one example that proves fixed logical canvas plus explicit world scale.
- [x] Recheck examples that currently rely on `Camera2d` assumptions.

Primary directories:

- `examples/`

## Phase 9 - Update Documentation

The docs must stop teaching the broken model.

- [x] Rewrite root docs in both books.
- [x] Update quick-start snippets.
- [x] Update the support matrix to distinguish `Screen`, `World2d`, and `World3d`.
- [x] Add a migration note for `alpha2`.
- [x] Document logical UI units.
- [x] Document viewport canvas semantics.
- [x] Document world scaling through `meters_per_unit`.

Primary files:

- `README.md`
- `book_en/src/layout/roots.md`
- `book_ar/src/layout/roots.md`
- `book_en/src/quick-start.md`
- `book_ar/src/quick-start.md`
- `book_en/src/interaction/support-matrix.md`
- `book_ar/src/interaction/support-matrix.md`
- `changelog.md`

## Phase 10 - Verification

This refactor needs behavior-level verification, not only compile success.

### Engine Tests

- [x] Verify root size resolution for viewport canvas, fixed canvas, and fallback behavior.
- [x] Verify layout correctness under the new root resolution path.
- [x] Verify switching between `World2d` and `World3d`.
- [x] Verify new child insertion under each root mode.

### Interaction Tests

- [x] Verify picking under `Screen`.
- [x] Verify picking under `World2d`.
- [x] Verify picking under `World3d` with the intended supported camera path.
- [x] Verify panel resize behavior after removing `Camera2d` hardcoding.
- [x] Verify clipping-aware hit testing after the root refactor.

### Behavior Validation

- [x] Move the camera and verify screen UI remains visually fixed.
- [x] Zoom the camera and verify screen UI remains visually fixed.
- [x] Rotate the camera and verify screen UI remains visually fixed.
- [x] Verify world UI remains attached to world transforms.
- [x] Verify `meters_per_unit` changes resize world UI physically without changing logical layout.

### Release Validation

- [x] Run `cargo check --workspace`.
- [x] Run targeted tests for engine and interaction crates.
- [x] Run example smoke checks for screen, world, and 3D examples.
- [x] Run manual visual validation for HUD correctness.

## Recommended Delivery Order

To reduce breakage during `alpha2`, implement in this order:

- [ ] Add `URootUi` and internal `ResolvedRootUi`.
- [ ] Fix real screen HUD behavior.
- [ ] Remove hardcoded camera assumptions from picking and panel resize.
- [ ] Separate render mode from root identity.
- [ ] Normalize units and world scaling.
- [ ] Migrate examples and docs.
- [ ] Remove or deprecate legacy roots.

## Done Criteria

This task is complete only when all of the following are true:

- [ ] `UScreenRoot` is no longer the source of misleading HUD semantics.
- [ ] `URootUi` is the single public root entry point.
- [ ] Screen UI is actually screen-fixed.
- [ ] World roots use explicit logical canvas sizing.
- [ ] World scaling is controlled explicitly instead of being implied by `UVal::Px`.
- [ ] Picking no longer depends on a global `Camera2d` assumption.
- [ ] The docs match the code.
- [ ] Examples demonstrate the intended behavior clearly.

## Optional Follow-Up

After the main refactor is stable:

- [ ] Consider removing `UI3d` from the public surface entirely if it remains purely internal.
- [ ] Consider adding debug overlays for resolved root context, including root kind, camera binding, canvas size, and world size.
- [ ] Consider adding a dedicated migration page for `0.2.0-alpha.2`.
