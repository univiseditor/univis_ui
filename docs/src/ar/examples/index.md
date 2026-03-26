# الأمثلة

الأمثلة أصبحت موزعة بجانب الـ crate التي تمثلها أكثر. شغّلها بصيغة:

```bash
cargo run -p <package> --example <name>
```

## أمثلة `univis_ui`

- `hello_world`: البداية السريعة.
- `complex_dashboard`: لوحة متكاملة تعتمد على الواجهة المجمعة.
- `card_profile`: بطاقة تعريف polished على سطح facade.
- `sci_fi`: HUD مركب يستعرض السطح الكامل.

## أمثلة `univis_ui_engine`

- `alignment`
- `border_light_3d`
- `ex_node`
- `layout_cache`
- `layout_case_alignment_overflow`
- `layout_case_flex_wrap`
- `layout_case_grid_auto_flow`
- `layout_case_grid_tracks`
- `layout_case_masonry_ext`
- `layout_case_radial`
- `layout_case_stack`
- `layout_solver_no_widgets`
- `layout_solver_ultra_complex`
- `masonry`
- `root_screen_hud`
- `root_world_scale`
- `root_capsule_overlap`
- `root_fit_content`
- `texture`

## أمثلة `univis_ui_widgets`

- `toggle`
- `radio`
- `seekbar`
- `drag_value`
- `select`
- `text_field`
- `scroll_view`
- `panel_divider`
- `panel_window`
- `text_label`
- `text_label_zoom`
- `widgets`

## أمثلة `univis_ui_interaction`

- `interaction`

## أوامر مقترحة

```bash
cargo run -p univis_ui --example hello_world
cargo run -p univis_ui_engine --example root_fit_content
cargo run -p univis_ui_widgets --example text_field
cargo run -p univis_ui_interaction --example interaction
```

## توصية عملية للتعلم

1. ابدأ بـ `hello_world`.
2. انتقل إلى `text_label` و`widgets` و`interaction`.
3. ثم راجع `layout_case_*` وأمثلة الجذور لفهم سلوك المحرك.
4. أخيرًا افحص `sci_fi` كحالة مركبة كبيرة.

## تقارير مرتبطة

- [تقرير التحقق من الأمثلة](../development/example-validation.md)
- [مصفوفة التوافق](../development/compatibility-matrix.md)
- [خطة Smoke Tests](../development/smoke-test-plan.md)
