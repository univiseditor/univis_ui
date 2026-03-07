# الاختبارات والتحقق

## اختبارات وحدة (فردي)

لتخفيف الحمل، شغّل الاختبارات كل واحدة على حدة:

```bash
cargo test --release <test_name> --lib
```

## تحقق البناء

```bash
cargo check --workspace --all-targets
cargo check --release --examples
```

## تحقق التوثيق

```bash
cargo doc --no-deps
mdbook build book_ar
mdbook build book_en
```

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
./scripts/check_examples_serial_release.sh -p univis_ui
./scripts/verify_serial_release.sh -p univis_ui

# تحقق alpha قبل النشر: check + lib tests + examples + package
./scripts/verify_alpha_release.sh

# إنشاء حزم alpha فقط بدون verify
./scripts/package_alpha_serial.sh --no-verify
```

لتمرير أمثلة محددة فقط:

```bash
./scripts/check_examples_serial_release.sh hello_world interaction select
```

## استراتيجية عملية قبل الدمج

1. شغل اختبارات الوحدة الخاصة بالتعديل.
2. شغل `cargo check --workspace --all-targets`.
3. شغل `cargo check --release --examples`.
4. جرّب مثال واحد على الأقل مرتبط بالتعديل.
