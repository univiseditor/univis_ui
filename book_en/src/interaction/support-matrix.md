# Interaction Support Matrix (Screen / World / 3D)

Legend:
- `Supported`: works in the documented path.
- `Partial`: works with constraints.
- `N/A`: capability is not intended for that mode.

| Capability | Screen UI (`UScreenRoot`) | World UI (`UWorldRoot`, `is_3d = false`) | 3D UI (`UWorldRoot`, `is_3d = true`) | Conditions / Notes |
|---|---|---|---|---|
| Base rendering | Supported | Supported | Supported | 3D path uses `UI3d` propagation and `UNodeMaterial3d`. |
| Picking + pointer events | Supported | Supported | Partial | Current picking backend queries `Camera2d`; 3D-camera-only scenes are not covered by this path. |
| Clipping-aware hit testing | Supported | Supported | Partial | Ancestor clipping checks run in picking backend; same camera constraint applies. |
| `UPanelWindow` resize handles | Supported | Supported | Partial | Resize path currently queries `Camera2d`. |
| `UTextField` input/events | Supported | Supported | Partial | Requires `UnivisTextFieldPlugin`; interaction still follows `Camera2d` picking path. |
| `UScrollContainer` interaction | Supported | Supported | Partial | Scroll plugin is auto-registered; interaction follows `Camera2d` picking path. |
| `UPbr` controls (`metallic`, `roughness`, `emissive`) | N/A | N/A | Supported | Intended for `UI3d` render path. |

## Verification Sources

- `src/interaction/picking.rs`
- `src/widget/panel.rs`
- `src/layout/layout_system.rs`
- `src/layout/render/mod.rs`
