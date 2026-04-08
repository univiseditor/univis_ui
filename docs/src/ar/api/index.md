# مرجع الواجهة العامة

هذا القسم يقدّم فهرسًا عمليًا لسطح الـ API العام في `univis_ui`.

## أين تجد المرجع الآلي الكامل؟

لتوليد rustdoc:

```bash
cargo doc --no-deps
```

ثم افتح:

```bash
cargo doc --no-deps --open
```

## كيف تستخدم هذا القسم؟

- ابدأ بـ [الأحداث والرسائل](events.md) إذا كنت تربط الواجهة بمنطق اللعبة أو التطبيق.
- استخدم [خريطة الحزم](crate-map.md) عندما تريد الاختيار بين الواجهة المجمعة والحزم الأدنى مستوى.
- راجع [جدول الأنواع العامة](public-types.md) للوصول السريع لكل بنية أو تعداد أو إضافة.

## نقاط الدخول الأعلى أهمية

- `univis_ui::UnivisUiPlugin` لمسار الواجهة المجمعة
- `univis_ui_engine::layout::layout_system::URootUi` للجذور
- `univis_ui_engine::layout::univis_node::UNode` لنموذج الصندوق
- `univis_ui_interaction::interaction::feedback::UInteraction` لحالة التفاعل
- `univis_ui_widgets::widget::text_label::UTextLabel` للنصوص
- `univis_ui_style::style::Theme` للخطوط والأيقونات المشتركة
