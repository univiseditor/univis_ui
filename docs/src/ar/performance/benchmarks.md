# حزمة القياس المعياري

يوجد الآن مسار قياس معياري من سطر الأوامر يتكون من حزمتين:

- السكربت: `./scripts/run_perf_baselines.sh`
- مثال المحلل: `cargo run -p univis_ui_engine --release --example solver_benchmarks`
- مثال المشاهد التشغيلية: `cargo run -p univis_ui_engine --release --example runtime_benchmarks`

## التغطية الحالية

التغطية الحالية تجمع بين المسارات الخوارزمية الثقيلة والمسارات التشغيلية الثقيلة.

سيناريوهات المحلل:

- `dense_row_512`
- `wrap_cards_400`
- `grid_dashboard_196`

سيناريوهات التشغيل:

- `root_capsules_96`
- `idle_after_settle_96`
- `single_root_local_change_96`
- `render_only_change_512`
- `text_measure_180`
- `picking_grid_512`
- `widget_panels_240`
- `world3d_panels_48`

تغطي حزمة المحلل صف `flex` كثيفاً، وتخطيط بطاقات ملتفاً، ولوحة `grid` ثقيلة.
وتضيف الحزمة التشغيلية لقطات الاستقرار بعد التسوية، وتغييرات محلية على جذر واحد،
وتغييرات بصرية دون تبديل الهندسة، إلى جانب حل الجذور وترتيبها، وقياس النصوص،
ومسار الالتقاط، ومشاهد اللوحات الثقيلة، ومشاهد `World3d`.

## طريقة التشغيل

```bash
./scripts/run_perf_baselines.sh
```

ولتشغيل حزمة واحدة مباشرة:

```bash
cargo run -p univis_ui_engine --release --example solver_benchmarks
cargo run -p univis_ui_engine --release --example runtime_benchmarks
```

لتقصير مدة التشغيل أثناء العمل المحلي:

```bash
UNIVIS_PERF_WARMUP=12 UNIVIS_PERF_ITERATIONS=40 ./scripts/run_perf_baselines.sh
```

ولجعل الأمر يفشل عندما يتجاوز سيناريو ميزانيته الحالية:

```bash
./scripts/run_perf_baselines.sh --check
```

## ماذا يعرض؟

تعرض الحزمة:

- اسم السيناريو
- عدد العناصر
- متوسط زمن التحديث أو الحل بالميلي ثانية
- قيمة `p95`
- أعلى زمن مسجل
- الميزانية الحالية لكل سيناريو
- الحالة (`ok` / `over`)

تُستخدم قيمة `p95` كميزانية حالية لأنها أكثر ثباتاً من أسوأ عينة منفردة.

## الميزانيات الحالية

ميزانيات المحلل:

- `dense_row_512`: `1.000ms` عند `p95`
- `wrap_cards_400`: `1.400ms` عند `p95`
- `grid_dashboard_196`: `1.800ms` عند `p95`

ميزانيات التشغيل:

- `root_capsules_96`: `6.000ms` عند `p95`
- `idle_after_settle_96`: `2.000ms` عند `p95`
- `single_root_local_change_96`: `1.200ms` عند `p95`
- `render_only_change_512`: `1.500ms` عند `p95`
- `text_measure_180`: `4.750ms` عند `p95`
- `picking_grid_512`: `4.000ms` عند `p95`
- `widget_panels_240`: `8.000ms` عند `p95`
- `world3d_panels_48`: `6.000ms` عند `p95`

تمثل هذه الأرقام خطوطاً مرجعية داخل المستودع للحزمتين الحاليتين، وليست وعداً مطلقاً لكل الأجهزة.
الغرض منها هو كشف الانحرافات داخل هذا المشروع وبنفس شكل القياس.
