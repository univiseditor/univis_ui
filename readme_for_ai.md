# Univis UI: AI Integration Manual

## 1. Purpose
This document is optimized for AI agents that generate, review, or refactor code using `univis_ui`.
It is not a marketing document; it is an operational specification based on the current repository state.

## 2. Package Metadata
- Crate: `univis_ui`
- Current version in repo: `0.1.2`
- Rust edition: `2024`
- Engine dependency: `bevy = 0.17.3`
- Status: **Alpha** (API/behavior may change)

## 3. Top-Level Architecture
`UnivisUiPlugin` composes these subsystems:
1. `UnivisInteractionPlugin`
2. `UnivisNodePlugin`
3. `UnivisLayoutPlugin`
4. `UnivisUiStylePlugin`
5. `UnivisWidgetPlugin`

Operational consequence:
- In most apps, `add_plugins(UnivisUiPlugin)` is the single entry point.
- Some features still require extra plugin/system registration (see section 12).

## 4. Core Concepts
### 4.1 Entity Model
All UI is ECS entities + components.
No retained-mode widget tree abstraction outside ECS.

### 4.2 Root Types
- `UScreenRoot`: screen-space root (HUD-like).
- `UWorldRoot`: world-space root with fields:
  - `size: Vec2`
  - `is_3d: bool`
  - `resolution_scale: f32`

When `UWorldRoot.is_3d = true`, `auto_propagate_ui3d` can attach `UI3d` marker to hierarchy.

### 4.3 Node + Layout + Child Override
- `UNode`: size/box/visual base (`width`, `height`, `padding`, `margin`, `background_color`, `border_radius`, `shape_mode`).
- `ULayout`: container algorithm and alignment.
- `USelf`: per-child overrides (`align_self`, positional offsets, `position_type`, `order`).

### 4.4 Size Units
`UVal` supports:
- `Px(f32)`
- `Percent(f32)`
- `Content`
- `Auto`
- `Flex(f32)`

## 5. Layout Engine Capabilities
### 5.1 Display Modes (`UDisplay`)
- `Flex`
- `Grid`
- `Masonry`
- `Stack`
- `Radial`
- `None`

### 5.2 Direction + Distribution
- Direction: `UFlexDirection::{Row, Column, RowReverse, ColumnReverse}`
- Main-axis distribution: `UJustifyContent::{Start, Center, End, SpaceBetween, Stretch, SpaceAround, SpaceEvenly}`
- Cross-axis alignment: `UAlignItems` variants including `Stretch`.

### 5.3 Extended Alignment / CSS-like Controls
Container-level:
- `UBoxAlignContainer`
  - `justify_items`
  - `align_content`
  - `row_gap: Option<f32>`
  - `column_gap: Option<f32>`

Item-level:
- `UBoxAlignSelf`
  - `justify_self`
  - `align_self`
  - `justify_overflow: UOverflowPosition::{Safe, Unsafe}`
  - `align_overflow: UOverflowPosition::{Safe, Unsafe}`

### 5.4 Flex Extensions
- `UFlexContainerExt`
  - `wrap: UFlexWrap::{NoWrap, Wrap, WrapReverse}`
  - `align_content`
- `UFlexItemExt`
  - `flex_grow`
  - `flex_shrink`
  - `flex_basis: UVal`

### 5.5 Grid Extensions
- `UGridContainerExt`
  - `template_columns: Vec<UTrackSize>`
  - `template_rows: Vec<UTrackSize>`
  - `auto_flow: UGridAutoFlow::{Row, Column}`
  - `auto_rows: UTrackSize`
  - `auto_columns: UTrackSize`
- `UTrackSize::{Px, Fr, Auto}`
- `UGridItemExt`
  - `column_start`, `column_span`
  - `row_start`, `row_span`

## 6. Rendering Capabilities
### 6.1 SDF Materials
- 2D material: `UNodeMaterial`
- 3D material: `UNodeMaterial3d`
- Embedded shaders:
  - `layout/render/shaders/unode.wgsl`
  - `layout/render/shaders/unode_3d.wgsl`

### 6.2 2D vs 3D UI
- 2D path is default.
- 3D path uses `UI3d` marker and supports `UPbr`:
  - `metallic`
  - `roughness`
  - `emissive`

### 6.3 Visual Components
- Border: `UBorder`
- Corner radius: `UCornerRadius`
- Shape mode: `UShapeMode::{Round, Cut}`
- Clipping container: `UClip { enabled }`

## 7. Interaction Capabilities
### 7.1 Picking Backend
- System: `univis_picking_backend`.
- Hit test uses SDF rounded-box math + ancestor clipping checks.
- Filters parent hits when deeper child hit exists.

### 7.2 Interaction State
- Component: `UInteraction`
  - `Normal`, `Hovered`, `Pressed`, `Released`, `Clicked`
- Optional style mapping: `UInteractionColors { normal, hovered, pressed }`

### 7.3 Pointer Observers Registered by Plugin
- `on_pointer_over`
- `on_pointer_out`
- `on_pointer_press`
- `on_pointer_release`
- `on_pointer_click`

## 8. Widget Capability Matrix
### 8.1 Display/Visual Widgets
- `UTextLabel`
- `UImage`
- `UBadge`, `UTag`
- `UProgressBar`

