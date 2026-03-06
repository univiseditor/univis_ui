# القيود الحالية

هذه الصفحة تجمع القيود الحالية المؤكدة من الكود، حتى تبقى إعدادات الاستخدام واضحة ومتوقعة.

## اعتماد التفاعل على الكاميرا

- `univis_picking_backend` يستعلم `Camera2d` حاليًا.
- تغيير حجم `UPanelWindow` يستعلم `Camera2d` حاليًا.
- النتيجة العملية: للحصول على تفاعل موثوق، أضف `Camera2d` في مشهد الواجهة النشط.

## Widget Plugins اختيارية

- `UnivisTextFieldPlugin` **غير** مسجل تلقائيًا داخل `UnivisWidgetPlugin`.
- `UnivisBadgePlugin` **غير** مسجل تلقائيًا داخل `UnivisWidgetPlugin`.
- النتيجة العملية: أضف هذه الـ plugins يدويًا عند الحاجة إلى سلوكها/أحداثها.

## أسطح Placeholder / غير مكتملة

- الملف `src/widget/menu.rs` ما يزال placeholder داخليًا فارغًا (غير معرّض ضمن الـ API العامة للـ widgets).

## مصادر التحقق

- `src/interaction/picking.rs`
- `src/widget/panel.rs`
- `src/widget/mod.rs`
- `src/widget/menu.rs`
- `src/layout/mod.rs`
