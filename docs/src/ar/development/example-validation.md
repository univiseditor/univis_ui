# تقرير التحقق من الأمثلة

## نطاق التحقق الحالي

يتحقق هذا الفرع فقط من ملفات الأمثلة الموجودة فعليًا في المستودع.

## الأوامر

افحص أمثلة مساحة العمل:

```bash
cargo check --workspace --examples
```

افحص حزمة Android المستقلة:

```bash
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
```

شغّل مسار التحقق التمثيلي في وضع release:

```bash
./scripts/check_representative_examples.sh
```

## مصادر الأمثلة الحالية

- `android/android_phone_app/examples/android_phone.rs`
- `examples/responsive_layout_test.rs`
- `examples/toggle_seekbar.rs`
- `examples/z_order_hierarchy.rs`
- `examples/grid/auto_flow.rs`
- `examples/grid/columns.rs`
- `examples/grid/item_placement.rs`
- `examples/grid/tracks.rs`
- `examples/layout/flex.rs`
- `examples/layout/masonry.rs`
- `examples/layout/radial.rs`
- `examples/layout/stack.rs`
- `examples/widgets/containers.rs`
- `examples/widgets/controls.rs`
- `examples/widgets/display.rs`
- `examples/widgets/inputs.rs`

ما يزال سلوك Runtime يحتاج فحوص smoke يدوية بنافذة فعلية للتفاعل والدقة البصرية.
