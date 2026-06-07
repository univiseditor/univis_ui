# Roots And Spaces

`URootUi` is the single public root API.
It resolves each UI tree into one of three spaces:

- `UiSpace::Screen`
- `UiSpace::World2d`
- `UiSpace::World3d`

## Root Shape

```rust
#[derive(Component, Clone, Reflect)]
pub struct URootUi {
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}
```

## Screen

Use:

```rust
URootUi::screen()
```

Semantics:

- Resolves against the target camera viewport, not raw `Window.width()/height()`.
- Behaves as a real HUD.
- Camera translation, zoom, and rotation should not visually move screen UI.
- Forms a closed stacking capsule: local ordering never escapes above another root.
- Best for menus, overlays, HUDs, and cursor-fixed elements.

## World2d

Use:

```rust
URootUi::world_2d(Vec2::new(1280.0, 720.0))
```

or:

```rust
URootUi::world_2d_fit_content()
```

Semantics:

- Supports either a fixed logical canvas or a content-driven one.
- Lives in world space.
- Renders through the flat 2D material path.
- Forms a closed stacking capsule: descendants stay inside the root's visual layer.
- Best for panels, diegetic monitors, and boards attached to the scene.

## World3d

Use:

```rust
URootUi::world_3d(Vec2::new(1280.0, 720.0))
```

or:

```rust
URootUi::world_3d_fit_content()
```

Semantics:

- Uses the same logical canvas model as `World2d`, including fit-content sizing.
- Lives in world space.
- Renders through the 3D material path.
- Forms a closed stacking capsule relative to other UI roots.
- `UPbr` settings apply here.

## Logical Canvas And Units

`UVal` remains a layout-unit type for the UI tree.

- `UVal::Px(f32)` means logical UI units.
- `UiCanvasSize::Viewport` means "follow the resolved camera viewport".
- `UiCanvasSize::Fixed(Vec2)` means a fixed logical canvas for layout.
- `UiCanvasSize::FitContent { min, max }` means "measure the logical canvas from content, then clamp it".

For world roots, physical size is derived explicitly:

```text
world_size = canvas_size * meters_per_unit
```

This means:

- layout stays in logical UI units
- `meters_per_unit` controls only physical world size
- `resolution_scale` controls visual quality independently from world size
- fit-content roots work best when their direct content has intrinsic or fixed sizing; heavy `%` or root-relative flex can introduce circular expectations

## Camera Resolution

`UiCameraRef::Auto` is convenient for simple scenes with exactly one compatible camera.

For multi-camera scenes, prefer:

```rust
UiCameraRef::Entity(camera_entity)
```

That keeps viewport resolution, interaction, and screen-root anchoring deterministic.

## Legacy Compatibility Note

`UScreenRoot` and `UWorldRoot` are deprecated compatibility wrappers.

They are explicit-only migration paths, and the current removal target is the
first alpha after `0.3.0` that no longer needs wrapper-based migration
support.

Migration rules:

- `UScreenRoot` -> `URootUi::screen()`
- `UWorldRoot { size, is_3d: false }` -> `URootUi::world_2d(size)`
- `UWorldRoot { size, is_3d: true }` -> `URootUi::world_3d(size)`

If you need a specific world-space physical size, set:

```rust
URootUi {
    meters_per_unit: 1.0,
    ..URootUi::world_2d(size)
}
```

or:

```rust
URootUi {
    meters_per_unit: 1.0,
    ..URootUi::world_3d(size)
}
```

## Related Examples

- [`responsive_layout_test`](../examples/index.md#workspace-examples)
- [`z_order_hierarchy`](../examples/index.md#workspace-examples)
- [`layout_stack`](../examples/index.md#layout-modes)

## Related Migration Notes

- [Root API Migration to `URootUi`](../migration/root-api.md)
- [Legacy Compatibility Status](../migration/legacy-compatibility.md)

## API Entry Points

- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::layout_system::UiSpace`
- `univis_ui_engine::layout::layout_system::UiCanvasSize`
- `univis_ui_engine::layout::layout_system::UiCameraRef`
- `univis_ui_engine::layout::pbr::UPbr`

## Where To Look Next

- related example: [`responsive_layout_test`](../examples/index.md#workspace-examples)
- related API index: [API Reference](../api/index.md)
- related migration page: [Root API Migration to `URootUi`](../migration/root-api.md)
