# Display Modes

`UDisplay` currently supports:

- `Flex`
- `Grid`
- `Masonry`
- `Stack`
- `Radial`
- `None`

## Flex

- uses `flex_direction`, `justify_content`, and `align_items`
- supports wrapping through `container_ext.flex.wrap`
- supports `flex_grow`, `flex_shrink`, and `flex_basis` on items

## Grid

- can run in a simple mode through `grid_columns`
- or in an advanced mode through `template_rows` and `template_columns`
- supports auto-placement through `auto_flow`, `auto_rows`, and `auto_columns`

## Masonry

- places each item in the shortest column
- useful for Pinterest-like layouts

## Stack

- overlays children in the same space

## Radial

- distributes items around a circle
- useful for sci-fi or menu-like layouts

## None

- disables layout behavior for the container
