# الوحدات التنفيذية

## `UButton`

- الملف: `src/widget/button.rs`
- يطبق نمط زر عبر `UNode` + `UInteractionColors`.
- الأنماط الجاهزة: `primary`, `secondary`, `danger`, `success`.

## `UIconButton`

- الملف: `src/widget/icon_btn.rs`
- نفس فكرة الزر مع دعم خط الأيقونات.

## `UToggle`

- الملف: `src/widget/toggle.rs`
- مفتاح تبديل مع تحريك للمقبض.
- الحدث:
  - `ToggleChangedEvent`

## `UCheckbox`

- الملف: `src/widget/checkbox.rs`
- منطق بسيط يعتمد نقرة المؤشر.

## `URadioButton` و`URadioGroup`

- الملف: `src/widget/radio.rs`
- إدارة مجموعة اختيار واحد.
- الحدث:
  - `RadioButtonChangedEvent`

## مثال حي

- استخدم `cargo run --example widgets_controls` لرؤية `UButton` و`UCheckbox` و`UToggle` و`URadioGroup` و`URadioButton` في شاشة واحدة
