# ترحيل سطح التوثيق والأمثلة

هذه الصفحة موجهة لمن تعلّم المشروع سابقًا عبر `README` فقط، أو عبر الكتب المنفصلة، أو عبر افتراض أن كل الأمثلة تعيش في الجذر.

## ما الذي تغيّر

- أصبحت الوثائق موجودة في كتاب ثنائي اللغة واحد داخل `docs/`
- أصبحت الفصول العربية والإنجليزية متوازية تحت `SUMMARY.md` مشترك
- أصبحت الأمثلة موجودة بجانب الـ crate التي تمثلها أساسًا
- أصبحت أوامر الأمثلة المرتبطة بالـ package هي القاعدة
- أصبحت وثائق الجذور تدور حول `URootUi`

## مسار الاكتشاف القديم مقابل المسار الجديد

المسار القديم:

- `README`
- أمثلة متفرقة
- `book_en` و`book_ar` منفصلان
- افتراضات واسعة حول الوحدات الداخلية

المسار الجديد:

1. `README.md` أو `README_AR.md`
2. `docs/src/index.md`
3. فصل الشرح المناسب
4. المدخل المرجعي المناسب في `docs/src/*/examples/index.md`
5. `rustdoc` المولد للحصول على المسارات الدقيقة والتواقيع

## ترحيل أوامر الأمثلة

كانت الأوامر القديمة غالبًا تفترض package الجذر:

```bash
cargo run --example hello_world
```

أما الآن فتُشغَّل الأمثلة من خلال الـ package المالكة:

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui_engine --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui_interaction --example interaction
```

## ترحيل وثائق الجذور

إذا كنت قد تعلّمت نظام الجذور سابقًا عبر `UScreenRoot` أو `UWorldRoot`، فانتقل الآن إلى:

- [ترحيل الجذور إلى `URootUi`](root-api.md)
- [الجذور والمساحات](../layout/roots.md)

## ما هي المصادر المرجعية الآن

- قصة الدخول من GitHub: `README.md` و`README_AR.md`
- صفحات الشرح: `docs/src/en/*` و`docs/src/ar/*`
- فهرس الأمثلة: `docs/src/en/examples/index.md` و`docs/src/ar/examples/index.md`
- مرجع `API`: ناتج `cargo doc --no-deps -p univis_ui`
