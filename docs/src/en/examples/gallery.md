# Example Gallery

This page is the curated view of the example surface. Use [Examples](index.md) for the full catalog and package inventory. For a task-oriented path that starts from setup and moves into examples, start with [Plugin Setup and First Examples](../first-steps.md).

## Best First Examples

1. `hello_world`
2. `root_screen_hud`
3. `root_world_scale`
4. `text_field`
5. `interaction`

## HUD And Screen UI

| Example | Package | Why run it | Command | Visual |
| --- | --- | --- | --- | --- |
| `hello_world` | `univis_ui` | Start here when you want the smallest facade-level boot path. | `cargo run -p univis_ui --example hello_world` | run the example |
| `root_screen_hud` | `univis_ui_engine` | Run this when you want to confirm true HUD behavior against camera motion. | `cargo run -p univis_ui_engine --example root_screen_hud` | run the example |
| `layout_cache` | `univis_ui_engine` | Run this when you want to inspect cache invalidation and repeated solve behavior. | `cargo run -p univis_ui_engine --example layout_cache` | run the example |
| `layout_sizing_semantics` | `univis_ui_engine` | Run this when you want one compact scene for `Auto`, intrinsic modes, and `min/max` flex behavior. | `cargo run -p univis_ui_engine --example layout_sizing_semantics` | run the example |

## World-Space UI

| Example | Package | Why run it | Command | Visual |
| --- | --- | --- | --- | --- |
| `root_world_scale` | `univis_ui_engine` | Run this when you need to calibrate logical canvas size against physical world size. | `cargo run -p univis_ui_engine --example root_world_scale` | run the example |
| `root_fit_content` | `univis_ui_engine` | Run this when you want content-sized world panels and inspectors. | `cargo run -p univis_ui_engine --example root_fit_content` | run the example |
| `root_capsule_overlap` | `univis_ui_engine` | Run this when you want to verify sealed root-capsule overlap rules. | `cargo run -p univis_ui_engine --example root_capsule_overlap` | run the example |
| `border_light_3d` | `univis_ui_engine` | Run this when you want to inspect the lit `World3d` path and `UPbr`. | `cargo run -p univis_ui_engine --features example_bloom --example border_light_3d` | run the example |

## Widgets And Forms

| Example | Package | Why run it | Command | Visual |
| --- | --- | --- | --- | --- |
| `widgets` | `univis_ui_widgets` | Run this when you want a broad first pass over the built-in widget surface. | `cargo run -p univis_ui_widgets --example widgets` | run the example |
| `text_label` | `univis_ui_widgets` | Run this when you want to study text clipping, overflow, and autosize behavior. | `cargo run -p univis_ui_widgets --example text_label` | run the example |
| `widget_sizing_semantics` | `univis_ui_widgets` | Run this when you want a compact widget-level sizing scene for wrap cards, `min_width`, explicit intrinsic text, and native-size images. | `cargo run -p univis_ui_widgets --example widget_sizing_semantics` | run the example |
| `mixed_bidi_text` | `univis_ui_widgets` | Run this when you want to inspect Arabic + Latin text, bidi-aware ellipsis, and truncation sides. | `cargo run -p univis_ui_widgets --example mixed_bidi_text` | run the example |
| `text_edge_cases` | `univis_ui_widgets` | Run this when you want a compact regression scene for tricky text overflow cases. | `cargo run -p univis_ui_widgets --example text_edge_cases` | run the example |
| `text_field` | `univis_ui_widgets` | Run this when you want editable input plus filtering and submit/change events. | `cargo run -p univis_ui_widgets --example text_field` | run the example |
| `panel_window` | `univis_ui_widgets` | Run this when you want floating, resizable tool-window behavior. | `cargo run -p univis_ui_widgets --example panel_window` | run the example |
| `scroll_view` | `univis_ui_widgets` | Run this when you want overflow handling and explicit scrolling behavior. | `cargo run -p univis_ui_widgets --example scroll_view` | run the example |

## Interaction

| Example | Package | Why run it | Command | Visual |
| --- | --- | --- | --- | --- |
| `interaction` | `univis_ui_interaction` | Run this when you want to understand hit blocking, ignore, and passthrough rules. | `cargo run -p univis_ui_interaction --example interaction` | run the example |

## Showcase Demos

| Example | Package | Why run it | Command | Visual |
| --- | --- | --- | --- | --- |
| `card_profile` | `univis_ui` | Run this when you want the most polished compact showcase card. | `cargo run -p univis_ui --features example_bloom --example card_profile` | [screenshot](../../assets/profile.png) |
| `sci_fi` | `univis_ui` | Run this when you want a larger full-stack scene that mixes several systems. | `cargo run -p univis_ui --features example_bloom --example sci_fi` | run the example |
| `transit_control` | `univis_ui` | Run this when you want a dense dispatch-board showcase with a different visual language from `sci_fi`. | `cargo run -p univis_ui --example transit_control` | run the example |
| `complex_dashboard` | `univis_ui` | Run this when you want a denser integrated composition with multiple UI regions. | `cargo run -p univis_ui --example complex_dashboard` | [HTML reference](../../assets/visual-references/complex_dashboard.html) |

## Static Visual References

These are useful when you want a quick visual reference without starting Bevy:

- [Complex Dashboard HTML](../../assets/visual-references/complex_dashboard.html)
- [Layout Solver No Widgets HTML](../../assets/visual-references/layout_solver_no_widgets.html)
- [Layout Solver Ultra Complex HTML](../../assets/visual-references/layout_solver_ultra_complex.html)
- [Card Profile Screenshot](../../assets/profile.png)

## Related Pages

- [Examples](index.md)
- [Migration and Limitations](../migration/index.md)
- [Example Validation Report](../development/example-validation.md)
