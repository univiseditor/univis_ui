# Quick Start

## 1) Add Dependency

```toml
[dependencies]
univis_ui = "0.3.0"
```

## 2) Minimal App

```rust,no_run
use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            URootUi::screen(),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgb(0.08, 0.1, 0.14),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(UTextLabel::new("Hello Univis UI"));
        });
}
```

## 3) Root Choices

- `URootUi::screen()` for real HUD and screen-fixed overlays.
- `URootUi::world_2d(size)` for flat world-space UI.
- `URootUi::world_3d(size)` for world-space UI that uses the 3D material path.
- `URootUi::world_2d_fit_content()` and `URootUi::world_3d_fit_content()` size world roots from measured content.
- `UVal::Px` means logical UI units, not literal display pixels.
- `UiCanvasSize::Viewport` follows the resolved camera viewport.
- `UiCanvasSize::FitContent { min, max }` measures logical canvas size from content and clamps it when needed.
- World-space physical size is derived as:
  `world_size = canvas_size * meters_per_unit`

## 4) What `UnivisUiPlugin` Adds

- Interaction: `UnivisInteractionPlugin`
- Engine: `UnivisEnginePlugin`
- Style/fonts/icons: `UnivisUiStylePlugin`
- Widgets: `UnivisWidgetPlugin`

## 5) Important Notes

- `UnivisWidgetPlugin` now auto-registers the built-in `UTextField` and `UBadge` runtime systems.
- `UnivisScrollViewPlugin` is included by default in `UnivisWidgetPlugin`.
- Interaction resolves the camera from each `URootUi`.
- In multi-camera scenes, prefer binding the root explicitly with `UiCameraRef::Entity`.
- `UScreenRoot` and `UWorldRoot` remain available only as deprecated compatibility wrappers on explicit paths such as `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}`.
- Set `meters_per_unit` explicitly when you need a specific physical size for world-space roots.
- For a task-oriented setup checklist and recommended first runs, continue with [Plugin Setup and First Examples](first-steps.md).

If you compose widgets narrowly without `UnivisWidgetPlugin`, you can still add `UnivisTextFieldPlugin` or `UnivisBadgePlugin` directly.

## 6) Direct Crate Mode (Advanced)

```rust,no_run
use bevy::prelude::*;
use univis_ui_engine::prelude::*;
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

## Related Examples

- [`responsive_layout_test`](examples/index.md#workspace-examples)
- [`widgets_controls`](examples/index.md#widgets)
- [`widgets_inputs`](examples/index.md#widgets)

## Related Migration Notes

- [Root API Migration to `URootUi`](migration/root-api.md)
- [Example Path Migration](migration/example-paths.md)

## API Entry Points

- `univis_ui::UnivisUiPlugin`
- `univis_ui::prelude`
- `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}` for explicit legacy compatibility only
- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::univis_node::{UNode, ULayout}`

## Where To Look Next

- related example: [`responsive_layout_test`](examples/index.md#workspace-examples)
- related setup page: [Plugin Setup and First Examples](first-steps.md)
- related API index: [API Reference](api/index.md)
- related migration page: [Root API Migration to `URootUi`](migration/root-api.md)
