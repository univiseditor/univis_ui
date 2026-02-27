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
univis_ui = "0.1.2"
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
- `UBadge`, `UTag`
- `UDragValue`
- `USelect`

### Widget Notes
- `UnivisUiPlugin` installs the core widget plugin set.
- If you use text field features heavily, ensure `UnivisTextFieldPlugin` is added.
- If you rely on dynamic `UBadge` / `UTag` styling updates, add `UnivisBadgePlugin` explicitly.
- For scrolling behavior, add `scroll_interaction_system` in your app schedule.
- `USelect` supports mouse interaction and basic keyboard navigation.

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
