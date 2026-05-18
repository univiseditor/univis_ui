# معرض الأمثلة

تعرض هذه الصفحة الأمثلة الموجودة في المستودع الحالي فقط.

## أفضل مسارات البدء

1. [البدء السريع](../quick-start.md)
2. [`android_phone`](android-phone.md)
3. `widgets_controls`
4. `widgets_inputs`
5. `responsive_layout_test`

## واجهات الشاشة

| المثال | استخدمه من أجل | الأمر |
| --- | --- | --- |
| `android_phone` | شاشة كاملة بطابع تطبيق هاتف. | `cargo run --manifest-path android/android_phone_app/Cargo.toml` |
| `responsive_layout_test` | تركيب واجهة شاشة متجاوبة. | `cargo run --example responsive_layout_test` |
| `z_order_hierarchy` | فحص الترتيب البصري والتراكب. | `cargo run --example z_order_hierarchy` |

## التخطيط

| المثال | استخدمه من أجل | الأمر |
| --- | --- | --- |
| `layout_flex` | تخطيط flex. | `cargo run --example layout_flex` |
| `layout_masonry` | تخطيط masonry. | `cargo run --example layout_masonry` |
| `layout_stack` | تخطيط stack. | `cargo run --example layout_stack` |
| `layout_radial` | تخطيط radial. | `cargo run --example layout_radial` |
| `grid_columns` | أعمدة grid. | `cargo run --example grid_columns` |
| `grid_tracks` | مسارات grid. | `cargo run --example grid_tracks` |
| `grid_auto_flow` | التموضع التلقائي في grid. | `cargo run --example grid_auto_flow` |
| `grid_item_placement` | التموضع الصريح لعناصر grid. | `cargo run --example grid_item_placement` |

## الوحدات الجاهزة

| المثال | استخدمه من أجل | الأمر |
| --- | --- | --- |
| `widgets_controls` | الأزرار والتبديلات وcheckbox وradio controls. | `cargo run --example widgets_controls` |
| `widgets_inputs` | Text fields وselect وdrag-value وseek-bar inputs. | `cargo run --example widgets_inputs` |
| `widgets_display` | النصوص وbadges والمقسمات واللوحات وprogress display. | `cargo run --example widgets_display` |
| `widgets_containers` | Panel windows وscroll containers. | `cargo run --example widgets_containers` |
| `toggle_seekbar` | مشهد صغير لـ toggle وseek-bar. | `cargo run --example toggle_seekbar` |

## صفحات مرتبطة

- [فهرس الأمثلة](index.md)
- [شاشة تطبيق Android](android-phone.md)
- [نظرة عامة على الوحدات الجاهزة](../widgets/overview.md)
