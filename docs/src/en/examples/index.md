# Examples

This catalog records example ownership, history, and current availability across the repository.

## Current Availability

- `Live package`: source still ships in this branch and can be run directly
- `Static reference`: an HTML or image reference is kept for visual comparison
- `Archived source`: the historical example name is still documented, but the source file is not shipped in this branch

If you want the current live demo, start with:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

For the curated view and best-first path, see [Example Gallery](gallery.md).

## Featured Learning Path

### `hello_world`

- Availability: `Archived source`
- Historical package: `univis_ui`
- Purpose: the smallest facade-level entry point for `UnivisUiPlugin`, a root, and one visible child
- Use now: [Quick Start](../quick-start.md)

### `root_screen_hud`

- Availability: `Archived source`
- Historical package: `univis_ui`
- Purpose: proved that `URootUi::screen()` stayed viewport-fixed while a world root moved with the camera
- Use now: [Roots and Spaces](../layout/roots.md)

### `android_phone`

- Availability: `Live package`
- Current package: `android/android_phone_app`
- Purpose: builds a clean Android-style app screen with search, toggles, sliders, and compact bottom navigation
- Run now: `cargo run --manifest-path android/android_phone_app/Cargo.toml`

### `widgets_controls`

- Availability: `Live package`
- Current package: `univis_ui`
- Purpose: showcases `UButton`, `UCheckbox`, `UToggle`, `URadioGroup`, and `URadioButton` in one screen
- Run now: `cargo run --example widgets_controls`

### `widgets_inputs`

- Availability: `Live package`
- Current package: `univis_ui`
- Purpose: demonstrates `UTextField`, `USelect`, `UDragValue`, and `USeekBar` with live interaction and console messages
- Run now: `cargo run --example widgets_inputs`

### `widgets_display`

- Availability: `Live package`
- Current package: `univis_ui`
- Purpose: demonstrates `UBadge`, `UDivider`, `UProgressBar`, and `UPanel` with animated progress updates
- Run now: `cargo run --example widgets_display`

### `widgets_containers`

- Availability: `Live package`
- Current package: `univis_ui`
- Purpose: demonstrates `UPanelWindow`, `UScrollContainer`, and `UClip` through a resizable panel and a scrollable viewport
- Run now: `cargo run --example widgets_containers`

### `responsive_dashboard`

- Availability: `Static reference`
- Historical package: `univis_ui`
- Purpose: demonstrates a soft dashboard composition that steps from a narrow card into a full desktop workspace
- Use now: [responsive_dashboard.html](../../assets/visual-references/responsive_dashboard.html)

### `root_world_scale`

- Availability: `Archived source`
- Historical package: `univis_ui`
- Purpose: showed that the same logical canvas could map to different world sizes through `meters_per_unit`
- Use now: [Roots and Spaces](../layout/roots.md)

### `root_fit_content`

- Availability: `Archived source`
- Historical package: `univis_ui`
- Purpose: demonstrated content-sized world roots through `UiCanvasSize::FitContent`
- Use now: [Roots and Spaces](../layout/roots.md)

### `root_capsule_overlap`

- Availability: `Archived source`
- Historical package: `univis_ui`
- Purpose: demonstrated root-capsule stacking where protruding children stayed below another root
- Use now: [Interaction Overview](../interaction/overview.md)

### `border_light_3d`

- Availability: `Archived source`
- Historical package: `univis_ui_engine`
- Purpose: exercised the lit `World3d` render path with bloom-friendly styling
- Use now: [Rendering Overview](../rendering/overview.md)

### `interaction`

- Availability: `Archived source`
- Historical package: `univis_ui_interaction`
- Purpose: showed how nested interactive elements block, ignore, or pass through hits
- Use now: [Interaction Overview](../interaction/overview.md)

### `widgets`

- Availability: `Archived source`
- Historical package: `univis_ui_widgets`
- Purpose: broad widget sampler covering common built-in controls
- Use now: [Widgets Overview](../widgets/overview.md), `widgets_controls`, and `widgets_display`

### `text_label`

- Availability: `Archived source`
- Historical package: `univis_ui_widgets`
- Purpose: explored `UTextLabel` layout, clipping, overflow, and autosize behavior
- Use now: [Text, Image, and Badge Widgets](../widgets/text-image-badge.md)

### `text_field`

- Availability: `Archived source`
- Historical package: `univis_ui_widgets`
- Purpose: demonstrated editable input, filtering modes, and submit/change messages
- Use now: [Widget Inputs](../widgets/inputs.md) and `cargo run --example widgets_inputs`

### `panel_window`

- Availability: `Archived source`
- Historical package: `univis_ui_widgets`
- Purpose: showed floating and resizable panel behavior
- Use now: [Panel and Panel Window](../widgets/panel-window.md) and `cargo run --example widgets_containers`

### `scroll_view`

- Availability: `Archived source`
- Historical package: `univis_ui_widgets`
- Purpose: demonstrated scrolling overflow with a dedicated scroll container
- Use now: [Scroll View](../widgets/scroll-view.md) and `cargo run --example widgets_containers`

## Package History

### `univis_ui`

