# نظام التخطيط Layout

نظام التخطيط في `univis_ui` يعتمد على فكرة:

- Pass صاعد (قياس intrinsic)
- Pass هابط (حل القيود + التموضع)

ويعمل فوق مكونات أساسية:

- `UNode`: خصائص box والمرئيات الأساسية.
- `ULayout`: خصائص الحاوية (container behavior).
- `USelf`: خصائص العنصر الطفل (item behavior).

## الملفات المهمة

- `src/layout/univis_node.rs`
- `src/layout/core/pass_up.rs`
- `src/layout/core/pass_down.rs`
- `src/layout/core/solver.rs`
- `src/layout/core/layout_cache.rs`

## مبادئ أساسية

- الـ roots هي نقطة البداية: `UScreenRoot` أو `UWorldRoot`.
- `LayoutDepth` يُحسب تلقائيًا عبر traversal.
- `IntrinsicSize` يُستخدم لتقدير المقاسات المعتمدة على المحتوى.
- `ComputedSize` هو الناتج النهائي المعتمد للرندر.
