# Examples

This is the canonical example catalog for the workspace.

For the curated view, best-first list, and visual references, start with [Example Gallery](gallery.md).

Run any example with:

```bash
cargo run -p <package> --example <name>
```

## Tag Legend

- `Learning`: a focused teaching example
- `Showcase`: a larger or more polished composition
- `Smoke`: a good candidate for automated or manual release validation

## Featured Learning Path

### `hello_world`

- Package: `univis_ui`
- Purpose: the smallest facade-level entry point for `UnivisUiPlugin`, a root, and one visible child.
- Best for: first boot and import sanity.
- Kind: `Learning`, `Smoke`

### `root_screen_hud`

- Package: `univis_ui`
- Purpose: proves that `URootUi::screen()` stays viewport-fixed while a world root still moves with the camera.
- Best for: HUD semantics and root-space mental model.
- Kind: `Learning`, `Smoke`

### `root_world_scale`

- Package: `univis_ui`
- Purpose: shows that the same logical canvas can map to different world sizes through `meters_per_unit`.
- Best for: world-root sizing and scale calibration.
- Kind: `Learning`, `Smoke`

### `root_fit_content`

- Package: `univis_ui`
- Purpose: demonstrates content-sized world roots through `UiCanvasSize::FitContent`.
- Best for: inspectors, floating cards, and node-like panels.
- Kind: `Learning`, `Smoke`

### `root_capsule_overlap`

- Package: `univis_ui`
- Purpose: demonstrates root-capsule stacking where protruding children stay below another root when their own root is below it.
- Best for: node-graph style UI and cross-root overlap rules.
- Kind: `Learning`, `Smoke`

### `border_light_3d`

- Package: `univis_ui_engine`
- Purpose: exercises the lit `World3d` render path with bloom-friendly styling.
- Best for: `World3d`, `UPbr`, and visual material validation.
- Kind: `Learning`, `Smoke`

### `interaction`

- Package: `univis_ui_interaction`
- Purpose: shows how nested interactive elements block, ignore, or pass through hits.
- Best for: picking semantics and observer-based interaction behavior.
- Kind: `Learning`, `Smoke`

### `widgets`

- Package: `univis_ui_widgets`
- Purpose: broad widget sampler covering common built-in controls.
- Best for: surveying the widget surface quickly.
- Kind: `Learning`

### `text_label`

- Package: `univis_ui_widgets`
- Purpose: explores `UTextLabel` layout, clipping, overflow, and autosize behavior.
- Best for: text rendering semantics.
- Kind: `Learning`

### `mixed_bidi_text`

- Package: `univis_ui_widgets`
- Purpose: demonstrates mixed Arabic + Latin text with bidi-aware ellipsis and truncation sides.
- Best for: multilingual text overflow behavior.
- Kind: `Learning`

### `text_edge_cases`

- Package: `univis_ui_widgets`
- Purpose: concentrates tricky `UTextLabel` cases like leading digits, mixed bidi strings, truncation sides, autosize clamp, and max-lines.
- Best for: text overflow regression checks.
- Kind: `Learning`

### `text_field`

- Package: `univis_ui_widgets`
- Purpose: demonstrates editable input, filtering modes, and submit/change messages.
- Best for: form input and default text-field runtime coverage.
- Kind: `Learning`

### `panel_window`

- Package: `univis_ui_widgets`
- Purpose: shows floating/resizable panel behavior.
- Best for: tool windows and editor-style layouts.
- Kind: `Learning`

### `scroll_view`

- Package: `univis_ui_widgets`
- Purpose: demonstrates scrolling overflow with a dedicated scroll container.
- Best for: large lists and constrained content areas.
- Kind: `Learning`

## Complete Catalog

### `univis_ui`

This package also owns the integrated root/layout demos that compose `univis_ui_engine`
with widgets or interaction helpers.

