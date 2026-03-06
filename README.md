# Univis UI
[![Crates.io](https://img.shields.io/crates/v/univis_ui)](https://crates.io/crates/univis_ui)
[![Bevy](https://img.shields.io/badge/Bevy-0.17.3-blue)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green)](LICENSE)

High-performance ECS UI framework for Bevy with an SDF-based rendering pipeline.

> Important:
> This project is in **Alpha** stage. API and behavior can change between versions.

From example `cargo run --release --example card_profile`:

![profile](profile.png)


## What Is Univis UI?
Univis UI is a Bevy-native UI framework designed for both screen-space and world-space interfaces.
It uses Signed Distance Field (SDF) materials to render crisp shapes and rounded UI at any scale.

### Core Highlights
- ECS-first architecture (all UI is entities + components)
- Custom layout solver with multiple display modes:
`Flex`, `Grid`, `Masonry`, `Stack`, `Radial`
- Extended CSS-inspired alignment/flex/grid controls
- Screen-space and world-space roots
- Optional 3D-lit UI with PBR controls
- Built-in interaction states and pointer picking backend
- Ready widgets: text, image, button, icon button, toggle, radio, seekbar, checkbox, progress
- Optional profiling overlay/tools for layout performance

## Installation
```toml
[dependencies]
univis_ui = "0.2.0"
```

### Direct Crate Mode (Advanced)
```toml
[dependencies]
univis_ui_core = "0.2.0"
univis_ui_layout = "0.2.0"
univis_ui_render = "0.2.0"
univis_ui_interaction = "0.2.0"
univis_ui_widgets = "0.2.0"
```

## Workspace Crates
- `univis_ui_core`: shared types, roots, style/theme, schedule labels.
- `univis_ui_layout`: hierarchy/measure/solve engine + algorithms/cache/profiling.
- `univis_ui_render`: SDF materials/shaders and material sync path.
- `univis_ui_interaction`: picking backend + interaction feedback.
- `univis_ui_widgets`: built-in widgets and optional-widget warnings.
- `univis_ui` (facade): unified entrypoint and compatibility prelude.

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
            root.spawn(UTextLabel {
                text: "Hello Univis UI".to_string(),
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            });
        });
}
```

Direct-crate composition is also available when you do not want the full facade:
```rust
use bevy::prelude::*;
use univis_ui_core::prelude::*;
use univis_ui_layout::layout::UnivisLayoutPlugin;
use univis_ui_render::layout::render::UnivisRenderPlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisInteractionPlugin)
        .add_plugins(UnivisNodePlugin)
        .add_plugins(UnivisLayoutPlugin)
        .add_plugins(UnivisRenderPlugin)
        .add_plugins(UnivisUiStylePlugin)
        .add_plugins(UnivisWidgetPlugin)
        .run();
}
```

## Project Book
This repository includes two mdBook editions:
- Arabic: `book_ar/`
- English: `book_en/`
English chapters are being translated incrementally while keeping full chapter parity.

Build it:
```bash
mdbook build book_ar
mdbook build book_en
```

Serve locally:
```bash
mdbook serve book_ar -n 127.0.0.1 -p 3000
mdbook serve book_en -n 127.0.0.1 -p 3001
```

Reference pages:
- [Plugin truth table (EN)](book_en/src/architecture/plugin-truth-table.md)
- [Plugin truth table (AR)](book_ar/src/architecture/plugin-truth-table.md)
- [Interaction support matrix (EN)](book_en/src/interaction/support-matrix.md)
- [Interaction support matrix (AR)](book_ar/src/interaction/support-matrix.md)
- [Current limitations (EN)](book_en/src/development/current-limitations.md)
- [Current limitations (AR)](book_ar/src/development/current-limitations.md)

## Spaces And Roots
### Screen Space
- Use `Camera2d`
- Root marker: `UScreenRoot`

### World Space
- Use `UWorldRoot { size, is_3d, resolution_scale }`
- Supports 2D/3D placement depending on your scene and camera setup
- Set `is_3d: true` to propagate `UI3d` and use the 3D material path

## Layout Model
### Primary Components
- `UNode`: size, padding, margin, background, border radius, shape mode
- `ULayout`: display algorithm + axis alignment + gaps + grid columns + `container_ext`
- `USelf`: per-child overrides (`align_self`, absolute positioning, order) + `item_ext`

### Units
- `UVal::Px(f32)`
- `UVal::Percent(f32)`
- `UVal::Content`
- `UVal::Auto`
- `UVal::Flex(f32)`

### Display Modes
- `UDisplay::Flex`
- `UDisplay::Grid`
- `UDisplay::Masonry`
- `UDisplay::Stack`
- `UDisplay::Radial`
- `UDisplay::None`

### Runtime Scheduling
- `UnivisLayoutPlugin` runs this `PostUpdate` chain:
  `update_layout_hierarchy` -> `upward_measure_pass_cached` -> `downward_solve_pass_safe`

### Extended Controls (New)
- Container-level alignment/flex/grid: `ULayout.container_ext`
  - `box_align: ULayoutBoxAlignContainer`
  - `flex: ULayoutFlexContainer`
  - `grid: ULayoutGridContainer`
- Item-level alignment/flex/grid: `USelf.item_ext`
  - `box_align: ULayoutBoxAlignSelf`
  - `flex: ULayoutFlexItem`
  - `grid: ULayoutGridItem`
- Grid track sizing: `UTrackSize::{Px, Fr, Auto}`
- Grid auto flow: `UGridAutoFlow::{Row, Column}`

## Rendering And Visuals
- Borders: `UBorder`
- Shapes: `UShapeMode::{Round, Cut}`
- Clipping: `UClip { enabled: bool }`
- 3D lighting controls: `UPbr { metallic, roughness, emissive }`

## Interaction Model
- Interaction state component: `UInteraction`
- Auto color feedback: `UInteractionColors`
- Pointer observers for over/out/press/release/click
- Picking backend performs SDF hit-tests and respects clipping ancestors

### Current Limitations
- `univis_picking_backend` currently queries `Camera2d`.
- `UPanelWindow` resize interaction currently queries `Camera2d`.
- For reliable interaction behavior, spawn `Camera2d` in the active UI scene.

## Built-in Widgets
- `UTextLabel`
- `UImage`
- `UButton`
- `UIconButton`
- `UCheckbox`
- `UToggle`
- `URadioButton`, `URadioGroup`
- `USeekBar`
- `UProgressBar`
- `UTextField`
- `UScrollContainer`
- `UPanel`
- `UBadge`, `UTag`
- `UDragValue`
- `USelect`

### Widget Notes
- `UnivisUiPlugin` installs the core widget plugin set.
- `UTextField` behavior/events require adding `UnivisTextFieldPlugin` explicitly.
- If you rely on dynamic `UBadge` / `UTag` styling updates, add `UnivisBadgePlugin` explicitly.
- Scroll behavior is provided by `UnivisScrollViewPlugin` (included by `UnivisUiPlugin`).
- `USelect` supports mouse interaction and basic keyboard navigation.

### Resizable Panel Borders
`UPanelWindow` enables opt-in border resize zones for `UPanel` (edges + corners).
Scope in this release:
- resize only (no move / no bring-to-front)
- cursor icon changes only on panel resize zones

```rust
commands.spawn((
    UPanel::glass(),
    UPanelWindow::default()
        .with_min_size(240.0, 160.0)
        .with_border_hit_thickness(8.0),
    UNode {
        width: UVal::Px(420.0),
        height: UVal::Px(260.0),
        ..default()
    },
));
```

## Style And Icons
- Embedded fonts (Inter, Adwaita Sans, Fira Sans)
- Embedded Lucide icon font
- `Theme` resource available via prelude
- Icon constants available from `style::icons::Icon`

## Profiling
- Optional plugin: `LayoutProfilingPlugin`
- Tracks pass timings, node stats, cache hit ratio, material reuse stats
- Visual overlay with timing bars and frame graph
- Keyboard controls: `F10` enable/disable profiler, `F11` overlay, `F9` graph, `F12` overlay position
- No terminal logging by default (overlay-only diagnostics)

## Examples
### Existing Examples
```bash
cargo run --release --example hello_world
cargo run --release --example text_label
cargo run --release --example text_field
cargo run --release --example texture
cargo run --release --example widgets
cargo run --release --example toggle
cargo run --release --example radio
cargo run --release --example seekbar
cargo run --release --example scroll_view
cargo run --release --example interaction
cargo run --release --example alignment
cargo run --release --example masonry
cargo run --release --example ex_node
cargo run --release --example layout_cache
cargo run --release --example card_profile
cargo run --release --example border_light_3d
cargo run --release --example sci_fi
cargo run --release --example panel_divider
cargo run --release --example panel_window
cargo run --release --example drag_value
cargo run --release --example select
```

### New Layout Case Examples
```bash
cargo run --release --example layout_case_flex_wrap
cargo run --release --example layout_case_grid_tracks
cargo run --release --example layout_case_grid_auto_flow
cargo run --release --example layout_case_stack
cargo run --release --example layout_case_masonry_ext
cargo run --release --example layout_case_radial
cargo run --release --example layout_case_alignment_overflow
```

## Contributing
Contributions are welcome.
Useful areas:
- Layout edge-case fixes
- Performance and cache improvements
- New widgets and examples
- Documentation quality and API consistency

## License
Licensed under [MIT](LICENSE) OR [Apache-2.0](LICENSE).

---
For AI-focused documentation, read: `readme_for_ai.md`.
