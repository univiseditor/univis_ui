# `UScrollContainer`

File: `src/widget/scroll_view.rs`

## Component

`UScrollContainer` stores:

- scroll offset
- orientation
- scroll speed

## Plugin

- `UnivisScrollViewPlugin`

## Scroll Logic

- driven by `MouseWheel`
- scrolling only applies while the container is in `UInteraction::Hovered`
- the first child is expected to be the scrollable content
- the offset is clamped to `[-overflow, 0]`

## Practical Notes

- the container needs `UInteraction` to detect hover
- it is usually paired with `UClip { enabled: true }`

## Example

See: `crates/univis_ui_widgets/examples/scroll_view.rs`
