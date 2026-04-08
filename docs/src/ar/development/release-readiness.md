# الجاهزية للإصدار

هذه الصفحة هي قائمة التحقق النهائية قبل أي قطع alpha جديد.

## أمثلة الجذور المرجعية التي يجب إعادة التحقق منها

تحقق من البناء لهذه الأمثلة بالتسلسل:

```bash
cargo check -p univis_ui_engine --example root_screen_hud
cargo check -p univis_ui_engine --example root_world_scale
cargo check -p univis_ui_engine --example root_fit_content
cargo check -p univis_ui_engine --example root_capsule_overlap
```

يبقى التحقق اليدوي أثناء التشغيل مستحسنًا لجولة واحدة على الأقل من المجموعة نفسها.

## اتساق الوثائق والترحيل وملاحظات الإصدار

تأكد من أن هذه الملفات تصف الواقع الحالي نفسه:

- `README.md`
- `README_AR.md`
- `MIGRATION.md`
- `MIGRATION_AR.md`
- `RELEASE_NOTES.md`
- `changelog.md`

## القائمة النهائية

- [ ] `mdbook build docs`
- [ ] `cargo doc -p univis_ui_style --no-deps`
- [ ] `cargo doc -p univis_ui_engine --no-deps`
- [ ] `cargo doc -p univis_ui_interaction --no-deps`
- [ ] `cargo doc -p univis_ui_widgets --no-deps`
- [ ] `cargo doc -p univis_ui --no-deps`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui_engine`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui_widgets`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui_interaction`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui`
- [ ] جولة تحقق بصري واحدة على الأقل على الأمثلة المرجعية المذكورة في [التحقق البصري](visual-validation.md)
- [ ] أن تذكر ملاحظات الإصدار وصفحات الترحيل القصة العامة نفسها للجذور العامة

## قرار حذف طبقات التوافق

القرار للقطع alpha التالي هو:

- عدم حذف `UScreenRoot` أو `UWorldRoot` تلقائيًا مع القطع
- الإبقاء عليهما كطبقتي توافق مهجورتين إلى أن تكتمل مراجعة تثبيت إضافية
- إعادة النظر في الحذف فقط بعد صدور القطع التالي ووصول تغذية راجعة فعلية

## أدوار ملفات الجذر

- `README.md`: قصة الدخول وأسرع نقطة بدء
- `README_AR.md`: قصة الدخول العربية
- `MIGRATION.md`: مسار الترقية من الوثائق والأمثلة والافتراضات الأقدم
- `MIGRATION_AR.md`: مسار الترحيل العربي
- `RELEASE_NOTES.md`: الملخص الحالي على مستوى الإصدار alpha
- `changelog.md`: التاريخ الزمني المرتب حسب التواريخ

## صفحات مرتبطة

- [الاختبارات والتحقق](testing.md)
- [التحقق البصري](visual-validation.md)
- [خطة الاختبارات السريعة](smoke-test-plan.md)
