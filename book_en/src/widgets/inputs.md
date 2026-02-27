# Input Widgets

## USeekBar

- الملف: `src/widget/seekbar.rs`
- يدعم نطاقات وقيم وإظهار قيمة.
- event:
  - `SeekBarChangedEvent`

## UDragValue

- الملف: `src/widget/drag_value.rs`
- سحب أفقي لتغيير قيمة عددية.
- supports:
  - min/max
  - step
  - decimals
  - sensitivity
- events:
  - `DragValueChangedEvent`
  - `DragValueCommitEvent`

## USelect

- الملف: `src/widget/select.rs`
- dropdown select مع خيارات معطلة ودعم keyboard أساسي.
- events:
  - `SelectChangedEvent`
  - `SelectOpenStateChangedEvent`

## UTextField

- الملف: `src/widget/text_field.rs`
- text input مع cursor blink وfocus logic.
- plugin: `UnivisTextFieldPlugin` (اختياري).
- events:
  - `TextFieldChangedEvent`
  - `TextFieldSubmitEvent`
