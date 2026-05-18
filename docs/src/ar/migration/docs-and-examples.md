# ترحيل سطح الوثائق والأمثلة

تشرح هذه الصفحة شكل الوثائق والأمثلة الحالي.

## ما الذي تغيّر

- الوثائق موجودة في `mdBook` ثنائي اللغة واحد داخل `docs/`
- الفصول العربية والإنجليزية متقابلة تحت `SUMMARY.md` واحد
- فهرس الأمثلة يعرض فقط ملفات المصدر الموجودة في هذا الفرع
- حزمة العرض المستقلة هي `android/android_phone_app`
- قصة الجذور تدور حول `URootUi`

## مسار الاكتشاف الحالي

1. `README.md` أو `README_AR.md`
2. `docs/src/index.md`
3. فصل الشرح المناسب
4. قائمة الأمثلة الحالية داخل `docs/src/*/examples/index.md`
5. `rustdoc` المولد للمسارات والتواقيع الدقيقة

## أوامر الأمثلة في هذا الفرع

استخدم هذه الأوامر مع شجرة العمل الحالية:

- `cargo run --manifest-path android/android_phone_app/Cargo.toml`
- `cargo check --workspace --examples`
- `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets`

وللعرض بحسب المهمة، راجع:

- [فهرس الأمثلة](../examples/index.md)
- [معرض الأمثلة](../examples/gallery.md)

## ترحيل وثائق الجذور

إذا كنت قد تعلّمت نموذج الجذور عبر `UScreenRoot` أو `UWorldRoot`، فابدأ الآن من:

- [ترحيل Root API إلى `URootUi`](root-api.md)
- [الجذور والمساحات](../layout/roots.md)

## المصادر المرجعية الحالية

- قصة الدخول: `README.md` / `README_AR.md`
- صفحات الشرح: `docs/src/en/*` و`docs/src/ar/*`
- الأمثلة الحالية: `docs/src/en/examples/index.md` و`docs/src/ar/examples/index.md`
- مرجع `API`: ناتج `cargo doc --no-deps -p univis_ui`
