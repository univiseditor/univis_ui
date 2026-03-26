# Materials and Shaders

## `UNodeMaterial` (2D)

Important fields include:

- fill color
- border color
- size
- corner radius
- clip rectangle data

## `UNodeMaterial3d`

The 3D path adds PBR-related properties:

- metallic
- roughness
- emissive

## 2D Shader

In `unode.wgsl`:

- the base shape is evaluated as an SDF
- body and border masks are computed
- if `use_clip = 1`, the clip mask is applied before final color output

## 3D Shader

In `unode_3d.wgsl`:

- the same SDF shape logic is used
- the result is integrated into the 3D pipeline
