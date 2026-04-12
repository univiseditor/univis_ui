# ترحيل مسارات الأمثلة

أصبحت الأمثلة موجودة الآن بجانب الـ crate التي تمثلها أساسًا.

## ترحيل الأوامر

كانت الأوامر القديمة غالبًا من الشكل:

```bash
cargo run --example root_fit_content
```

أما الآن فالأفضل استخدام أوامر مرتبطة بالحزمة:

```bash
cargo run -p univis_ui --example root_fit_content
```

## قواعد ملكية الأمثلة

- يحتوي الجذر `examples/` على عروض الواجهة المجمعة وأمثلة الجذور/التخطيط متعددة الـ crate التي تحتاج إلى widgets أو interaction
- يحتوي `crates/univis_ui_engine/examples/` على أمثلة المحرك والتخطيط والرندر التي تبقى ضمن حدود المحرك نفسها
- يحتوي `crates/univis_ui_widgets/examples/` على أمثلة الوحدات الجاهزة
- يحتوي `examples/interaction.rs` في الحزمة الجذرية على مثال التفاعل

## أمثلة شائعة على الترحيل

- أمثلة `root_*` -> `univis_ui`
- أمثلة `layout_case_*` -> `univis_ui`
- أمثلة `alignment` و`ex_node` و`layout_cache` و`layout_sizing_semantics` و`runtime_benchmarks` -> `univis_ui`
- أمثلة `text_*` و`panel_*` و`scroll_view` و`select` و`toggle` -> `univis_ui_widgets`
- مثال `interaction` -> `univis_ui_interaction`
- أمثلة `hello_world` و`card_profile` و`sci_fi` و`complex_dashboard` -> `univis_ui`

## صفحات مرتبطة

- [فهرس الأمثلة](../examples/index.md)
- [الاختبارات والتحقق](../development/testing.md)
- [خطة التحقق السريع](../development/smoke-test-plan.md)
