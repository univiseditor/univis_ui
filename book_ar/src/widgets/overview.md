# الـ Widgets

كل widget في Univis عبارة عن مكوّن ECS + plugin صغير يدير:

- تهيئة الشكل/الأبناء.
- منطق التفاعل.
- تحديث المرئيات.
- إطلاق events (Messages) عند الحاجة.

## ترتيب ذهني سريع

- عرض/نص:
  - `UTextLabel`, `UImage`, `UBadge`, `UTag`, `UProgressBar`
- Actions:
  - `UButton`, `UIconButton`, `UToggle`, `UCheckbox`, `URadioButton`
- Inputs:
  - `USeekBar`, `UDragValue`, `USelect`, `UTextField`
- Containers:
  - `UPanel`, `UPanelWindow`, `UScrollContainer`, `UDivider`

## Plugins المهمة

- مضافة تلقائيًا ضمن `UnivisWidgetPlugin`: أغلب widgets + scroll + panel.
- اختيارية حاليًا:
  - `UnivisTextFieldPlugin`
  - `UnivisBadgePlugin`
