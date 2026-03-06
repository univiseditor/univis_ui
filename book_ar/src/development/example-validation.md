# تقرير التحقق من الأمثلة

## تاريخ التحقق

- March 6, 2026

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

- هذا التقرير يؤكد صلاحية البناء عبر `cargo check --release --example ...`.
- السلوك وقت التشغيل ما يزال يحتاج smoke checks يدوية (نافذة فعلية) للتفاعل والدقة البصرية.
