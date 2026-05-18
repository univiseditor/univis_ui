# الاختبارات والتحقق

## اختبارات الوحدة

لتخفيف الضغط على الأجهزة الأضعف، شغّل الاختبارات هدفًا واحدًا في كل مرة:

```bash
cargo test --release <test_name> --lib
```

## التحقق من الجودة

```bash
./scripts/check_quality.sh
./scripts/check_representative_examples.sh
./scripts/check_examples_serial_release.sh
```

يشغّل `./scripts/check_quality.sh` ما يلي:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets` مع قائمة السماح الحالية للديون المعروفة المرتبطة بـ Bevy
- `cargo check --workspace --all-targets`
- `./scripts/check_public_api_surface.sh`
- `./scripts/check_structure_guardrails.sh`
- `./scripts/check_engine_boundary_guardrails.sh`
- `cargo test --workspace --lib`

أما `./scripts/check_representative_examples.sh` فأصبح يترجم حزمة Android الحالية ثم يفحص أي أدلة أمثلة ما تزال موجودة في مساحة العمل.

ويظل `./scripts/check_examples_serial_release.sh` مفيدًا عندما يحتوي الفرع فعليًا على أهداف `examples/*.rs`؛ أما في هذا الفرع فقد يكتفي بالإبلاغ عن عدم وجود أمثلة تشغيلية داخل مساحة العمل.

## التحقق من الوثائق

```bash
cargo doc --no-deps
mdbook build docs
```

## خطوط الأداء الأساسية

استخدم هذه الملفات الملتزم بها:

- `perf_baselines/current_max/2026-04-05/solver_benchmarks.txt`
- `perf_baselines/current_max/2026-04-05/runtime_benchmarks.txt`
- `perf_baselines/current_max/2026-04-05/manifest.json`

## التحقق داخل CI

ما تزال GitHub Actions تتحقق من الجودة والوثائق و`API Docs` وفحوص الأمثلة الحالية عبر سير العمل الحالية.

## التحقق التسلسلي على الأجهزة الضعيفة

```bash
# كل اختبارات lib واحدة تلو الأخرى
./scripts/test_lib_serial_release.sh

# كل أمثلة مساحة العمل الحالية التي يعرفها السكربت
./scripts/check_examples_serial_release.sh

# حزمة Android الحالية مع فحص أمثلة مساحة العمل
./scripts/check_representative_examples.sh

# التحقق الكامل: اختبارات lib + الأمثلة الحالية
./scripts/verify_serial_release.sh

# التحقق التسلسلي لحزمة مساحة عمل محددة
./scripts/test_lib_serial_release.sh -p univis_ui_engine
./scripts/check_examples_serial_release.sh -p univis_ui_engine
./scripts/verify_serial_release.sh -p univis_ui_engine

# حزمة Android مباشرة
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets

# تحقق alpha قبل الإصدار
./scripts/verify_alpha_release.sh

# بناءات alpha فقط من دون تحقق
./scripts/package_alpha_serial.sh --no-verify
```

## إستراتيجية عملية قبل الدمج

1. شغّل اختبارات الوحدة المرتبطة بالتغيير
2. شغّل `./scripts/check_quality.sh`
3. شغّل `./scripts/check_representative_examples.sh`
4. شغّل `cargo check --workspace --examples`
5. شغّل حزمة Android عندما يمس التغيير السطح الحي الحالي
6. استخدم [التحقق البصري](visual-validation.md) عندما يكون التغيير ثقيلًا في الرندر أو التخطيط أو التفاعل

## المطلوب قبل قطع alpha التالية

- `./scripts/check_quality.sh`
- `mdbook build docs`
- `cargo doc -p univis_ui_style --no-deps`
- `cargo doc -p univis_ui_engine --no-deps`
- `cargo doc -p univis_ui_interaction --no-deps`
- `cargo doc -p univis_ui_widgets --no-deps`
- `cargo doc -p univis_ui --no-deps`
- `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets`
- `./scripts/check_representative_examples.sh`
- مرور يدوي واحد عبر حزمة Android والأمثلة الحالية المرتبطة
- مرور واحد عبر [جاهزية الإصدار](release-readiness.md)

## سياسة لقطات الشاشة

- تبقى لقطات الشاشة مواد تحضير يدوية قبل الإصدار
- يمكن لمعرض الأمثلة أن يربط إلى مراجع بصرية ثابتة
- توليد لقطات الشاشة ليس بوابة CI إلزامية في خط alpha الحالي
