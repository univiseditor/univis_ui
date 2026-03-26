# `UInteraction` States

Core transitions:

- `None`
- `Hovered`
- `Pressed`
- `Disabled`

## `UInteractionColors`

When an entity carries `UInteractionColors`, the feedback observer updates `UNode.background_color` automatically from the current state.

## Recommended Practice

- use `Pickable::IGNORE` on decorative text or child entities inside a button
- keep final state logic inside widget systems when needed, rather than relying only on color
