# جاهزية الإصدار

هذه الصفحة هي قائمة التحقق النهائية قبل قطع alpha التالية.

أدلة التحضير المحلية الحالية لـ `0.3.0` مسجلة في ملف الجذر `RELEASE_PREP_0.3.0.md`.

## الفحوص التي يجب إعادة تشغيلها

أعد فحص هذه الخطوات بالتسلسل:

```bash
cargo check --workspace --all-targets
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
./scripts/check_representative_examples.sh
```

ويبقى التحقق اليدوي وقت التشغيل مستحسنًا لمرور واحد على الأقل عبر الحزمة الحية الحالية.

## اتساق الوثائق والترحيل والإصدار

تأكد من أن هذه الملفات تصف الواقع الحالي نفسه:

- `README.md`
- `README_AR.md`
- `MIGRATION.md`
- `MIGRATION_AR.md`
- `RELEASE_NOTES.md`
- `changelog.md`

## القائمة المحلية الحالية

- [x] `mdbook build docs`
- [x] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets` عبر `./scripts/check_representative_examples.sh`
- [x] `./scripts/check_representative_examples.sh`
- [x] `./scripts/verify_alpha_release.sh`
- [x] أن تذكر ملاحظات الإصدار وصفحات الترحيل القصة الحالية نفسها لتوفر الأمثلة
- [ ] مرور بصري يدوي واحد على الحزمة ذات الطابع Android وعلى الأمثلة الحالية المرتبطة من [التحقق البصري](visual-validation.md)
- [ ] تأكيد GitHub Actions للبوابات نفسها
- [ ] فرز وإغلاق ملاحظات RC

## قرار إزالة الطبقات المتوافقة

في alpha التالية يكون القرار:

- لا تزل `UScreenRoot` أو `UWorldRoot` تلقائيًا ضمن القطع الحالي
- أبقهما كطبقتي توافق مهجورتين حتى تنتهي مراجعة استقرار أخرى
- أعد النظر في الإزالة فقط بعد شحن alpha التالية ووصول التغذية الراجعة

## أدوار ملفات الجذر

- `README.md`: قصة الدخول وأسرع نقطة بداية
- `README_AR.md`: صفحة الدخول العربية
- `MIGRATION.md`: مسار الترقية من الوثائق والأمثلة والافتراضات الأقدم
- `MIGRATION_AR.md`: مسار الترحيل العربي
- `RELEASE_NOTES.md`: الملخص الحالي على مستوى الإصدار alpha
- `RELEASE_PREP_0.3.0.md`: أدلة التحقق المحلية قبل قطع `0.3.0`
- `changelog.md`: السجل الزمني المرتب حسب التواريخ

## صفحات مرتبطة

- [الاختبارات والتحقق](testing.md)
- [التحقق البصري](visual-validation.md)
- [خطة اختبارات Smoke](smoke-test-plan.md)