| Example | Purpose | Best for | Availability |
| --- | --- | --- | --- |
| `hello_world` | Minimal facade-level app with one root and one label. | first run | `Archived source` |
| `alignment` | Small alignment reference scene. | alignment rules | `Archived source` |
| `ex_node` | Low-level node authoring demo. | engine primitives | `Archived source` |
| `layout_cache` | Visualized or stressed layout cache behavior. | cache diagnostics | `Archived source` |
| `layout_case_alignment_overflow` | Focused alignment and overflow case. | solver edge cases | `Archived source` |
| `layout_case_flex_wrap` | Demonstrated wrapping behavior in flex layouts. | flex wrap | `Archived source` |
| `layout_case_grid_auto_flow` | Demonstrated grid auto-placement flow. | grid placement | `Archived source` |
| `layout_case_grid_tracks` | Demonstrated grid track sizing and placement. | grid tracks | `Archived source` |
| `layout_case_masonry_ext` | Exercised masonry container extensions. | masonry tuning | `Archived source` |
| `layout_case_radial` | Demonstrated radial placement. | radial layout | `Archived source` |
| `layout_case_stack` | Demonstrated stack layout semantics. | stack layout | `Archived source` |
| `layout_sizing_semantics` | Demonstrated `Auto`, intrinsic modes, and explicit `min/max` bounds in one screen. | sizing model | `Archived source` |
| `root_screen_hud` | Confirmed real HUD behavior for `URootUi::screen()`. | screen roots | `Archived source` |
| `android_phone` | Android-style app screen with search, toggles, sliders, and compact navigation. | mobile-like screen UI | `Live package` |
| `widgets_controls` | Focused control/widget surface with buttons, toggles, checkboxes, and radio groups. | common widget actions | `Live package` |
| `widgets_inputs` | Input-focused widget screen with text, select, drag, and slider controls. | input workflows | `Live package` |
| `widgets_display` | Visual widget sampler with badges, dividers, panels, and animated progress bars. | display widgets | `Live package` |
| `widgets_containers` | Container/widget screen with a resizable panel window and scrollable viewport. | panel + scrolling behavior | `Live package` |
| `responsive_dashboard` | Dashboard study that grew from a narrow card into a desktop workspace. | responsive screen composition | `Static reference` |
| `root_world_scale` | Compared logical canvas size and physical world scaling. | `meters_per_unit` | `Archived source` |
| `root_fit_content` | Demonstrated content-sized `World2d` and `World3d` roots. | fit-content roots | `Archived source` |
| `root_capsule_overlap` | Demonstrated sealed root capsules under overlap. | root stacking | `Archived source` |
| `complex_dashboard` | Integrated dashboard with multiple widgets and denser composition. | facade-level composition | `Static reference` |
| `card_profile` | Profile card demo with polished presentation and optional bloom. | polished facade card | `Static reference` |
| `sci_fi` | Larger full-stack sci-fi HUD scene. | final-look showcase | `Archived source` |
| `transit_control` | Dense transit operations board with animated cards and dispatch panels. | non-sci-fi showcase composition | `Archived source` |

### `univis_ui_engine`

| Example | Purpose | Best for | Availability |
| --- | --- | --- | --- |
| `border_light_3d` | Lit `World3d` panel with bloom-friendly rendering. | 3D render path | `Archived source` |
| `layout_solver_no_widgets` | Solver-heavy scene without widget dependencies. | pure layout debugging | `Static reference` |
| `layout_solver_ultra_complex` | Larger solver stress scene. | solver stress | `Static reference` |
| `masonry` | Higher-level masonry layout showcase. | masonry reference | `Archived source` |
| `texture` | Texture/image path reference scene. | image geometry | `Archived source` |

### `univis_ui_widgets`

| Example | Purpose | Best for | Availability |
| --- | --- | --- | --- |
| `drag_value` | Demonstrated drag-to-change numeric input. | numeric controls | `Archived source` |
| `panel_divider` | Demonstrated divider layout inside panels. | panel layout | `Archived source` |
| `panel_window` | Floating and resizable panel windows. | tool windows | `Archived source` |
| `radio` | Radio buttons and groups. | mutually exclusive choices | `Archived source` |
| `scroll_view` | Scroll container behavior and overflow. | scrolling | `Archived source` |
| `seekbar` | Slider-style value selection. | continuous values | `Archived source` |
| `select` | Dropdown selection with options. | single-select inputs | `Archived source` |
| `text_field` | Editable text fields with filters and submit/change events. | text input | `Archived source` |
| `text_label` | Text measurement, overflow, clipping, and autosize. | text rendering | `Archived source` |
| `widget_sizing_semantics` | Focused widget scene for wrap cards, `min_width`, text sizing, and image intrinsic sizing. | widget sizing model | `Archived source` |
| `mixed_bidi_text` | Mixed Arabic + Latin text with bidi-aware ellipsis and truncation. | multilingual text overflow | `Archived source` |
| `text_edge_cases` | Edge-case text overflow scenes covering digits, bidi text, truncation sides, and max-lines. | text regression checks | `Archived source` |
| `text_label_zoom` | Text sharpness under zoom changes. | zoom/text fidelity | `Archived source` |
| `toggle` | Binary switch behavior. | on/off state | `Archived source` |
| `widgets` | Multi-widget sampler scene. | quick widget survey | `Archived source` |

### `univis_ui_interaction`

| Example | Purpose | Best for | Availability |
| --- | --- | --- | --- |
| `interaction` | Demonstrated hit blocking, ignore, and passthrough behavior. | picking semantics | `Archived source` |

## Related Reports

- [Example Validation Report](../development/example-validation.md)
- [Smoke Test Plan](../development/smoke-test-plan.md)
- [Example Path Migration](../migration/example-paths.md)
