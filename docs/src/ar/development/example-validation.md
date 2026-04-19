# تقرير التحقق من الأمثلة

## تاريخ التحقق

- March 6, 2026

## ملاحظة الحالة

تمثل هذه الصفحة snapshot تاريخية من دورة أقدم ضمن تحضير `alpha`.
أما الفهرس الحالي المعتمد للأمثلة فهو موجود في [فهرس الأمثلة](../examples/index.md)، ولم يعد هذا الفرع يشحن أغلب مصادر الأمثلة المذكورة أدناه.

## وضع التحقق

- نمط التنفيذ: تسلسلي وبنسخة `release`
- تخفيف الضغط: `CARGO_BUILD_JOBS=1`
- الأمر المستخدم:

```bash
./scripts/check_examples_serial_release.sh
```

## النتيجة

- إجمالي الأمثلة المفحوصة: 28
- الناجح: 28
- الفاشل: 0

## قائمة الأمثلة التي تم فحصها

- `alignment`
- `border_light_3d`
- `card_profile`
- `drag_value`
- `ex_node`
- `hello_world`
- `interaction`
- `layout_cache`
- `layout_case_alignment_overflow`
- `layout_case_flex_wrap`
- `layout_case_grid_auto_flow`
- `layout_case_grid_tracks`
- `layout_case_masonry_ext`
- `layout_case_radial`
- `layout_case_stack`
- `masonry`
- `panel_divider`
- `panel_window`
- `radio`
- `sci_fi`
- `scroll_view`
- `seekbar`
- `select`
- `text_field`
- `text_label`
- `texture`
- `toggle`
- `widgets`

## ملاحظات

- يؤكد هذا التقرير صلاحية البناء لمجموعة الأمثلة التي كانت مشحونة حينها عبر `cargo check --release -p <package> --example ...`.
- السلوك وقت التشغيل ما يزال يحتاج smoke checks يدوية (نافذة فعلية) للتفاعل والدقة البصرية.

## حالة Runtime Smoke

- March 6, 2026: تم تأجيل/تخطي هذه المرحلة في هذه الدورة بطلب مباشر (قيود الجهاز/الموارد).
- أوامر smoke المخطط لها موثقة في [خطة Smoke Tests](smoke-test-plan.md).
