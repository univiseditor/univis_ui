# أنماط العرض

`UDisplay` يدعم الأنماط التالية:

- `Flex`
- `Grid`
- `Masonry`
- `Stack`
- `Radial`
- `None`

## التدفق المرن

- يعتمد `flex_direction` و`justify_content` و`align_items`.
- يدعم `wrap` عبر `container_ext.flex.wrap`.
- يدعم `flex_grow/shrink/basis` على مستوى العنصر.

## الشبكة

- يمكن التشغيل بنمط بسيط عبر `grid_columns`.
- أو بنمط متقدم عبر `template_rows/template_columns`.
- التموضع التلقائي عبر `auto_flow` + `auto_rows/auto_columns`.

## البناء الحجري

- توزيع عنصر في أقصر عمود (شبيه بلوحات التثبيت).
- يتأثر بعدد الأعمدة والفواصل.

## التكديس

- تراكب العناصر (شبيه بالطبقات).

## الشعاعي

- توزيع العناصر حول دائرة، مناسب لقوائم خيال علمي.

## تعطيل التخطيط

- تعطيل وضع التخطيط لتلك الحاوية.
