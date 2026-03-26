# Univis UI
[![Crates.io](https://img.shields.io/crates/v/univis_ui)](https://crates.io/crates/univis_ui)
[![Bevy](https://img.shields.io/badge/Bevy-0.18.1-blue)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green)](LICENSE)

Arabic version: [README_AR.md](README_AR.md)

Build sharp, scalable UI for Bevy across screen HUDs, world panels, and 3D-lit interfaces with one ECS-native stack.

> Important:
> `univis_ui` is still in **alpha**. API and behavior can change between versions.

From `cargo run --release --example card_profile`:

![profile](profile.png)

## Why Univis UI

Univis UI is built for teams that want more than a basic HUD layer.

- One UI stack for `screen`, `world_2d`, and `world_3d`
- ECS-native composition: UI is entities and components, not a separate retained tree
- Custom layout engine with `Flex`, `Grid`, `Masonry`, `Stack`, and `Radial`
- SDF-driven rendering for crisp shapes and rounded surfaces under scale
- Built-in picking, interaction states, and widget behavior
- Fit-content world roots for node panels, floating inspectors, and diegetic cards
- Modular plugin surface when you want the engine without the full facade

## What It Helps You Build

- game HUDs and overlays
- diegetic world-space interfaces
- 3D control panels and sci-fi surfaces
- in-game tools, editors, and inspector-like panels
- node-graph style UI with overlapping root capsules

## Installation

```toml
[dependencies]
univis_ui = "0.2.0-alpha.1"
```

If you want direct control over the internal layers:

```toml
[dependencies]
univis_ui_engine = "0.2.0-alpha.1"
univis_ui_style = "0.2.0-alpha.1"
univis_ui_interaction = "0.2.0-alpha.1"
univis_ui_widgets = "0.2.0-alpha.1"
```

## Quick Start

```rust
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
                background_color: Color::srgb(0.08, 0.10, 0.14),
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
            root.spawn(UTextLabel {
                text: "Hello Univis UI".into(),
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            });
        });
}
```

## Core Strengths

### Roots That Match Real Use Cases

- `URootUi::screen()` for true viewport-fixed HUD UI
- `URootUi::world_2d(size)` for flat world UI
- `URootUi::world_3d(size)` for lit 3D UI
- `world_2d_fit_content()` and `world_3d_fit_content()` for content-sized world roots

### Layout That Goes Beyond Basic HUD Rows

- flex and grid for standard UI
- masonry and radial for expressive layouts
- item and container extensions for advanced alignment and track behavior
- root capsules that keep one UI tree from leaking over another

### Rendering That Survives Scale

- SDF materials for smooth corners and borders
- 2D and 3D material paths from the same UI model
- clipping and PBR support where needed
- physical world scaling through `meters_per_unit`

### Interaction And Widgets Included

- pointer picking that resolves through each root
- `UInteraction` and `UInteractionColors`
- built-in text, image, button, toggle, checkbox, radio, seekbar, select, panel, scroll, and more

## Docs And Examples

The full documentation lives in `docs/` as one mdBook with Arabic and English trees:

- Arabic: `docs/src/ar/`
- English: `docs/src/en/`

Build docs:

```bash
mdbook build docs
```

Serve docs locally:

```bash
mdbook serve docs -n 127.0.0.1 -p 3000
```

Good starting points:

- [Quick Start (EN)](docs/src/en/quick-start.md)
- [البدء السريع (AR)](docs/src/ar/quick-start.md)
- [Roots and Spaces (EN)](docs/src/en/layout/roots.md)
- [الجذور والمساحات (AR)](docs/src/ar/layout/roots.md)
- [Examples Index (EN)](docs/src/en/examples/index.md)
- [فهرس الأمثلة (AR)](docs/src/ar/examples/index.md)

Useful examples:

- `hello_world`
- `root_screen_hud`
- `root_world_scale`
- `root_fit_content`
- `root_capsule_overlap`
- `border_light_3d`
- `card_profile`
- `sci_fi`

Run one:

```bash
cargo run -p univis_ui_engine --example root_fit_content
```

## Crates

- `univis_ui`: facade entry point
- `univis_ui_engine`: roots, layout, rendering, and core node model
- `univis_ui_interaction`: picking and interaction feedback
- `univis_ui_style`: fonts, icons, and shared styling resources
- `univis_ui_widgets`: built-in widgets and widget plugins

## Current Status

The `alpha2` line is actively evolving around `URootUi`, world scaling, root capsules, unified docs, and API cleanup.

If you want a single Bevy UI stack that can handle HUDs, world-space panels, and 3D-lit interfaces without splitting your mental model across multiple systems, this is what Univis UI is trying to deliver.
