# `UNode` and Box Metrics

`UNode` is the fundamental building block of every UI element.

## Main Fields

- `width`
- `height`
- `min_width`
- `max_width`
- `min_height`
- `max_height`
- `padding`
- `margin`
- `background_color`
- `border_radius`
- `shape_mode`

Supported units:

- `UVal::Px`
- `UVal::Percent`
- `UVal::MinContent`
- `UVal::MaxContent`
- `UVal::Content`
- `UVal::Auto`
- `UVal::Flex`

## Sizing Notes

- `width` and `height` are the preferred size request.
- `min_width`, `max_width`, `min_height`, and `max_height` clamp the solved result.
- `padding` participates in intrinsic measurement, while `margin` affects placement and wrap span rather than the node's inner measured content.
- `UVal::Content` is the legacy alias for `UVal::MaxContent`.
- `UVal::Auto` is contextual rather than identical to `Content`.
- Use `MaxContent` or `MinContent` when you want explicit intrinsic sizing without implicit auto-stretch behavior.
- `border_radius` and `shape_mode` stay visual-only; they do not change the measured layout size by themselves.

See [Sizing Semantics](sizing-semantics.md) for the current detailed rules.

## Final Size

After layout solving, every entity receives:

- `ComputedSize`

That final size is what rendering uses.

## `UBorder`

Borders are visually separate from the `UNode` background and support:

- width
- color
- rounded corners

## Shape Modes

- `Round`: rounded SDF corners
- `Cut`: beveled or cut corners
