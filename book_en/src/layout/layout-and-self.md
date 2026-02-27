# ULayout و USelf (Hybrid API)

التصميم الحالي يعتمد Hybrid API:

- حقول أساسية flat وسهلة الاستخدام.
- حقول متقدمة داخل nested ext structs.

## ULayout (Container)

الحقول الأساسية:

- `display`
- `flex_direction`
- `justify_content`
- `align_items`
- `gap`
- `grid_columns`

الحقول المتقدمة:

- `container_ext: ULayoutContainerExt`
  - `box_align: ULayoutBoxAlignContainer`
  - `flex: ULayoutFlexContainer`
  - `grid: ULayoutGridContainer`

### box_align container

- `justify_items: Option<UAlignItemsExt>`
- `align_content: Option<UContentAlignExt>`
- `row_gap: Option<f32>`
- `column_gap: Option<f32>`

### flex container

- `wrap: UFlexWrap`
- `align_content: Option<UContentAlignExt>`

### grid container

- `template_columns: Vec<UTrackSize>`
- `template_rows: Vec<UTrackSize>`
- `auto_flow: UGridAutoFlow`
- `auto_rows: UTrackSize`
- `auto_columns: UTrackSize`

## USelf (Item)

الحقول الأساسية:

- `align_self`
- `left/right/top/bottom`
- `order`
- `position_type`

الحقول المتقدمة:

- `item_ext: ULayoutItemExt`
  - `box_align: ULayoutBoxAlignSelf`
  - `flex: ULayoutFlexItem`
  - `grid: ULayoutGridItem`

### box_align item

- `justify_self: Option<UAlignSelfExt>`
- `align_self: Option<UAlignSelfExt>`
- `justify_overflow: UOverflowPosition`
- `align_overflow: UOverflowPosition`

### flex item

- `flex_grow: Option<f32>`
- `flex_shrink: Option<f32>`
- `flex_basis: Option<UVal>`

### grid item

- `column_start: Option<u32>`
- `column_span: u32`
- `row_start: Option<u32>`
- `row_span: u32`
