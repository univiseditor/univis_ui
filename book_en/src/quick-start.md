# Quick Start

## 1) Add Dependency

```toml
[dependencies]
univis_ui = "0.2.0-alpha.1"
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
- `UVal::Px` means logical UI units, not literal display pixels.
- `UiCanvasSize::Viewport` follows the resolved camera viewport.
- World-space physical size is derived as:
  `world_size = canvas_size * meters_per_unit`

## 4) What `UnivisUiPlugin` Adds

- Interaction: `UnivisInteractionPlugin`
- Engine: `UnivisEnginePlugin`
- Style/fonts/icons: `UnivisUiStylePlugin`
- Widgets: `UnivisWidgetPlugin`

## 5) Important Notes

- `UnivisWidgetPlugin` does not auto-register `UnivisTextFieldPlugin`; add it explicitly when using `UTextField`.
- `UnivisWidgetPlugin` does not auto-register `UnivisBadgePlugin`.
- `UnivisScrollViewPlugin` is included by default in `UnivisWidgetPlugin`.
- Interaction resolves the camera from each `URootUi`.
- In multi-camera scenes, prefer binding the root explicitly with `UiCameraRef::Entity`.
- `UScreenRoot` and `UWorldRoot` remain only as deprecated compatibility wrappers during `alpha2`.
- If you need the historical world-space physical size from older examples, set `meters_per_unit: 1.0` explicitly.

If you compose plugins manually, add optional plugins explicitly when needed.

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
