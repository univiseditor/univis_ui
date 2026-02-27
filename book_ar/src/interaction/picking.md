# Picking Backend

الملف: `src/interaction/picking.rs`

## ماذا يفعل؟

- يقرأ pointer locations.
- يحول المؤشر من viewport إلى world.
- يختبر كل عنصر مرشح عبر SDF rounded box.
- يستبعد الضربات المقصوصة بواسطة الآباء (`UClip`).
- يحذف hit الأب إذا يوجد hit ابن أعمق في نفس المسار.

## تفاصيل مهمة

- الاستعلام الأساسي يستهدف الكيانات التي تحمل `UInteraction`.
- العمق النهائي يجمع:
  - عمق الشجرة
  - ترجمة Z

## دقة القص

`is_clipped_by_ancestors`:

- يصعد في شجرة الآباء.
- يحول نقطة المؤشر إلى local لكل clip ancestor.
- يطبق `sd_rounded_box` على حدود clip.

هذا يجعل الالتقاط متسقًا مع منطق القص المرئي.
