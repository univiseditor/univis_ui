# الوحدات الجاهزة

كل وحدة جاهزة في Univis عبارة عن مكوّن ECS مع إضافة صغيرة تدير:

- تهيئة الشكل/الأبناء.
- منطق التفاعل.
- تحديث المرئيات.
- إطلاق الرسائل عند الحاجة.

## ترتيب ذهني سريع

- عرض/نص:
  - `UTextLabel`, `UImage`, `UBadge`, `UTag`, `UProgressBar`
- أفعال:
  - `UButton`, `UIconButton`, `UToggle`, `UCheckbox`, `URadioButton`
- Inputs:
  - `USeekBar`, `UDragValue`, `USelect`, `UTextField`
- Containers:
  - `UPanel`, `UPanelWindow`, `UScrollContainer`, `UDivider`

## الإضافات المهمة

- مضافة تلقائيًا عبر `UnivisUiPlugin` -> `UnivisWidgetPlugin`: السطح القياسي للوحدات الجاهزة مع التمرير واللوحات وRuntime الخاص بـ `UTextField` و`UBadge`.
- وتبقى الإضافات المخصصة نفسها متاحة عندما تريد سطح widgets أضيق من `UnivisWidgetPlugin`.

## أمثلة مرتبطة

- [`widgets`](../examples/index.md#widgets) داخل `univis_ui_widgets`
- [`text_label`](../examples/index.md#text_label) داخل `univis_ui_widgets`
- [`text_field`](../examples/index.md#text_field) داخل `univis_ui_widgets`
- [`panel_window`](../examples/index.md#panel_window) داخل `univis_ui_widgets`
- [`scroll_view`](../examples/index.md#scroll_view) داخل `univis_ui_widgets`

## نقاط الدخول الرسمية في `API`

- `univis_ui_widgets::widget::text_label::UTextLabel`
- `univis_ui_widgets::widget::button::UButton`
- `univis_ui_widgets::widget::text_field::UTextField`
- `univis_ui_widgets::widget::panel::{UPanel, UPanelWindow}`
- `univis_ui_widgets::widget::scroll_view::UScrollContainer`

## إلى أين بعد ذلك؟

- المثال المرتبط: [`widgets`](../examples/index.md#widgets)
- صفحة الإعداد: [إعداد الإضافات وأولى الأمثلة](../first-steps.md)
- فهرس `API`: [مرجع الواجهة العامة](../api/index.md)
- صفحة الترحيل: [ترحيل مسارات الأمثلة](../migration/example-paths.md)