| Example | Purpose | Best for | Kind |
| --- | --- | --- | --- |
| `hello_world` | Minimal facade-level app with one root and one label. | first run | `Learning`, `Smoke` |
| `alignment` | Small alignment reference scene. | alignment rules | `Learning` |
| `ex_node` | Low-level node authoring demo. | engine primitives | `Learning` |
| `layout_cache` | Visualizes or stresses layout cache behavior. | cache diagnostics | `Learning` |
| `layout_case_alignment_overflow` | Focused alignment and overflow case. | solver edge cases | `Learning` |
| `layout_case_flex_wrap` | Demonstrates wrapping behavior in flex layouts. | flex wrap | `Learning` |
| `layout_case_grid_auto_flow` | Demonstrates grid auto-placement flow. | grid placement | `Learning` |
| `layout_case_grid_tracks` | Demonstrates grid track sizing and placement. | grid tracks | `Learning` |
| `layout_case_masonry_ext` | Exercises masonry container extensions. | masonry tuning | `Learning` |
| `layout_case_radial` | Demonstrates radial placement. | radial layout | `Learning` |
| `layout_sizing_semantics` | Demonstrates `Auto`, `MinContent`, `MaxContent`, and explicit `min/max` bounds in one screen. | sizing model | `Learning` |
| `layout_case_stack` | Demonstrates stack layout semantics. | stack layout | `Learning` |
| `root_screen_hud` | Confirms real HUD behavior for `URootUi::screen()`. | screen roots | `Learning`, `Smoke` |
| `root_world_scale` | Compares logical canvas size and physical world scaling. | `meters_per_unit` | `Learning`, `Smoke` |
| `root_capsule_overlap` | Demonstrates sealed root capsules under overlap. | root stacking | `Learning`, `Smoke` |
| `root_fit_content` | Demonstrates content-sized `World2d` and `World3d` roots. | fit-content roots | `Learning`, `Smoke` |
| `complex_dashboard` | Integrated dashboard with multiple widgets and denser composition. | facade-level composition | `Showcase` |
| `card_profile` | Profile card demo with polished presentation and optional bloom. | polished facade card | `Showcase`, `Smoke` |
| `sci_fi` | Larger full-stack sci-fi HUD scene. | final-look showcase | `Showcase`, `Smoke` |
| `transit_control` | Dense transit operations board with animated platform cards and dispatch panels. | non-sci-fi showcase composition | `Showcase` |

### `univis_ui_engine`

| Example | Purpose | Best for | Kind |
| --- | --- | --- | --- |
| `border_light_3d` | Lit `World3d` panel with bloom-friendly rendering. | 3D render path | `Learning`, `Smoke` |
| `layout_solver_no_widgets` | Solver-heavy scene without widget dependencies. | pure layout debugging | `Learning` |
| `layout_solver_ultra_complex` | Larger solver stress scene. | solver stress | `Learning`, `Smoke` |
| `masonry` | Higher-level masonry layout showcase. | masonry reference | `Learning` |
| `texture` | Texture/image path reference scene. | image geometry | `Learning` |

### `univis_ui_widgets`

| Example | Purpose | Best for | Kind |
| --- | --- | --- | --- |
| `drag_value` | Demonstrates drag-to-change numeric input. | numeric controls | `Learning` |
| `panel_divider` | Demonstrates divider layout inside panels. | panel layout | `Learning` |
| `panel_window` | Floating and resizable panel windows. | tool windows | `Learning` |
| `radio` | Radio buttons and groups. | mutually exclusive choices | `Learning` |
| `scroll_view` | Scroll container behavior and overflow. | scrolling | `Learning` |
| `seekbar` | Slider-style value selection. | continuous values | `Learning` |
| `select` | Dropdown selection with options. | single-select inputs | `Learning` |
| `text_field` | Editable text fields with filters and submit/change events. | text input | `Learning` |
| `text_label` | Text measurement, overflow, clipping, and autosize. | text rendering | `Learning` |
| `widget_sizing_semantics` | Focused widget scene for wrap cards, `min_width`, text `MinContent/MaxContent`, and `UImage` intrinsic sizing. | widget sizing model | `Learning` |
| `mixed_bidi_text` | Mixed Arabic + Latin text with bidi-aware ellipsis and start/end/middle truncation. | multilingual text overflow | `Learning` |
| `text_edge_cases` | Edge-case text overflow scenes covering digits, mixed bidi text, truncation sides, autosize clamp, and max-lines. | text regression checks | `Learning` |
| `text_label_zoom` | Text sharpness under zoom changes. | zoom/text fidelity | `Learning` |
| `toggle` | Binary switch behavior. | on/off state | `Learning` |
| `widgets` | Multi-widget sampler scene. | quick widget survey | `Learning` |

### `univis_ui_interaction`

| Example | Purpose | Best for | Kind |
| --- | --- | --- | --- |
| `interaction` | Demonstrates hit blocking, ignore, and passthrough behavior. | picking semantics | `Learning`, `Smoke` |

## Suggested Commands

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui --example interaction
```

## Related Reports

- [Example Validation Report](../development/example-validation.md)
- [Smoke Test Plan](../development/smoke-test-plan.md)
- [Example Path Migration](../migration/example-paths.md)
