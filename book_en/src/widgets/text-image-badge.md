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

الأنظمة:

- `init_text_label_container`
- `sync_text_label_props`
- `fit_node_to_text_size`
- `sync_text_clip_visibility`

> `sync_text_clip_visibility` يضمن عدم ظهور النص خارج clip ancestors.

## UImage

الملف: `src/widget/image.rs`

- يربط صورة بخلفية/texture مسار material.
- `sync_image_geometry` يزامن أبعاد العرض.

## UBadge و UTag

الملف: `src/widget/badge.rs`

- أنماط badge (style/size presets).
- `UnivisBadgePlugin` مسؤول عن أنظمة التحديث الديناميكي للأنماط.
- هذا plugin **اختياري** وغير مسجل ضمن `UnivisWidgetPlugin` تلقائيًا.
