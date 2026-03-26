# الوحدات الجاهزة

كل وحدة جاهزة في Univis عبارة عن مكوّن ECS مع إضافة صغيرة تدير:

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

## الإضافات المهمة

- مضافة تلقائيًا ضمن `UnivisWidgetPlugin`: أغلب الوحدات الجاهزة مع التمرير واللوحات.
- اختيارية حاليًا:
  - `UnivisTextFieldPlugin`
  - `UnivisBadgePlugin`
