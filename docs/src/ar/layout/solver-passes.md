# المرور الصاعد والهابط والحال

## المرور الصاعد: `upward_measure_pass_الذاكرة المؤقتةd`

- يتحرك من أعمق مستوى إلى الجذر.
- يحسب `IntrinsicSize` للحاويات اعتمادًا على الأبناء.
- يتجاهل العناصر `Absolute` خارج التدفق.
- يستخدم الذاكرة المؤقتة لتخطي الحساب عند عدم الاتساخ.

## المرور الهابط: `downward_solve_pass_safe`

- يتحرك من الجذر إلى العمق الأقصى.
- يبني `SolverConfig` و`SolverSpec` لكل عنصر.
- يستدعي `solve_flex_layout` (المحرك الأساسي لكل الأنماط عبر طبقة الربط).
- يكتب النتائج إلى:
  - `ComputedSize`
  - `Transform.translation`

## الحال

في `src/layout/core/solver.rs`:

- يفصل العناصر:
  - in-flow
  - absolute
- يحل main/cross sizes
- يطبق flex grow/shrink
- يطبق قواعد align/stretch
- يحل absolute box في نهاية الدورة

## translate_spec / translate_config

- `translate_config`: يقرأ container-level data من `ULayout`.
- `translate_spec`: يقرأ item-level data من `USelf`.

وهذا الفصل هو القلب الحسابي للمشروع.
