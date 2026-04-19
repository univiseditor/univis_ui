# Input Widgets

## `USeekBar`

- file: `src/widget/seekbar.rs`
- supports ranges, values, and optional value display

## `UDragValue`

- file: `src/widget/drag_value.rs`
- horizontal dragging changes a numeric value

## `USelect`

- file: `src/widget/select.rs`
- dropdown selection with disabled options and basic keyboard handling

## `UTextField`

- file: `src/widget/text_field.rs`
- text input with cursor blink and focus logic
- runtime coverage: included by default through `UnivisWidgetPlugin`
- dedicated plugin: `UnivisTextFieldPlugin` when composing a narrower widget surface

## Live Example

- use `cargo run --example widgets_inputs` for `UTextField`, `USelect`, `UDragValue`, and `USeekBar`
