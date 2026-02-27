# UInteraction States

الملف: `src/interaction/feedback.rs`

## transitions الأساسية

- `Pointer<Over>` => `Hovered`
- `Pointer<Out>` => `Normal`
- `Pointer<Press>` => `Pressed`
- `Pointer<Release>` => `Released`
- `Pointer<Click>` => `Clicked`

## UInteractionColors

إذا كان الكيان يحمل `UInteractionColors`، observer يحدّث `UNode.background_color` تلقائيًا حسب الحالة.

## ممارسات موصى بها

- استخدم `Pickable::IGNORE` على النصوص/الأبناء غير التفاعليين داخل زر.
- اجعل state logic النهائي داخل widget system عند الحاجة (مثل drag/select)، ولا تعتمد فقط على اللون.
