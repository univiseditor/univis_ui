# خطة اختبارات Smoke

## الهدف

تقديم قائمة تشغيل يدوية خفيفة بعد اجتياز التحقق البنيوي.

## الفحص المسبق

ابدأ بالتحقق البنيوي أولًا:

```bash
./scripts/check_representative_examples.sh
./scripts/verify_serial_release.sh
```

## سيناريوهات التشغيل اليدوي

1. سلامة حزمة الشاشة الضيقة بطابع Android
2. مقارنة بصرية مع المراجع الثابتة المؤرشفة
3. مرور سريع على وضوح النصوص وعناصر التحكم

## الأوامر

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

وافتح عند الحاجة للمقارنة السريعة:

- `docs/src/assets/visual-references/responsive_dashboard.html`
- `docs/src/assets/visual-references/complex_dashboard.html`
- `docs/src/assets/visual-references/layout_solver_no_widgets.html`
- `docs/src/assets/visual-references/layout_solver_ultra_complex.html`

## معايير النجاح

- عدم حدوث panic عند البداية
- فتح الحزمة بطابع Android مع بقاء سطح التطبيق المقروء داخل العرض الضيق
- بقاء `UTextField` و`UToggle` و`USeekBar` و`UButton` متماسكة بصريًا وتفاعليًا
- بقاء المراجع الثابتة قريبة بما يكفي من اللغة البصرية المقصودة لمواد الإصدار

## تشخيص الإخفاق

1. سجّل السطح الذي فشل والعرض الظاهر للمشكلة
2. أعد تشغيل حزمة Android مع `RUST_BACKTRACE=1` إذا كان الإخفاق وقت التشغيل
3. صنّف المشكلة: compile أو runtime أو widget أو rendering
4. أضف ملاحظة في issue مع أمر إعادة الإنتاج وبيانات البيئة

راجع أيضًا: [التحقق البصري](visual-validation.md) و[جاهزية الإصدار](release-readiness.md).
