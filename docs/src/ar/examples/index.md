# فهرس الأمثلة

تعرض هذه الصفحة فقط ملفات الأمثلة الموجودة فعليًا في المستودع الحالي.

## تشغيل العرض الحالي

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## حزمة Android المستقلة

| المثال | المصدر | الغرض | الأمر |
| --- | --- | --- | --- |
| `android_phone` | `android/android_phone_app/examples/android_phone.rs` | شاشة تطبيق بطابع Android مع بحث وتبديلات ومنزلقات ومحتوى قابل للتمرير وتنقل سفلي. | `cargo run --manifest-path android/android_phone_app/Cargo.toml --example android_phone` |

وتملك الحزمة نقطة دخول سطح مكتب مباشرة:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## أمثلة مساحة العمل

| المثال | المصدر | الغرض | الأمر |
| --- | --- | --- | --- |
| `responsive_layout_test` | `examples/responsive_layout_test.rs` | مشهد ضغط لتركيب واجهة شاشة متجاوبة. | `cargo run --example responsive_layout_test` |
| `toggle_seekbar` | `examples/toggle_seekbar.rs` | مشهد تحكم صغير لسلوك toggle وseek-bar. | `cargo run --example toggle_seekbar` |
| `z_order_hierarchy` | `examples/z_order_hierarchy.rs` | فحص الترتيب البصري وسلوك تراكب الهرمية. | `cargo run --example z_order_hierarchy` |

## تخطيط Grid

| المثال | المصدر | الغرض | الأمر |
| --- | --- | --- | --- |
| `grid_columns` | `examples/grid/columns.rs` | أحجام أعمدة grid وسلوك التخطيط. | `cargo run --example grid_columns` |
| `grid_tracks` | `examples/grid/tracks.rs` | سلوك أحجام مسارات grid. | `cargo run --example grid_tracks` |
| `grid_auto_flow` | `examples/grid/auto_flow.rs` | سلوك التموضع التلقائي داخل grid. | `cargo run --example grid_auto_flow` |
| `grid_item_placement` | `examples/grid/item_placement.rs` | التموضع الصريح لعناصر grid. | `cargo run --example grid_item_placement` |

## أنماط التخطيط

| المثال | المصدر | الغرض | الأمر |
| --- | --- | --- | --- |
| `layout_flex` | `examples/layout/flex.rs` | تركيب flex layout. | `cargo run --example layout_flex` |
| `layout_masonry` | `examples/layout/masonry.rs` | تركيب masonry layout. | `cargo run --example layout_masonry` |
| `layout_stack` | `examples/layout/stack.rs` | تركيب stack layout. | `cargo run --example layout_stack` |
| `layout_radial` | `examples/layout/radial.rs` | تركيب radial layout. | `cargo run --example layout_radial` |

## الوحدات الجاهزة

| المثال | المصدر | الغرض | الأمر |
| --- | --- | --- | --- |
| `widgets_controls` | `examples/widgets/controls.rs` | أزرار وتبديلات وcheckbox وradio controls. | `cargo run --example widgets_controls` |
| `widgets_inputs` | `examples/widgets/inputs.rs` | Text field وselect وdrag-value وseek-bar inputs. | `cargo run --example widgets_inputs` |
| `widgets_display` | `examples/widgets/display.rs` | نصوص وbadges ومقسمات ولوحات وprogress display. | `cargo run --example widgets_display` |
| `widgets_containers` | `examples/widgets/containers.rs` | سلوك panel window وscroll-container. | `cargo run --example widgets_containers` |

## التحقق

افحص كل أمثلة مساحة العمل التي يعرفها Cargo حاليًا:

```bash
cargo check --workspace --examples
```

وافحص حزمة Android:

```bash
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
```

## صفحات مرتبطة

- [معرض الأمثلة](gallery.md)
- [شاشة تطبيق Android](android-phone.md)
- [الاختبارات والتحقق](../development/testing.md)
