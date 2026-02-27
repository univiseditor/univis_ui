# Text Clipping Behavior

## المشكلة

النص (`Text2d`) لا يستخدم نفس مادة `UNodeMaterial`، لذلك لا يستفيد تلقائيًا من clip mask الموجود في shader.

## الحل الحالي في المشروع

في `src/widget/text_label.rs`:

- نظام `sync_text_clip_visibility`:
  - يحسب world quad للنص (باستخدام `Aabb` أو `TextLayoutInfo`).
  - يصعد في clip ancestors (`UClip`).
  - يخفي النص (`Visibility::Hidden`) إذا خرج عن أي clip ancestor.

## الميزة

- يمنع ظهور النص خارج إطار القص.
- يحافظ على اتساق السلوك البصري مع `UClip`.

## قيد معروف

- هذا الحل يعتمد visibility (all-or-nothing)، وليس قصًا بكسليًا جزئيًا للحروف.
- القص البكسلي للنص يتطلب مسار render text مخصص أو shader clipping مباشر في glyph path.
