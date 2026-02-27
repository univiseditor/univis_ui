# Pass Up/Down + Solver

## Pass Up: `upward_measure_pass_cached`

- يتحرك من أعمق مستوى إلى الجذر.
- يحسب `IntrinsicSize` للحاويات اعتمادًا على الأبناء.
- يتجاهل العناصر `Absolute` خارج التدفق.
- يستخدم cache لتخطي الحساب عند عدم الاتساخ.

## Pass Down: `downward_solve_pass_safe`

- يتحرك من الجذر إلى العمق الأقصى.
- يبني `SolverConfig` و`SolverSpec` لكل عنصر.
- يستدعي `solve_flex_layout` (المحرك الأساسي لكل الأنماط عبر bridge).
- يكتب النتائج إلى:
  - `ComputedSize`
  - `Transform.translation`

## Solver

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
