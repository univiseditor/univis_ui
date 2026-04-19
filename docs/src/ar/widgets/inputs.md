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
- Runtime مضاف افتراضيًا عبر `UnivisWidgetPlugin`.
- الإضافة المخصصة: `UnivisTextFieldPlugin` عند تركيب سطح widgets أضيق.
- events:
  - `TextFieldChangedEvent`
  - `TextFieldSubmitEvent`

## مثال حي

- استخدم `cargo run --example widgets_inputs` لرؤية `UTextField` و`USelect` و`UDragValue` و`USeekBar`
