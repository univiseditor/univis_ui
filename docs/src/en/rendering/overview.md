# Rendering

Univis rendering is built on custom SDF materials:

- `UNodeMaterial` for the 2D path
- `UNodeMaterial3d` for the 3D path

Main files:

- `src/layout/render/system.rs`
- `src/layout/render/material.rs`
- `src/layout/render/material_3d.rs`
- `src/layout/render/shaders/unode.wgsl`
- `src/layout/render/shaders/unode_3d.wgsl`

## Update Path

The render sync system:

- reads `UNode`, `ComputedSize`, `UBorder`, `UImage`, `UI3d`, and `UPbr`
- chooses the 2D or 3D path from the resolved root context
- reuses `MaterialHandles` to reduce allocations
- passes clipping data to the 2D material path

## Why SDF?

- clean edges under zoom
- good support for border radius, clipping, and antialiasing
- flexibility for both `Round` and `Cut` shapes

## Related Examples

- [`border_light_3d`](../examples/index.md#border_light_3d) in `univis_ui_engine`
- `card_profile` in `univis_ui`
- `sci_fi` in `univis_ui`
