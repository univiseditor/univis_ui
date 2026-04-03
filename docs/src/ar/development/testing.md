# الاختبارات والتحقق

## اختبارات وحدة (فردي)

لتخفيف الحمل، شغّل الاختبارات كل واحدة على حدة:

```bash
cargo test --release <test_name> --lib
```

## تحقق الجودة

```bash
./scripts/check_quality.sh
./scripts/check_representative_examples.sh
./scripts/check_examples_serial_release.sh
```

يشغّل `./scripts/check_quality.sh` ما يلي:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets` مع allowlist الحالية في `CI` للديون المعروفة في lints الخاصة بكود Bevy الثقيل
- `cargo check --workspace --all-targets`
- `cargo test --workspace --lib`

ويفحص `./scripts/check_representative_examples.sh` مجموعة `release` مختارة تغطي مسار الواجهة
المجمعة، وأنماط الجذور، والتفاعل، وتغيير حجم اللوحات، والإدخال النصي، ومسار `World3d`.

## تحقق التوثيق

```bash
cargo doc --no-deps
mdbook build docs
```

## التحقق داخل `CI`

أصبح GitHub Actions يتحقق من الجودة والتوثيق والأمثلة و`API Docs` عبر:

- `.github/workflows/docs_examples_api.yml`
- `.github/workflows/docs_publish.yml`

ويشغّل:

- `./scripts/check_quality.sh`
- `./scripts/check_representative_examples.sh`
- `mdbook build docs`
- `cargo doc --no-deps` لكل crate عامة
- فحص الأمثلة لكل package على حدة عبر `./scripts/check_examples_serial_release.sh -p ...`
- مسار نشر واحد مستقل لصفحات الوثائق العامة على `main`

## للأجهزة الضعيفة (تشغيل تسلسلي)

```bash
# كل اختبارات lib واحدة واحدة
./scripts/test_lib_serial_release.sh

# كل الأمثلة واحدة واحدة
./scripts/check_examples_serial_release.sh

# مرور compile سريع على أمثلة ممثلة لأسطح المشروع
./scripts/check_representative_examples.sh

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
2. شغّل `./scripts/check_quality.sh`.
3. شغّل `./scripts/check_representative_examples.sh`.
4. شغّل `./scripts/check_examples_serial_release.sh` للحزمة المتأثرة أو قبل الإصدار.
5. جرّب مثالًا واحدًا على الأقل مرتبطًا بالتعديل.
6. استخدم [التحقق البصري](visual-validation.md) عندما يكون التعديل غنيًا بالرندر أو التخطيط أو التفاعل.

## المطلوب قبل إصدار alpha القادم

- `./scripts/check_quality.sh`
- `mdbook build docs`
- `cargo doc -p univis_ui_style --no-deps`
- `cargo doc -p univis_ui_engine --no-deps`
- `cargo doc -p univis_ui_interaction --no-deps`
- `cargo doc -p univis_ui_widgets --no-deps`
- `cargo doc -p univis_ui --no-deps`
- `./scripts/check_representative_examples.sh`
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
