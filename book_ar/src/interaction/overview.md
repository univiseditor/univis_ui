# نظام التفاعل Interaction

طبقة التفاعل في Univis مبنية على:

- Picking backend مخصص (`univis_picking_backend`).
- observer callbacks لحالات pointer.
- component state (`UInteraction`).

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

1. backend يحسب hits.
2. Bevy picking يطلق أحداث pointer.
3. observers في `interaction/feedback.rs` تحدّث `UInteraction` واللون.

## ملاحظة

التفاعل يعتمد على وجود `UInteraction` على الكيان الهدف.
