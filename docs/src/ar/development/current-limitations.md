# القيود الحالية

هذه الصفحة تجمع القيود الحالية المؤكدة من الكود، حتى تبقى إعدادات الاستخدام واضحة ومتوقعة.

## اعتماد التفاعل على الكاميرا

- `univis_picking_backend` يحسم الكاميرا من كل `URootUi` عبر `ResolvedRootUi`، ولا يعتمد على `Camera2d` بشكل صلب.
- تغيير حجم `UPanelWindow` يتبع المسار نفسه المبني على كاميرا الجذر المحلولة.
- النتيجة العملية: يبقى التفاعل الموثوق مشروطًا بقدرة كل root نشط على حل كاميرا تملك viewport صالحًا.
- في المشاهد متعددة الكاميرات، يُفضّل استخدام `UiCameraRef::Entity(...)` بدل الاعتماد على الحل التلقائي.

## ملاحظات Runtime للوحدات الجاهزة

- `UnivisUiPlugin` يضيف `UnivisWidgetPlugin` تلقائيًا.
- `UnivisWidgetPlugin` يضم الآن Runtime المدمج الخاص بـ `UTextField` و`UBadge` افتراضيًا.
- تبقى الإضافات المخصصة نفسها متاحة عندما تريد سطح widgets أضيق.
- ما تزال أنظمة Runtime الخاصة بـ `UTag` محدودة، لذا اختبر المشاهد الثقيلة بالوسوم يدويًا.

## مصادر التحقق

- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`
- `crates/univis_ui_widgets/src/widget/mod.rs`
- `crates/univis_ui_widgets/src/widget/text_field.rs`
- `crates/univis_ui_widgets/src/widget/badge.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
