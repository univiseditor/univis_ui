# ترحيل مسارات الأمثلة

أصبحت هذه الصفحة تاريخية إلى حد كبير.

كانت فروع alpha الأقدم تنقل الأمثلة إلى جانب الـ crate التي تملكها أساسًا. أما هذا الفرع فلم يعد
يشحن أغلب تلك الأشجار المصدرية، لذلك استخدم هذه الصفحة كقاموس توافق للمشاكل القديمة وطلبات الدمج
والإشارات القديمة في الوثائق.

## واقع هذا الفرع الآن

- الحزمة الحية الحالية: `android/android_phone_app`
- المراجع الثابتة الحالية: `docs/src/assets/visual-references/`
- خريطة الملكية والحالة المؤرشفة: [فهرس الأمثلة](../examples/index.md)

## قواعد الملكية التاريخية

- كان الجذر `examples/` يحتفظ بعروض الواجهة المجمعة ومشاهد الجذور/التخطيط متعددة الـ crate
- كان `crates/univis_ui_engine/examples/` يحتفظ بعروض المحرك والتخطيط والرندر
- كان `crates/univis_ui_widgets/examples/` يحتفظ بعروض الوحدات الجاهزة
- كان `examples/interaction.rs` يحتفظ بمشهد التفاعل

## خرائط الملكية التاريخية

- أمثلة `root_*` -> `univis_ui`
- أمثلة `layout_case_*` -> `univis_ui`
- `alignment` و`ex_node` و`layout_cache` و`layout_sizing_semantics` و`runtime_benchmarks` -> `univis_ui`
- `text_*` و`panel_*` و`scroll_view` و`select` و`toggle` -> `univis_ui_widgets`
- `interaction` -> `univis_ui_interaction`
- `hello_world` و`card_profile` و`sci_fi` و`complex_dashboard` -> `univis_ui`

تعامل مع الخريطة أعلاه كسياق تاريخي فقط، لا كضمان أن الملفات المصدرية ما تزال موجودة.

## صفحات مرتبطة

- [فهرس الأمثلة](../examples/index.md)
- [الاختبارات والتحقق](../development/testing.md)
- [خطة اختبارات Smoke](../development/smoke-test-plan.md)
