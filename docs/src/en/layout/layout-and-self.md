# `ULayout` and `USelf` (Hybrid API)

The current design follows a hybrid API:

- flat fields for common use
- nested extension structs for advanced behavior

## `ULayout` (Container)

Common fields:

- `display`
- `flex_direction`
- `justify_content`
- `align_items`
- `gap`
- `grid_columns`

Advanced groups:

- `container_ext.box_align`
- `container_ext.flex`
- `container_ext.grid`

### Box Align Container

Holds advanced alignment controls such as:

- `justify_items`
- `align_content`
- `row_gap`
- `column_gap`

### Flex Container

Holds advanced flex controls such as:

- `wrap`
- `align_content`

### Grid Container

Holds advanced grid controls such as:

- `template_columns`
- `template_rows`
- `auto_flow`
- `auto_rows`
- `auto_columns`

## `USelf` (Item)

Common fields:

- `position_type`
- `left`, `right`, `top`, `bottom`
- `align_self`
- `order`

Advanced groups:

- `item_ext.box_align`
- `item_ext.flex`
- `item_ext.grid`

### Box Align Item

Includes:

- `justify_self`
- `align_self`

### Flex Item

Includes:

- `flex_grow`
- `flex_shrink`
- `flex_basis`

### Grid Item

Includes:

- `grid_column_start`
- `grid_column_span`
- `grid_row_start`
- `grid_row_span`
