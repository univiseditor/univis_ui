# Current Limitations

This page lists current, code-verified constraints so setup guidance remains explicit and predictable.

## Interaction Camera Dependency

- `univis_picking_backend` resolves the camera from each `URootUi` through `ResolvedRootUi`; it does not hardcode `Camera2d`.
- `UPanelWindow` resize interaction follows the same resolved-root camera path.
- Practical consequence: reliable interaction still requires each active root to resolve to a camera with a valid viewport.
- In multi-camera scenes, prefer `UiCameraRef::Entity(...)` instead of relying on automatic camera resolution.

## Widget Runtime Notes

- `UnivisUiPlugin` adds `UnivisWidgetPlugin` automatically.
- `UnivisWidgetPlugin` now includes the built-in `UTextField` and `UBadge` runtime systems by default.
- Dedicated widget plugins remain available when you intentionally compose a narrower widget surface.
- `UTag` runtime systems are still limited; validate tag-heavy scenes manually.

## Verification Sources

- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`
- `crates/univis_ui_widgets/src/widget/mod.rs`
- `crates/univis_ui_widgets/src/widget/text_field.rs`
- `crates/univis_ui_widgets/src/widget/badge.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
