# حالات `UInteraction`

الملف: `src/interaction/feedback.rs`

## transitions الأساسية

- `Pointer<Over>` => `Hovered`
- `Pointer<Out>` => `Normal`
- `Pointer<Press>` => `Pressed`
- `Pointer<Release>` => `Released`
- `Pointer<Click>` => `Clicked`

## ألوان `UInteraction`

إذا كان الكيان يحمل `UInteractionColors`، فإن مراقب التفاعل يحدّث `UNode.background_color` تلقائيًا حسب الحالة.

## ممارسات موصى بها

- استخدم `Pickable::IGNORE` على النصوص/الأبناء غير التفاعليين داخل زر.
- اجعل منطق الحالة النهائي داخل نظام الوحدة الجاهزة عند الحاجة، مثل السحب أو الاختيار، ولا تعتمد فقط على اللون.
