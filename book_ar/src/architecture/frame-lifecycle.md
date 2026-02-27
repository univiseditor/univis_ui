# تدفق Frame Lifecycle

هذا الفصل يربط بين الأنظمة في الزمن داخل إطار Bevy واحد.

## PreUpdate

- `univis_picking_backend`:
  - يحول موقع المؤشر إلى world space.
  - يفحص تقاطع SDF مع كل عنصر تفاعلي.
  - يحترم القص من الآباء (`UClip`).
  - ينشر `PointerHits` لاستخدامها بواسطة observer events.

## Update

- أنظمة widgets (تحديث state، مزامنة visuals، إطلاق messages).
- أنظمة التمرير والـ panel resize وغيرها.
- أنظمة النص وتحديث حجمه.

## PostUpdate

- Layout pipeline (ترتيب critical):
  1. `update_layout_hierarchy`
  2. `upward_measure_pass_cached`
  3. `downward_solve_pass_safe`

- Render sync:
  - `update_materials_optimized` يزامن `UNode/ComputedSize/UBorder/...` إلى المواد.

## لماذا هذا الترتيب مهم؟

- أي تعديل في `Update` (مثل تغيير حجم panel أو قيمة input) يجب أن ينعكس في layout النهائي قبل الرسم.
- cache/invalidation يعمل قبل القياس لتقليل الحساب غير الضروري.
