# Frame Lifecycle

This chapter connects the main systems across a single Bevy frame.

## `PreUpdate`

- `univis_picking_backend`
  - converts pointer position into world space
  - evaluates SDF intersection against interactive entities
  - respects ancestor clipping through `UClip`
  - emits `PointerHits` for Bevy picking observers

## `Update`

- widget systems update state, sync visuals, and emit messages
- scroll behavior, panel resize, and similar interaction systems run here
- text systems refresh text layout inputs

## `PostUpdate`

Critical layout order:

1. `update_layout_hierarchy`
2. `upward_measure_pass_cached`
3. `downward_solve_pass_safe`

After layout:

- `update_materials_optimized` syncs `UNode`, `ComputedSize`, `UBorder`, and related data into materials

## Why The Order Matters

- any `Update` change, such as panel resize or input state, must be reflected before the frame is rendered
- cache and invalidation run before measurement to avoid unnecessary work
- rendering depends on final `ComputedSize` and resolved transforms
