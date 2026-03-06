# Current Limitations

This page lists current, code-verified constraints so setup guidance remains explicit and predictable.

## Interaction Camera Dependency

- `univis_picking_backend` currently queries `Camera2d`.
- `UPanelWindow` resize interaction currently queries `Camera2d`.
- Practical consequence: for reliable interaction behavior, spawn a `Camera2d` in the active UI scene.

## Optional Widget Plugins

- `UnivisTextFieldPlugin` is **not** auto-registered by `UnivisWidgetPlugin`.
- `UnivisBadgePlugin` is **not** auto-registered by `UnivisWidgetPlugin`.
- Practical consequence: add these plugins explicitly when you need their behavior/events systems.

## Placeholder / Incomplete Surfaces

- `src/widget/menu.rs` is currently an empty placeholder module.

## Naming Inconsistency

- Layout module name is currently `geomerty` in code paths/exports.
- This is a known cleanup target and may be renamed in a future breaking cleanup.

## Verification Sources

- `src/interaction/picking.rs`
- `src/widget/panel.rs`
- `src/widget/mod.rs`
- `src/widget/menu.rs`
- `src/layout/mod.rs`
