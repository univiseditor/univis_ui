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

## Historical Example

- use `cargo run --example widgets_containers` for a live scroll viewport wired with `UClip`, `UInteraction`, and `UScrollContainer`

For a current scroll-container scene, run `cargo run --example widgets_containers`.
