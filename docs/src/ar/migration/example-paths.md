# ترحيل مسارات الأمثلة

تسجل هذه الصفحة مواقع الأمثلة الحالية.

## المسارات الحالية

- عرض حزمة Android: `android/android_phone_app`
- أمثلة جذر مساحة العمل: `examples/*.rs`
- أمثلة grid: `examples/grid/*.rs`
- أمثلة layout: `examples/layout/*.rs`
- أمثلة widgets: `examples/widgets/*.rs`

## الأوامر الحالية

شغّل التطبيق ذي الطابع Android:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

افحص كل أمثلة مساحة العمل المسجلة في Cargo:

```bash
cargo check --workspace --examples
```

افحص حزمة Android المستقلة:

```bash
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
```

## صفحات مرتبطة

- [فهرس الأمثلة](../examples/index.md)
- [معرض الأمثلة](../examples/gallery.md)
- [الاختبارات والتحقق](../development/testing.md)
