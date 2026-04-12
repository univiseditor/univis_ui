# خطة الاختبارات السريعة

## الهدف

توفير قائمة تشغيل يدوي خفيفة بعد نجاح تحقق البناء التسلسلي بنسخة `release`.

## فحص مبدئي

شغّل تحقق البناء أولًا:

```bash
./scripts/check_representative_examples.sh
./scripts/verify_serial_release.sh
```

## سيناريوهات التشغيل اليدوي (بالأولوية)

1. فصل الواجهة الثابتة على الشاشة عن جذور العالم
2. القياس داخل العالم وسلوك الجذور المعتمدة على المحتوى
3. سلوك كبسولات الجذور عند التداخل
4. انتقالات تفاعل المؤشر
5. سلوك إدخال النص ورندر النص
6. سلوك تغيير حجم اللوحة
7. تحقق سريع للمسار البصري ثلاثي الأبعاد

## الأوامر

```bash
cargo run --release -p univis_ui --example root_screen_hud
cargo run --release -p univis_ui --example root_world_scale
cargo run --release -p univis_ui --example root_fit_content
cargo run --release -p univis_ui --example root_capsule_overlap
cargo run --release -p univis_ui --example interaction
cargo run --release -p univis_ui_widgets --example text_field
cargo run --release -p univis_ui_widgets --example text_label
cargo run --release -p univis_ui_widgets --example panel_window
cargo run --release -p univis_ui_engine --features example_bloom --example border_light_3d
```

## معايير النجاح

- لا يوجد panic عند البدء.
- يبقي `root_screen_hud` الواجهة الثابتة في مكانها بينما يتحرك الجذر العالمي مع الكاميرا.
- يبقي `root_world_scale` القياس المنطقي منفصلًا عن الحجم الفيزيائي في العالم.
- يلتف `root_fit_content` حول المحتوى المقاس من دون قص واضح أو نمو منفلت.
- يحافظ `root_capsule_overlap` على تراكب الجذور المغلق.
- إشارات التفاعل المتوقعة (hover/press/click) قابلة للملاحظة.
- `text_field` يقبل الإدخال ويصدر سلوك submit/change المتوقع.
- يبقى `text_label` مقروءًا مع القص والفيض و`autosize`.
- مقابض `panel_window` تستجيب مع السحب.
- مثال `border_light_3d` يعرض مسار 3D بشكل صحيح.

## التعامل مع الفشل

1. سجّل اسم المثال وعرض المشكلة.
2. أعد التشغيل مع `RUST_BACKTRACE=1`.
3. صنّف المشكلة: بناء أو تشغيل أو تفاعل أو رندر.
4. أضف ملاحظة issue فيها أمر إعادة الإنتاج وبيئة التشغيل.

راجع أيضًا: [التحقق البصري](visual-validation.md) و[الجاهزية للإصدار](release-readiness.md).
