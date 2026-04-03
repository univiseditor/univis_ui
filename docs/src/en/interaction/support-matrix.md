# Interaction Support Matrix (Screen / World2d / World3d)

Legend:

- `Supported`: works in the documented path.
- `Partial`: works with constraints.
- `N/A`: capability is not intended for that mode.

| Capability | Screen UI (`URootUi::screen()`) | World UI (`URootUi::world_2d(...)`) | 3D UI (`URootUi::world_3d(...)`) | Conditions / Notes |
|---|---|---|---|---|
| Base rendering | Supported | Supported | Supported | `Screen` and `World2d` use the 2D path. `World3d` uses the 3D material path. |
| Picking + pointer events | Supported | Supported | Supported | Interaction resolves the camera from each root through `ResolvedRootUi`. For multi-camera scenes, prefer `UiCameraRef::Entity`. |
| Clipping-aware hit testing | Supported | Supported | Supported | Ancestor clipping checks apply across all spaces. World roots are still hit-tested against their UI plane. |
| `UPanelWindow` resize handles | Supported | Supported | Supported | Resize logic resolves cursor movement through the root camera and panel plane. |
| `UTextField` input/events | Supported | Supported | Supported | Included by default through `UnivisWidgetPlugin`; add `UnivisTextFieldPlugin` directly only for narrower manual widget composition. |
| `UScrollContainer` interaction | Supported | Supported | Supported | Scroll interaction follows the resolved root camera path. |
| `UPbr` controls (`metallic`, `roughness`, `emissive`) | N/A | N/A | Supported | Intended only for the `World3d` render path. |

## Notes

- `URootUi::screen()` is a real HUD path tied to the resolved viewport.
- `URootUi::world_2d(...)` and `URootUi::world_3d(...)` use a fixed logical canvas plus explicit world scaling.
- `meters_per_unit` changes physical world size without changing logical layout size.

## Verification Sources

- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/layout/render/system.rs`
