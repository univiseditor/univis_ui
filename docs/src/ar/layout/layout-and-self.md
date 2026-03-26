# `ULayout` و`USelf` (واجهة هجينة)

التصميم الحالي يعتمد واجهة هجينة:

- حقول أساسية flat وسهلة الاستخدام.
- حقول متقدمة داخل nested ext structs.

## `ULayout` للحاوية

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

### محاذاة الحاوية

- `justify_items: Option<UAlignItemsExt>`
- `align_content: Option<UContentAlignExt>`
- `row_gap: Option<f32>`
- `column_gap: Option<f32>`

### إعدادات الحاوية المرنة

- `wrap: UFlexWrap`
- `align_content: Option<UContentAlignExt>`

### إعدادات الحاوية الشبكية

- `template_columns: Vec<UTrackSize>`
- `template_rows: Vec<UTrackSize>`
- `auto_flow: UGridAutoFlow`
- `auto_rows: UTrackSize`
- `auto_columns: UTrackSize`

## `USelf` للعنصر

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

### محاذاة العنصر

- `justify_self: Option<UAlignSelfExt>`
- `align_self: Option<UAlignSelfExt>`
- `justify_overflow: UOverflowPosition`
- `align_overflow: UOverflowPosition`

### إعدادات العنصر المرن

- `flex_grow: Option<f32>`
- `flex_shrink: Option<f32>`
- `flex_basis: Option<UVal>`

### إعدادات العنصر الشبكي

- `column_start: Option<u32>`
- `column_span: u32`
- `row_start: Option<u32>`
- `row_span: u32`
