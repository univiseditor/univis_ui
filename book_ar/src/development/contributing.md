# التطوير والمساهمة

## قواعد عملية قبل أي تعديل

1. افهم system order قبل التعديل (خصوصًا layout/render/interaction).
2. لا تكسر `Reflection` للأنواع العامة بلا سبب.
3. اختبر التعديل على أمثلة فعلية وليس فقط unit tests.
4. عند إضافة widget جديد:
   - component API واضح
   - plugin مستقل
   - event messages إن لزم
   - مثال demo
   - توثيق في README + هذا الكتاب

## أسلوب الكود داخل المشروع

- ECS-first.
- مكونات صغيرة وأنظمة واضحة.
- سلوك مرئي عبر `UNode/UBorder/UInteractionColors`.
- تجنب side effects غير متوقعة خارج schedule المقصود.

## فروع العمل المقترحة

- `feat/<name>` للميزات.
- `fix/<name>` للإصلاحات.
- `docs/<name>` للتوثيق.
