# خطة الاختبارات السريعة

## الهدف

توفير قائمة تشغيل يدوي خفيفة بعد نجاح تحقق البناء التسلسلي بنسخة `release`.

## فحص مبدئي

شغّل تحقق البناء أولًا:

```bash
./scripts/verify_serial_release.sh
```

## سيناريوهات التشغيل اليدوي (بالأولوية)

1. بدء التشغيل الأساسي + ظهور الجذر
2. انتقالات تفاعل المؤشر
3. سلوك التمرير
4. سلوك إدخال النص
5. سلوك تغيير حجم اللوحة
6. تحقق سريع لمسار 3D البصري

## الأوامر

```bash
cargo run --release -p univis_ui --example hello_world
cargo run --release -p univis_ui_interaction --example interaction
cargo run --release -p univis_ui_widgets --example scroll_view
cargo run --release -p univis_ui_widgets --example text_field
cargo run --release -p univis_ui_widgets --example panel_window
cargo run --release -p univis_ui_engine --features example_bloom --example border_light_3d
```

## معايير النجاح

- لا يوجد panic عند البدء.
- الواجهة تظهر وتستجيب.
- إشارات التفاعل المتوقعة (hover/press/click) قابلة للملاحظة.
- `text_field` يقبل الإدخال ويصدر سلوك submit/change المتوقع.
- مقابض `panel_window` تستجيب مع السحب.
- مثال `border_light_3d` يعرض مسار 3D بشكل صحيح.

## التعامل مع الفشل

1. سجّل اسم المثال وعرض المشكلة.
2. أعد التشغيل مع `RUST_BACKTRACE=1`.
3. صنّف المشكلة: بناء أو تشغيل أو تفاعل أو رندر.
4. أضف ملاحظة issue فيها أمر إعادة الإنتاج وبيئة التشغيل.
