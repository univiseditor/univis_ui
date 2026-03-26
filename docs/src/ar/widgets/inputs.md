# وحدات الإدخال

## USeekBar

- الملف: `src/widget/seekbar.rs`
- يدعم نطاقات وقيم وإظهار قيمة.
- الحدث:
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
- الإضافة: `UnivisTextFieldPlugin` (اختيارية).
- events:
  - `TextFieldChangedEvent`
  - `TextFieldSubmitEvent`
