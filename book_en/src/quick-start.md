# Quick Start

## 1) Add Dependency

```toml
[dependencies]
univis_ui = "0.1.2"
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
            UScreenRoot,
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

## 3) What `UnivisUiPlugin` Adds

- Interaction: `UnivisInteractionPlugin`
- Core node/layout: `UnivisNodePlugin` + `UnivisLayoutPlugin`
- Style/fonts/icons: `UnivisUiStylePlugin`
- Widgets: `UnivisWidgetPlugin`

## 4) Important Notes

- `UnivisWidgetPlugin` does not auto-register `UnivisTextFieldPlugin`.
- `UnivisWidgetPlugin` does not auto-register `UnivisBadgePlugin`.
- `UnivisScrollViewPlugin` is included by default in `UnivisWidgetPlugin`.

If you compose plugins manually, add optional plugins explicitly when needed.
