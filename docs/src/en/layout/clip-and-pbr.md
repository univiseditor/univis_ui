# `UClip` and `UPbr`

## `UClip`

`UClip { enabled: bool }` clips descendants.

Typical usage:

```rust,no_run
UClip { enabled: true }
```

### Where Clipping Is Used

- in `interaction/picking` to reject hits outside clip bounds
- in `render/system` to pass clip data into `UNodeMaterial`
- in text through local glyph clipping in `sync_text_label_meshes` and ancestor material clipping in `sync_text_clipper_materials`

## `UPbr`

For 3D UI roots:

- `UPbr` configures metallic, roughness, and emissive properties
- the settings affect the `UNodeMaterial3d` path
