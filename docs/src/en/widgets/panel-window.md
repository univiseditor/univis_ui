# `UPanel` and `UPanelWindow`

## `UPanel`

File: `src/widget/panel.rs`

Basic fields:

- width and height limits
- background styling
- panel frame behavior

Its role is to provide a ready-to-use panel container.

## `UPanelWindow`

`UPanelWindow` adds border and corner resize behavior on top of `UPanel`.

Fields:

- `min_width`
- `min_height`
- `border_hit_thickness`

## Current Behavior

- eight resize handles: `N`, `S`, `E`, `W`, `NE`, `NW`, `SE`, `SW`
- resize only, no drag-to-move behavior yet
- cursor icons only change on active resize handles
- the panel is converted to `Absolute + Px` on the first resize for predictable behavior

## Example

See: `crates/univis_ui_widgets/examples/panel_window.rs`