### 8.2 Action/Input Widgets
- `UButton`
- `UIconButton`
- `UCheckbox`
- `UToggle`
- `URadioButton`, `URadioGroup`
- `USeekBar`
- `UTextField`
- `UDragValue`
- `USelect`

### 8.3 Scroll
- `UScrollContainer`
- Runtime logic system: `scroll_interaction_system`

### 8.4 Widget Events (Bevy messages)
- Toggle: `ToggleChangedEvent`
- Radio: `RadioButtonChangedEvent`
- SeekBar: `SeekBarChangedEvent`
- DragValue:
  - `DragValueChangedEvent`
  - `DragValueCommitEvent`
- Select:
  - `SelectChangedEvent`
  - `SelectOpenStateChangedEvent`
- TextField:
  - `TextFieldChangedEvent`
  - `TextFieldSubmitEvent`

## 9. Style System
`UnivisUiStylePlugin` embeds assets and initializes `Theme` resource:
- Text fonts:
  - Inter Regular
  - Adwaita Sans Regular
  - Fira Sans Regular
- Icon font:
  - Lucide
- Icon constants are exposed under `style::icons::Icon`.

## 10. Performance + Diagnostics
- Layout cache resource: `LayoutCache`
  - dirty node tracking
  - cached intrinsic sizes
  - depth map
- Profiling plugin: `LayoutProfilingPlugin`
  - timing metrics (upward/downward/material)
  - cache/material hit stats
  - frame history + percentile summaries (including p95)
  - visual overlay (timing bars + frame-time graph)
  - overlay diagnostics are visual-only (no periodic terminal logging by default)
  - controls:
    - `F10`: enable/disable profiler
    - `F11`: toggle overlay
    - `F9`: toggle graph
    - `F12`: cycle overlay position

## 11. System Scheduling Snapshot
### 11.1 Layout Plugin
`UnivisLayoutPlugin` schedules in `PostUpdate` chain:
1. `update_layout_hierarchy`
2. `upward_measure_pass_cached`
3. `downward_solve_pass_safe`
4. `downward_solve_pass_safe` (currently registered twice)

### 11.2 Interaction Plugin
- `univis_picking_backend` in `PreUpdate`
- Pointer observers registered globally

## 12. Current Gaps / Practical Constraints
AI agents should account for these current repo realities:
1. `UnivisWidgetPlugin` does **not** register `UnivisTextFieldPlugin`.
   - If text field behavior is required, add `UnivisTextFieldPlugin` explicitly.
2. `UnivisWidgetPlugin` does **not** register `UnivisBadgePlugin`.
   - If dynamic badge styling systems are required, add plugin explicitly.
3. Scroll behavior is a standalone system (`scroll_interaction_system`), not a dedicated plugin.
   - Add it manually in `Update`.
4. `src/widget/menu.rs` is currently empty (placeholder).
5. Picking backend queries `Camera2d`; interaction path is centered around 2D camera setup.

## 13. Recommended AI Codegen Workflow
### Step A: Bootstrap
1. Add `DefaultPlugins` + `UnivisUiPlugin`.
2. Spawn `Camera2d` unless you have a validated alternative path.

### Step B: Root Selection
- Screen UI: `UScreenRoot`.
- World UI: `UWorldRoot` with explicit `size`.
- For lit 3D surfaces, set `is_3d = true` and optionally add `UPbr`.

### Step C: Build Hierarchy
- Container entities: `UNode + ULayout`.
- Child overrides: add `USelf` and/or extended align components.
- For interactive nodes: include `UInteraction` or widget components that require pickability.
- For single-choice inputs:
  - Use `USelect` for compact or long option lists.
  - Use `URadioGroup` for short lists that should remain always visible.

### Step D: Optional Feature Activation
- Text field: `UnivisTextFieldPlugin`.
- Badge update systems: `UnivisBadgePlugin`.
- Scroll: `.add_systems(Update, scroll_interaction_system)`.

### Step E: Event Consumption
Use `MessageReader<T>` for widget events and handle updates in `Update` systems.

## 14. Example Index (Operational)
Stable commands for sampling coverage:

```bash
cargo run --release --example hello_world
cargo run --release --example interaction
cargo run --release --example masonry
cargo run --release --example css_alignment_showcase
cargo run --release --example drag_value
cargo run --release --example select
cargo run --release --example layout_case_flex_wrap
cargo run --release --example layout_case_grid_tracks
cargo run --release --example layout_case_grid_auto_flow
cargo run --release --example layout_case_stack
cargo run --release --example layout_case_masonry_ext
cargo run --release --example layout_case_radial
cargo run --release --example layout_case_alignment_overflow
```

## 15. Minimal AI Template
```rust,no_run
use bevy::prelude::*;
use univis_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UnivisUiPlugin)
        // Optional, only if needed:
        // .add_plugins(UnivisTextFieldPlugin)
        // .add_plugins(UnivisBadgePlugin)
        // .add_systems(Update, scroll_interaction_system)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        UScreenRoot,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            ..default()
        },
        ULayout::default(),
    ));
}
```

## 16. AI Safety Notes For Refactors
- Preserve existing ECS scheduling order unless intentional.
- Avoid removing reflection registrations for public components.
- Validate new examples with `cargo check --release --example <name>`.
- Prefer additive docs/examples over API-breaking edits in Alpha unless explicitly requested.
