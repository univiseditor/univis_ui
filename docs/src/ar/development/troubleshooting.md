# استكشاف الأخطاء

## 1) العناصر لا تظهر

تحقق من:

- وجود `Camera2d`.
- وجود root (`UScreenRoot` أو `UWorldRoot`).
- أن `ComputedSize` ليس صفرًا.
- أن `Visibility` ليست `Hidden`.

## 2) التفاعل لا يعمل

تحقق من:

- وجود `UInteraction` على الكيان التفاعلي.
- عدم اعتراض ابن/أب الالتقاط بشكل غير مقصود.
- عدم وجود قص ancestor يلغي hit.

## 3) النص يخرج خارج container مقصوص

- تأكد من `UClip { enabled: true }` على ancestor الصحيح.
- تأكد أن نظام `sync_text_clip_visibility` يعمل ضمن `UnivisTextPlugin`.

## 4) التمرير لا يعمل

- الكيان الحاوي يحمل `UScrollContainer` + `UInteraction`.
- الماوس في حالة hover على الحاوية.
- المحتوى أطول/أعرض من النافذة (overflow > 0).

## 5) panel resize لا يعمل

- تأكد من إضافة `UPanelWindow` مع `UPanel`.
- افحص `min_width/min_height` و`border_hit_thickness`.
- تأكد أن الواجهة ضمن مساحة قابلة للالتقاط من الكاميرا الحالية.

## مراجع مرتبطة

- [القيود الحالية](current-limitations.md)
