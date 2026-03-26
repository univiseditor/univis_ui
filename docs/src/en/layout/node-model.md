# `UNode` and Box Metrics

`UNode` is the fundamental building block of every UI element.

## Main Fields

- `width`
- `height`
- `padding`
- `margin`
- `background_color`
- `border_radius`
- `shape_mode`

Supported units:

- `UVal::Px`
- `UVal::Percent`
- `UVal::Content`
- `UVal::Auto`
- `UVal::Flex`

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
