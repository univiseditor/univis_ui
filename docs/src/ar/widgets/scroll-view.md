# حاوية التمرير

الملف: `src/widget/scroll_view.rs`

## المكوّن

`UScrollContainer`:

- `scroll_speed`
- `vertical`
- `horizontal`
- `offset`

## الإضافة

- `UnivisScrollViewPlugin`
- يسجّل `UScrollContainer` ضمن الانعكاس.
- يضيف `scroll_interaction_system` إلى `Update`.

## منطق التمرير

- يعتمد على `MouseWheel`.
- يطبّق التمرير فقط عندما container في حالة `UInteraction::Hovered`.
- يتوقع أن أول ابن للحاوية هو content القابل للتحريك.
- يطبّق clamp بالاعتماد على overflow:
  - المدى: `[-overflow, 0]`

## ملاحظات عملية

- يلزم `UInteraction` على الحاوية لاكتشاف hover.
- عادةً يدمج مع `UClip { enabled: true }` لإخفاء المحتوى الخارج عن الإطار.

## مثال

راجع: `crates/univis_ui_widgets/examples/scroll_view.rs`.
