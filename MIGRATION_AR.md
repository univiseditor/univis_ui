# ملخص الترحيل

يستبدل هذا الملف ملفات خارطة الطريق المؤقتة الخاصة بالتوثيق والأمثلة والوثائق البرمجية المولدة بعد أن تم تطبيق الترحيل فعليًا داخل المستودع.

## متى تقرأ هذا الملف

اقرأ `MIGRATION_AR.md` عندما تأتي من وثائق أقدم، أو من مسارات أمثلة أقدم، أو من افتراضات أقدم حول الجذور.

واستخدم بقية ملفات الجذر لأهداف مختلفة:

- `README.md`: قصة الدخول وأسرع نظرة عامة
- `README_AR.md`: قصة الدخول العربية
- `RELEASE_NOTES.md`: الملخص الحالي على مستوى الإصدار alpha
- `changelog.md`: السجل الزمني للتغييرات الملحوظة

## ما الذي تغيّر

- أصبحت الوثائق موجودة في `mdBook` ثنائي اللغة واحد داخل `docs/`
- أصبحت الفصول العربية والإنجليزية متوازية تحت `docs/src/SUMMARY.md` مشترك
- أصبحت الأمثلة موجودة بجانب الـ crate التي تملكها أساسًا
- أصبحت أوامر الأمثلة المرتبطة بالحزمة هي القاعدة
- أصبحت وثائق `API` المولدة جزءًا من مسار التعلم الموثق
- أصبح نموذج الجذر العام يدور حول `URootUi`

## مسار الاكتشاف الجديد

استخدم هذا الترتيب عند التنقل داخل المشروع:

1. `README.md` أو `README_AR.md`
2. `docs/src/index.md`
3. فصل الشرح المناسب
4. `docs/src/en/examples/index.md` أو `docs/src/ar/examples/index.md`
5. ناتج `cargo doc --no-deps -p univis_ui`

## ترحيل أوامر الأمثلة

كانت الأوامر القديمة كثيرًا ما تفترض حزمة الجذر:

```bash
cargo run --example hello_world
```

أما الآن فالأوامر الصحيحة مرتبطة بالحزمة المالكة:

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui_engine --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui_interaction --example interaction
```

## ترحيل الجذور والتوثيق

إذا كنت قد تعلّمت المشروع عبر الوثائق الأقدم أو عبر طبقات الجذور القديمة، فابدأ من هذه الصفحات:

- `docs/src/ar/migration/docs-and-examples.md`
- `docs/src/ar/migration/root-api.md`
- `docs/src/ar/migration/example-paths.md`

ونظيراتها الإنجليزية:

- `docs/src/en/migration/docs-and-examples.md`
- `docs/src/en/migration/root-api.md`
- `docs/src/en/migration/example-paths.md`

## ما هو المصدر المرجعي الآن

- قصة المشروع وخريطة الحزم: `README.md`
- صفحة الدخول العربية: `README_AR.md`
- ملاحظة استقرار `alpha2`: `ALPHA2_STATUS_AR.md`
- صفحات الشرح: `docs/src/en/*` و`docs/src/ar/*`
- فهرس الأمثلة: `docs/src/en/examples/index.md` و`docs/src/ar/examples/index.md`
- مرجع `API`: ناتج `cargo doc --no-deps -p univis_ui`

## لماذا أزيلت ملفات خارطة الطريق

كانت قوائم خارطة الطريق مفيدة أثناء التنفيذ، لكنها أصبحت بعد الإنجاز مجرد ملاحظات تاريخية داخلية. لذلك يحتفظ الجذر الآن بملف ترحيل واضح بدل ملفات تخطيط مفتوحة انتهى دورها.
