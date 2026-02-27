# الاختبارات والتحقق

## اختبارات وحدة (فردي)

لتخفيف الحمل، شغّل الاختبارات كل واحدة على حدة:

```bash
cargo test --release <test_name> --lib
```

## تحقق البناء

```bash
cargo check --release
cargo check --release --examples
```

## تحقق التوثيق

```bash
cargo doc --no-deps
mdbook build book
mdbook build book_en
```

## استراتيجية عملية قبل الدمج

1. شغل اختبارات الوحدة الخاصة بالتعديل.
2. شغل `cargo check --release`.
3. شغل `cargo check --release --examples`.
4. جرّب مثال واحد على الأقل مرتبط بالتعديل.
