# Text و Image و Badge

## UTextLabel

الملف: `src/widget/text_label.rs`

الحقول الأساسية:

- `text`
- `font_size`
- `color`
- `font`
- `justify`
- `linebreak`
- `autosize`
- `overflow` افتراضيًا هو `Ellipsis`، ويمكن التحويل إلى `Clip` أو `Visible` عند الحاجة
- `truncate_side` يتحكم في القص مع `Ellipsis`: من البداية أو النهاية أو الوسط

الأنظمة:

- `measure_text_label_layout`
- `fit_node_to_text_size`
- `sync_text_label_meshes`
- `sync_text_clipper_materials`

> القص المحلي يتم على مستوى محارف النص، بينما ينتقل قص الأسلاف `UClip` إلى مادة النص كي يطبقه الشيدر.

## UImage

الملف: `src/widget/image.rs`

- يربط صورة بخلفية/texture مسار material.
- `sync_image_geometry` يزامن أبعاد العرض.
- `Auto` و`Content` و`MinContent` و`MaxContent` تتحول إلى الحجم الأصلي للصورة عندما تصبح الـ asset متاحة.
- تبقى قيود `UNode` مثل `min/max` مطبقة على الحجم النهائي بعد القياس الأصلي.

## UBadge و UTag

الملف: `src/widget/badge.rs`

- أنماط badge (style/size presets).
- `UnivisBadgePlugin` مسؤول عن أنظمة التحديث الديناميكي للأنماط.
- هذا الـ runtime مضاف افتراضيًا عبر `UnivisWidgetPlugin`.
- وتبقى الإضافة المخصصة نفسها متاحة عندما تريد سطح widgets أضيق.

## مثال حي

- استخدم `cargo run --example widgets_display` لرؤية `UBadge` و`UDivider` و`UProgressBar` و`UPanel`
