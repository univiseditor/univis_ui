# نظام التخطيط

نظام التخطيط في `univis_ui` يعتمد على فكرة:

- Pass صاعد (قياس intrinsic)
- Pass هابط (حل القيود + التموضع)

ويعمل فوق مكونات أساسية:

- `UNode`: خصائص الصندوق والمرئيات الأساسية.
- `ULayout`: خصائص الحاوية (container behavior).
- `USelf`: خصائص العنصر الطفل (item behavior).

## الملفات المهمة

- `crates/univis_ui_engine/src/layout/univis_node.rs`
- `crates/univis_ui_engine/src/layout/core/pass_up.rs`
- `crates/univis_ui_engine/src/layout/core/pass_down.rs`
- `crates/univis_ui_engine/src/layout/core/solver.rs`
- `crates/univis_ui_engine/src/layout/core/layout_cache.rs`

## مبادئ أساسية

- تبدأ الجذور من `URootUi`.
- `LayoutDepth` يُحسب تلقائيًا عبر traversal.
- `IntrinsicSize` يُستخدم لتقدير المقاسات المعتمدة على المحتوى.
- `ComputedSize` هو الناتج النهائي المعتمد للرندر.
