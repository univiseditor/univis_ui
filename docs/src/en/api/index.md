# API Reference

This section is a practical index for the public `univis_ui` API surface.

## Where To Find The Generated Reference

To generate rustdoc:

```bash
cargo doc --no-deps
```

Then open:

```text
target/doc/univis_ui/index.html
```

## How To Use This Section

- start with [Events and Messages](events.md) if you connect UI to gameplay or app logic
- use [Crate Map](crate-map.md) when deciding between the facade crate and the lower-level crates
- use [Public Types Table](public-types.md) for a fast map of the main structs, enums, and plugins

## High-Value Entry Points

- `univis_ui::UnivisUiPlugin` for the facade path
- `univis_ui_engine::layout::layout_system::URootUi` for roots
- `univis_ui_engine::layout::univis_node::UNode` for the box model
- `univis_ui_interaction::interaction::feedback::UInteraction` for runtime state
- `univis_ui_widgets::widget::text_label::UTextLabel` for text
- `univis_ui_style::style::Theme` for shared fonts and icon handles
