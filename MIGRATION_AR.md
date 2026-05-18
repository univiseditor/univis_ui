# ملخص الترحيل

يشرح هذا الملف شكل المستودع الحالي بعد تنظيف الوثائق والأمثلة.

## حالة هذا الفرع حاليًا

- الوثائق موجودة في `mdBook` ثنائي اللغة واحد داخل `docs/`
- السطح العام للجذور يتمحور حول `URootUi`
- فهرس الأمثلة يعرض فقط ملفات المصدر الموجودة في هذا الفرع
- حزمة العرض المستقلة هي `android/android_phone_app`
- أمثلة مساحة العمل موجودة تحت `examples/` ومسجلة في `Cargo.toml`

## نقاط دخول الأمثلة الحالية

شغّل العرض ذي الطابع Android:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

افحص كل أمثلة مساحة العمل التي يعرفها Cargo:

```bash
cargo check --workspace --examples
```

راجع قائمة الأمثلة الحالية:

- `docs/src/en/examples/index.md`
- `docs/src/ar/examples/index.md`
- `docs/src/en/examples/gallery.md`
- `docs/src/ar/examples/gallery.md`

## السطح العام الرسمي

استخدم هذه النقاط باعتبارها الافتراضات العامة الحالية:

- الجذور الرسمية: `URootUi` و`URootUi::screen()` و`URootUi::world_2d(...)` و`URootUi::world_3d(...)`
- الأنواع الداعمة الرسمية: `UiSpace` و`UiCameraRef` و`UiCanvasSize`
- طبقات التوافق المهجورة: `UScreenRoot` و`UWorldRoot` على مسارات صريحة فقط
- الحجم الفيزيائي داخل العالم يتحكم به `meters_per_unit` على `URootUi`

وما يزال من المفيد إعادة التحقق منه أثناء الترحيل:

- جذور `fit-content` العالمية تحت أحجام نسبية قوية
- السلوك البصري في مشاهد شبيهة بـ `World3d`
- أي تغيير ثقيل في الرندر أو التخطيط أو الالتقاط يحتاج تحققًا يدويًا

## صفحات الترحيل المرتبطة

- `docs/src/ar/migration/docs-and-examples.md`
- `docs/src/ar/migration/root-api.md`
- `docs/src/ar/migration/legacy-compatibility.md`
- `docs/src/ar/migration/example-paths.md`

ونظيراتها الإنجليزية:

- `docs/src/en/migration/docs-and-examples.md`
- `docs/src/en/migration/root-api.md`
- `docs/src/en/migration/legacy-compatibility.md`
- `docs/src/en/migration/example-paths.md`

## ما هو المرجع الأساسي الآن

- قصة المشروع وخريطة الحزم: `README.md`
- صفحة الدخول العربية: `README_AR.md`
- صفحات الشرح: `docs/src/en/*` و`docs/src/ar/*`
- قائمة الأمثلة الحالية: `docs/src/en/examples/index.md` و`docs/src/ar/examples/index.md`
- مرجع `API`: ناتج `cargo doc --no-deps -p univis_ui`
