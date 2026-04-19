# Plugin Setup and First Paths

This page is the shortest task-oriented path after [Quick Start](quick-start.md).

Use it when you want one page that answers two questions:

- which plugins do I really get by default?
- what should I open or run first for the task I care about?

## Current Branch Note

Most of the older workspace example sources are archived in this branch. The current live demo is
the standalone Android-style package under `android/android_phone_app`, while the examples catalog
now doubles as an ownership/history map for archived names.

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
- a simple `Camera2d` is enough for the smallest screen-space scenes
- in multi-camera scenes, prefer `UiCameraRef::Entity(camera_entity)`

## Best-First Paths

### Basic Screen HUD

Start with:

- [Quick Start](quick-start.md)
- [`hello_world`](examples/index.md#hello_world)
- [`root_screen_hud`](examples/index.md#root_screen_hud)

Focus on:

- the smallest facade-level boot path
- viewport-fixed HUD behavior while the camera moves

### Android App Screen

Run this:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

Focus on:

- composing a clean Android-style application surface inside `URootUi::screen()`
- combining `UTextField`, `UToggle`, `USeekBar`, and `UButton` in one narrow screen
- checking whether your mobile-like spacing still reads well without drawing device hardware

### World-Space Panel

Start with:

- [Roots and Spaces](layout/roots.md)
- [`root_world_scale`](examples/index.md#root_world_scale)
- [`root_fit_content`](examples/index.md#root_fit_content)

Focus on:

- logical canvas size versus physical world size
- content-sized world panels and inspector-like roots

### Text Input

Start with:

- [Widget Inputs](widgets/inputs.md)
- [`text_field`](examples/index.md#text_field)

Focus on:

- editable input
- change and submit behavior
- default text-field runtime coverage through `UnivisWidgetPlugin`

### Selection Controls

Start with:

- [Widgets Overview](widgets/overview.md)
- [`widgets`](examples/index.md#widgets)

Focus on:

- how selection-oriented controls fit into the broader built-in widget surface
- how archived example names map back to the current widget APIs

## Related Pages

- [Quick Start](quick-start.md)
- [Plugin Truth Table](architecture/plugin-truth-table.md)
- [Example Gallery](examples/gallery.md)
- [Android Phone UI](examples/android-phone.md)
- [Current Limitations](development/current-limitations.md)
