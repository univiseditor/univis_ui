# نظام التفاعل

طبقة التفاعل في Univis مبنية على:

- خلفية التقاط مخصصة (`univis_picking_backend`).
- نداءات مراقبة لحالات المؤشر.
- حالة المكوّن (`UInteraction`).

## المكونات الأساسية

- `UInteraction`:
  - `Normal`
  - `Hovered`
  - `Pressed`
  - `Released`
  - `Clicked`

- `UInteractionColors`:
  - `normal`
  - `hovered`
  - `pressed`

## فكرة العمل

1. خلفية الالتقاط تحسب الضربات.
2. نظام الالتقاط في Bevy يطلق أحداث المؤشر.
3. مراقبو الأحداث في `interaction/feedback.rs` تحدّث `UInteraction` واللون.

## ملاحظة

التفاعل يعتمد على وجود `UInteraction` على الكيان الهدف.

## مراجع مرتبطة

- [مصفوفة دعم التفاعل (الشاشة/العالم/ثلاثي الأبعاد)](support-matrix.md)
- [القيود الحالية](../development/current-limitations.md)

## أمثلة مرتبطة

- [`widgets_controls`](../examples/index.md#الوحدات-الجاهزة)
- [`widgets_inputs`](../examples/index.md#الوحدات-الجاهزة)
- [`widgets_containers`](../examples/index.md#الوحدات-الجاهزة)
- [`z_order_hierarchy`](../examples/index.md#أمثلة-مساحة-العمل)

## نقاط الدخول الرسمية في `API`

- `univis_ui_interaction::interaction::feedback::UInteraction`
- `univis_ui_interaction::interaction::feedback::UInteractionColors`
- `univis_ui_interaction::interaction::picking::univis_picking_backend`

## إلى أين بعد ذلك؟

- المثال المرتبط: [`widgets_controls`](../examples/index.md#الوحدات-الجاهزة)
- فهرس `API`: [مرجع الواجهة العامة](../api/index.md)
- صفحة مرجعية إضافية: [القيود الحالية](../development/current-limitations.md)
