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

Semantics:

- Uses a fixed logical canvas for layout.
- Lives in world space.
- Renders through the flat 2D material path.
- Forms a closed stacking capsule: descendants stay inside the root's visual layer.
- Best for panels, diegetic monitors, and boards attached to the scene.

## World3d

Use:

```rust
URootUi::world_3d(Vec2::new(1280.0, 720.0))
```

Semantics:

- Uses the same fixed logical canvas model as `World2d`.
- Lives in world space.
- Renders through the 3D material path.
- Forms a closed stacking capsule relative to other UI roots.
- `UPbr` settings apply here.

## Logical Canvas And Units

`UVal` remains a layout-unit type for the UI tree.

- `UVal::Px(f32)` means logical UI units.
- `UiCanvasSize::Viewport` means "follow the resolved camera viewport".
- `UiCanvasSize::Fixed(Vec2)` means a fixed logical canvas for layout.

For world roots, physical size is derived explicitly:

```text
world_size = canvas_size * meters_per_unit
```

This means:

- layout stays in logical UI units
- `meters_per_unit` controls only physical world size
- `resolution_scale` controls visual quality independently from world size

## Camera Resolution

`UiCameraRef::Auto` is convenient for simple scenes with exactly one compatible camera.

For multi-camera scenes, prefer:

```rust
UiCameraRef::Entity(camera_entity)
```

That keeps viewport resolution, interaction, and screen-root anchoring deterministic.

## Alpha2 Migration Note

`UScreenRoot` and `UWorldRoot` are deprecated compatibility wrappers during `alpha2`.

Migration rules:

- `UScreenRoot` -> `URootUi::screen()`
- `UWorldRoot { size, is_3d: false }` -> `URootUi::world_2d(size)`
- `UWorldRoot { size, is_3d: true }` -> `URootUi::world_3d(size)`

If you need the historical world-space physical size from legacy examples during `alpha2`,
set:

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
