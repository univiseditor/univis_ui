# ملخص الترحيل

يشرح هذا الملف كيف تتنقل داخل المستودع بعد إعادة ترتيب الوثائق والأمثلة.

## حالة هذا الفرع حاليًا

- أصبحت الوثائق موجودة في `mdBook` ثنائي اللغة واحد داخل `docs/`
- أصبح السطح العام للجذور يتمحور حول `URootUi`
- أغلب ملفات الأمثلة القديمة في مساحة العمل أصبحت مؤرشفة، ولم تعد موجودة تحت `examples/` في الجذر أو تحت `crates/*/examples/`
- الحزمة الحية القابلة للتشغيل حاليًا هي `android/android_phone_app`
- المراجع البصرية الثابتة الحالية موجودة تحت `docs/src/assets/visual-references/`

## إذا كنت قادمًا من وثائق أقدم

اعتبر أوامر الأمثلة الأقدم مرجعًا تاريخيًا فقط في هذا الفرع.

أوامر مثل:

- `cargo run --example hello_world`
- `cargo run -p univis_ui --example root_fit_content`
- `cargo run -p univis_ui_widgets --example text_field`

تصف تخطيطات أقدم للمستودع، ولا تعكس الشجرة الحالية.

استخدم نقاط الدخول هذه بدلًا منها:

1. `README.md` أو `README_AR.md`
2. `docs/src/index.md`
3. فصل الشرح المناسب
4. `docs/src/en/examples/index.md` أو `docs/src/ar/examples/index.md` لفهم الملكية والحالة التاريخية
5. `cargo run --manifest-path android/android_phone_app/Cargo.toml` لتشغيل العرض الحي الحالي بطابع الهاتف

## السطح العام الرسمي

استخدم هذه النقاط باعتبارها الافتراضات العامة الحالية:

- الجذور الرسمية: `URootUi` و`URootUi::screen()` و`URootUi::world_2d(...)` و`URootUi::world_3d(...)`
- الأنواع الداعمة الرسمية: `UiSpace` و`UiCameraRef` و`UiCanvasSize`
- طبقات التوافق المهجورة: `UScreenRoot` و`UWorldRoot` على مسارات صريحة فقط
- مفتاح التوافق طويل المدى مع الحجم الفيزيائي القديم: `meters_per_unit: 1.0` على `URootUi`

وما يزال من المفيد إعادة التحقق منه أثناء الترحيل:

- جذور `fit-content` العالمية تحت أحجام نسبية قوية
- السلوك البصري في مشاهد شبيهة بـ `World3d`
- أي تغيير ثقيل في الرندر أو التخطيط أو الالتقاط ما يزال يحتاج تحققًا يدويًا

## حالات توفر الأمثلة

أصبح فهرس الأمثلة يميز بين ثلاث حالات:

- `حزمة حيّة`: المصدر ما يزال موجودًا ويمكن تشغيله مباشرة
- `مرجع ثابت`: ملف HTML أو صورة محفوظة للمقارنة البصرية
- `مصدر مؤرشف`: الاسم التاريخي للمثال ما يزال موثقًا، لكن ملف المصدر غير موجود في هذا الفرع

راجع:

- `docs/src/en/examples/index.md`
- `docs/src/ar/examples/index.md`
- `docs/src/en/examples/gallery.md`
- `docs/src/ar/examples/gallery.md`

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
- خريطة توفر الأمثلة: `docs/src/en/examples/index.md` و`docs/src/ar/examples/index.md`
- مرجع `API`: ناتج `cargo doc --no-deps -p univis_ui`
