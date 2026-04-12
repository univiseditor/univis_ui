# Plugin Setup and First Examples

This page is the shortest task-oriented path after [Quick Start](quick-start.md).

Use it when you want one page that answers two questions:

- which plugins do I really get by default?
- which example should I run first for the task I care about?

## Facade Setup

If you add `UnivisUiPlugin`, you already get:

- `UnivisUiStylePlugin`
- `UnivisEnginePlugin`
- `UnivisInteractionPlugin`
- `UnivisWidgetPlugin`

This is the recommended path for most applications.

## Widget Runtime Coverage

`UnivisUiPlugin` includes `UnivisWidgetPlugin`, and that default widget surface now includes:

- `UnivisTextFieldPlugin` for `UTextField` behavior and events
- `UnivisBadgePlugin` for dynamic `UBadge` / `UTag` updates

If you compose plugins manually around `UnivisWidgetPlugin`, you do not need extra widget runtime plugins anymore:

```rust,no_run
use bevy::prelude::*;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiStylePlugin)
        .add_plugins(UnivisEnginePlugin)
        .add_plugins(UnivisInteractionPlugin)
        .add_plugins(UnivisWidgetPlugin)
        .run();
}
```

Use the dedicated widget plugins directly only when you intentionally want a narrower widget surface than `UnivisWidgetPlugin`.

## Camera Setup

- interaction and panel resizing resolve the camera from each `URootUi`
- a simple `Camera2d` is enough for the smallest screen-space examples
- in multi-camera scenes, prefer `UiCameraRef::Entity(camera_entity)`

## Best-First Paths

### Basic Screen HUD

Run these in order:

1. `cargo run -p univis_ui --example hello_world`
2. `cargo run -p univis_ui --example root_screen_hud`

Focus on:

- the smallest facade-level boot path
- viewport-fixed HUD behavior while the camera moves

### World-Space Panel

Run these in order:

1. `cargo run -p univis_ui --example root_world_scale`
2. `cargo run -p univis_ui --example root_fit_content`

Focus on:

- logical canvas size versus physical world size
- content-sized world panels and inspector-like roots

### Text Input

Run this first:

1. `cargo run -p univis_ui_widgets --example text_field`

Focus on:

- editable input
- change and submit behavior
- default text-field runtime coverage through `UnivisWidgetPlugin`

### Selection Controls

Run these in order:

1. `cargo run -p univis_ui_widgets --example select`
2. `cargo run -p univis_ui_widgets --example widgets`

Focus on:

- option navigation and disabled-option handling
- how `USelect` fits into the broader built-in widget surface

## Related Pages

- [Quick Start](quick-start.md)
- [Plugin Truth Table](architecture/plugin-truth-table.md)
- [Example Gallery](examples/gallery.md)
- [Current Limitations](development/current-limitations.md)
