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
