# Univis UI
[![Crates.io](https://img.shields.io/crates/v/univis_ui)](https://crates.io/crates/univis_ui)
[![Bevy](https://img.shields.io/badge/Bevy-0.18.1-blue)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green)](LICENSE)

Arabic version: [README_AR.md](README_AR.md)
Migration summary: [MIGRATION.md](MIGRATION.md)
Hosted docs target: `https://univiseditor.github.io/univis_ui/`

Build sharp, scalable UI for Bevy across screen HUDs, world panels, and 3D-lit interfaces with one ECS-native stack.

> Important:
> `univis_ui` is still in **alpha**. API and behavior can change between versions.

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
univis_ui = "0.3.0"
```

If you want direct control over the internal layers:

```toml
[dependencies]
univis_ui_engine = "0.3.0"
univis_ui_style = "0.3.0"
univis_ui_interaction = "0.3.0"
univis_ui_widgets = "0.3.0"
```

## Crate Map

- `univis_ui`: start here for most applications; use `univis_ui::prelude` for the canonical root, layout, interaction, style, and widget surface.
- `univis_ui_engine`: depend on this when you need roots, layout, rendering sync, or custom widget/runtime work without the full facade.
- `univis_ui_interaction`: depend on this when you only need picking and `UInteraction`-driven feedback on top of engine roots.
- `univis_ui_widgets`: depend on this when you want the built-in controls but still compose plugins and dependencies yourself.
- `univis_ui_style`: depend on this when you only need shared theme resources, fonts, icons, or text styles.

Prelude policy:
each crate's `prelude` is the recommended day-to-day surface. Advanced integrations should use explicit named modules, and deprecated compatibility wrappers stay on explicit paths such as `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}` or `univis_ui_engine::layout::layout_system::{UScreenRoot, UWorldRoot}`. New examples and recommended docs use canonical `URootUi` roots only.

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

Facade note: `UnivisUiPlugin` includes `UnivisWidgetPlugin`, and that default widget surface now covers the built-in `UTextField` and `UBadge` runtime systems too. If you intentionally compose a narrower widget surface, dedicated plugins such as `UnivisTextFieldPlugin` and `UnivisBadgePlugin` still remain available. For the shortest setup checklist and task-oriented example path, see [Plugin Setup and First Examples (EN)](docs/src/en/first-steps.md) and [إعداد الإضافات وأولى الأمثلة (AR)](docs/src/ar/first-steps.md).

## Current Alpha Surface

### Stable Enough To Build On

- `URootUi` is the canonical public root API
- `URootUi::screen()`, `URootUi::world_2d(...)`, and `URootUi::world_3d(...)` are the intended entry points
- `UiSpace`, `UiCameraRef`, and `UiCanvasSize`, including `UiCanvasSize::FitContent { min, max }`, are part of the public root story
- the examples catalog lists only source files that ship in the current branch

### Deprecated But Still Supported

- `UScreenRoot` and `UWorldRoot` remain available only as deprecated compatibility wrappers on explicit paths such as `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}`; the current removal target is the first alpha after `0.3.0` that no longer needs wrapper-based migration support
- `meters_per_unit` remains the explicit knob for world-root physical size, and it stays on the canonical `URootUi` surface

### Still Settling

- fit-content world roots under heavy `%` sizing or strongly root-relative flex layouts
- some of the more polished `World3d` showcase defaults
- visual validation for rendering-heavy changes still relies partly on manual review

### Frame-Zero Rollout Gate

- `UiRolloutConfig` exposes internal switches for cached ancestry, incremental measure/solve/render frontiers, mesh reuse, and post-settle picking during rollout
- `UiValidationState` tracks engine-side rollout validation such as cached ancestry checks
- `PickingValidationState` tracks interaction-side picking validation without pushing picking internals back into `engine`
- run `./scripts/verify_frame_zero_rollout.sh` before changing rollout defaults or relaxing validation

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
- Docs home: `docs/src/index.md`
- Hosted docs: `https://univiseditor.github.io/univis_ui/`

Build docs:

```bash
mdbook build docs
```

Serve docs locally:

```bash
mdbook serve docs -n 127.0.0.1 -p 3000
```

Good starting points:

- [Docs Home](docs/src/index.md)
- [Migration Summary](MIGRATION.md)
- [Stable 0.3.x Roadmap](ROADMAP.md)
- [Quick Start (EN)](docs/src/en/quick-start.md)
- [Quick Start (AR)](docs/src/ar/quick-start.md)
- [Plugin Setup and First Examples (EN)](docs/src/en/first-steps.md)
- [إعداد الإضافات وأولى الأمثلة (AR)](docs/src/ar/first-steps.md)
- [Android Phone Demo Package](android/android_phone_app)
- [Example Gallery (EN)](docs/src/en/examples/gallery.md)
- [Example Gallery (AR)](docs/src/ar/examples/gallery.md)
- [Roots and Spaces (EN)](docs/src/en/layout/roots.md)
- [Roots and Spaces (AR)](docs/src/ar/layout/roots.md)
- [Crate Map (EN)](docs/src/en/api/crate-map.md)
- [خريطة الحزم (AR)](docs/src/ar/api/crate-map.md)
- [Examples Index (EN)](docs/src/en/examples/index.md)
- [Examples Index (AR)](docs/src/ar/examples/index.md)
- [Migration and Limitations (EN)](docs/src/en/migration/index.md)
- [الترحيل والقيود (AR)](docs/src/ar/migration/index.md)

## GitHub Discovery Path

1. Read this `README.md` for the project story and crate map.
2. Check [RELEASE_NOTES.md](RELEASE_NOTES.md) for the current release story, [RELEASE_PREP_0.3.0.md](RELEASE_PREP_0.3.0.md) for next-release evidence, and [changelog.md](changelog.md) for dated implementation history.
3. Open [Docs Home](docs/src/index.md) or the hosted docs URL and jump into the relevant guide.
4. Use the language-specific [Example Gallery (EN)](docs/src/en/examples/gallery.md) or [Example Gallery (AR)](docs/src/ar/examples/gallery.md) before dropping to the full index.
5. Generate `cargo doc --no-deps -p univis_ui` when you need exact type paths and signatures.

## Root File Guide

- `README.md`: project story, strengths, and the fastest entry path.
- `README_AR.md`: Arabic landing page with the same high-level role.
- `ROADMAP.md`: execution roadmap from alpha cleanup to beta readiness.
- `MIGRATION.md`: migration path from older docs and root assumptions.
- `MIGRATION_AR.md`: Arabic migration path.
- `RELEASE_NOTES.md`: current alpha release-scale summary.
- `RELEASE_PREP_0.3.0.md`: local verification evidence for the next `0.3.0` cut.
- `changelog.md`: dated chronological record of notable changes.

## API Docs

Use guides for concepts, migration notes, and example-driven learning. Use generated API docs for exact paths, fields, and signatures.

Generate the API docs:

```bash
cargo doc --no-deps -p univis_ui
```

Useful entry points:

- `univis_ui::UnivisUiPlugin`
- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::univis_node::UNode`
- `univis_ui_interaction::interaction::feedback::UInteraction`
- `univis_ui_widgets::widget::text_label::UTextLabel`
- `univis_ui_style::style::Theme`

Useful reference entries:

- `android_phone`
- `responsive_layout_test`
- `widgets_controls`
- `widgets_inputs`

Run the current live demo:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## Crates

- `univis_ui`: facade entry point and recommended default dependency
- `univis_ui_engine`: roots, layout, rendering, and core node model
- `univis_ui_interaction`: picking and interaction feedback
- `univis_ui_style`: fonts, icons, and shared styling resources
- `univis_ui_widgets`: built-in widgets and widget plugins

## Current Status

The current alpha line is focused on `URootUi`, world scaling, root capsules, unified docs, and API cleanup.

If you want a single Bevy UI stack that can handle HUDs, world-space panels, and 3D-lit interfaces without splitting your mental model across multiple systems, this is what Univis UI is trying to deliver.
