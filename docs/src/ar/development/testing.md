# الاختبارات والتحقق

## اختبارات وحدة (فردي)

لتخفيف الحمل، شغّل الاختبارات كل واحدة على حدة:

```bash
cargo test --release <test_name> --lib
```

## تحقق البناء

```bash
cargo check --workspace --all-targets
./scripts/check_examples_serial_release.sh
```

## تحقق التوثيق

```bash
cargo doc --no-deps
mdbook build docs
```

## التحقق داخل `CI`

أصبح GitHub Actions يتحقق من التوثيق والأمثلة و`API Docs` عبر:

- `.github/workflows/docs_examples_api.yml`
- `.github/workflows/docs_publish.yml`

ويشغّل:

- `mdbook build docs`
- `cargo doc --no-deps` لكل crate عامة
- فحص الأمثلة لكل package على حدة عبر `./scripts/check_examples_serial_release.sh -p ...`
- مسار نشر مستقل لصفحات الوثائق العامة على `main`

## للأجهزة الضعيفة (تشغيل تسلسلي)

```bash
# كل اختبارات lib واحدة واحدة
./scripts/test_lib_serial_release.sh

# كل الأمثلة واحدة واحدة
./scripts/check_examples_serial_release.sh

# التحقق الكامل: lib tests + examples
./scripts/verify_serial_release.sh

# تحقق تسلسلي لحزمة محددة داخل workspace
./scripts/test_lib_serial_release.sh -p univis_ui_engine
./scripts/check_examples_serial_release.sh -p univis_ui_engine
./scripts/verify_serial_release.sh -p univis_ui_engine

# تحقق alpha قبل النشر: check + lib tests + examples + package
./scripts/verify_alpha_release.sh

# إنشاء حزم alpha فقط بدون verify
./scripts/package_alpha_serial.sh --no-verify
```

لتمرير أمثلة محددة فقط:

```bash
./scripts/check_examples_serial_release.sh -p univis_ui hello_world
./scripts/check_examples_serial_release.sh -p univis_ui_interaction interaction
./scripts/check_examples_serial_release.sh -p univis_ui_widgets select
```

## استراتيجية عملية قبل الدمج

1. شغل اختبارات الوحدة الخاصة بالتعديل.
2. شغل `cargo check --workspace --all-targets`.
3. شغل `./scripts/check_examples_serial_release.sh`.
4. جرّب مثال واحد على الأقل مرتبط بالتعديل.
5. استخدم [التحقق البصري](visual-validation.md) عندما يكون التعديل غنيًا بالرندر أو التخطيط أو التفاعل.

## المطلوب قبل إصدار alpha القادم

- `cargo check --workspace --all-targets`
- `mdbook build docs`
- `cargo doc -p univis_ui_style --no-deps`
- `cargo doc -p univis_ui_engine --no-deps`
- `cargo doc -p univis_ui_interaction --no-deps`
- `cargo doc -p univis_ui_widgets --no-deps`
- `cargo doc -p univis_ui --no-deps`
- `./scripts/check_examples_serial_release.sh -p univis_ui_engine`
- `./scripts/check_examples_serial_release.sh -p univis_ui_widgets`
- `./scripts/check_examples_serial_release.sh -p univis_ui_interaction`
- `./scripts/check_examples_serial_release.sh -p univis_ui`
- جولة يدوية واحدة على الأمثلة المرجعية في [التحقق البصري](visual-validation.md)
- مراجعة صفحة [الجاهزية لإصدار `alpha2`](alpha2-release-readiness.md)

## سياسة لقطات الشاشة

- تبقى لقطات الشاشة مواد يدوية مرتبطة بتجهيز الإصدار
- يمكن لمعرض الأمثلة أن يربط بمراجع بصرية ثابتة
- لا يعد توليد الصور شرط تحقق آلي في خط alpha الحالي
