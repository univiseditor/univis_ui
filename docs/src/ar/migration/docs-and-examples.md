# ترحيل سطح الوثائق والأمثلة

هذه الصفحة مخصصة لمن تعلّم المشروع عبر README قديم فقط، أو كتب منفصلة، أو أمثلة تشغيلية داخل
مساحة العمل لم تعد موجودة في هذا الفرع.

## ما الذي تغيّر

- أصبحت الوثائق موجودة في `mdBook` ثنائي اللغة واحد داخل `docs/`
- أصبحت الفصول العربية والإنجليزية متقابلة تحت `SUMMARY.md` واحد
- أصبح فهرس الأمثلة يتتبع حالة التوفر لا الملكية فقط
- أصبحت الحزمة الحية الحالية هي `android/android_phone_app`
- أصبحت قصة الجذور تدور حول `URootUi`

## مسار الاكتشاف القديم مقابل الجديد

المسار القديم:

- README
- أمثلة تشغيلية متناثرة
- `book_en` / `book_ar` منفصلان
- افتراضات واسعة حول الوحدات الداخلية

المسار الجديد:

1. `README.md` أو `README_AR.md`
2. `docs/src/index.md`
3. فصل الشرح المناسب
4. مدخل الحالة المرجعي داخل `docs/src/*/examples/index.md`
5. `rustdoc` المولد للمسارات والتواقيع الدقيقة

## أوامر الأمثلة في هذا الفرع

الأوامر القديمة مثل `cargo run --example hello_world` أو
`cargo run -p univis_ui --example root_fit_content` أصبحت تاريخية هنا فقط.

استخدم ما يلي بدلًا منها:

- `cargo run --manifest-path android/android_phone_app/Cargo.toml` للعرض الحي الحالي بطابع Android
- [فهرس الأمثلة](../examples/index.md) لمعرفة الملكية والحالة التاريخية
- [معرض الأمثلة](../examples/gallery.md) للاطلاع على الحالات الحية/الثابتة/المؤرشفة

## ترحيل وثائق الجذور

إذا كنت قد تعلّمت نموذج الجذور عبر `UScreenRoot` أو `UWorldRoot`، فابدأ الآن من:

- [ترحيل Root API إلى `URootUi`](root-api.md)
- [الجذور والمساحات](../layout/roots.md)

## المصادر المرجعية الحالية

- قصة الدخول: `README.md` / `README_AR.md`
- صفحات الشرح: `docs/src/en/*` و`docs/src/ar/*`
- خريطة توفر الأمثلة: `docs/src/en/examples/index.md` و`docs/src/ar/examples/index.md`
- مرجع `API`: ناتج `cargo doc --no-deps -p univis_ui`
