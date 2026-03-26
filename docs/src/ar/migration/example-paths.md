# ترحيل مسارات الأمثلة

أصبحت الأمثلة موجودة الآن بجانب الـ crate التي تمثلها أساسًا.

## ترحيل الأوامر

كانت الأوامر القديمة غالبًا من الشكل:

```bash
cargo run --example root_fit_content
```

أما الآن فالأفضل استخدام أوامر مرتبطة بالحزمة:

```bash
cargo run -p univis_ui_engine --example root_fit_content
```

## قواعد ملكية الأمثلة

- يبقى الجذر `examples/` مخصصًا لعروض الواجهة المجمعة المتكاملة
- يحتوي `crates/univis_ui_engine/examples/` على أمثلة الجذور والتخطيط والرندر
- يحتوي `crates/univis_ui_widgets/examples/` على أمثلة الوحدات الجاهزة
- يحتوي `crates/univis_ui_interaction/examples/` على أمثلة التفاعل

## أمثلة شائعة على الترحيل

- أمثلة `root_*` -> `univis_ui_engine`
- أمثلة `layout_case_*` -> `univis_ui_engine`
- أمثلة `text_*` و`panel_*` و`scroll_view` و`select` و`toggle` -> `univis_ui_widgets`
- مثال `interaction` -> `univis_ui_interaction`
- أمثلة `hello_world` و`card_profile` و`sci_fi` و`complex_dashboard` -> `univis_ui`

## صفحات مرتبطة

- [فهرس الأمثلة](../examples/index.md)
- [الاختبارات والتحقق](../development/testing.md)
- [خطة التحقق السريع](../development/smoke-test-plan.md)
