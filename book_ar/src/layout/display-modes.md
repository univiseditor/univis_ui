# أنماط العرض Display Modes

`UDisplay` يدعم الأنماط التالية:

- `Flex`
- `Grid`
- `Masonry`
- `Stack`
- `Radial`
- `None`

## Flex

- يعتمد `flex_direction` و`justify_content` و`align_items`.
- يدعم `wrap` عبر `container_ext.flex.wrap`.
- يدعم `flex_grow/shrink/basis` على مستوى item.

## Grid

- يمكن التشغيل بنمط بسيط عبر `grid_columns`.
- أو بنمط متقدم عبر `template_rows/template_columns`.
- auto placement عبر `auto_flow` + `auto_rows/auto_columns`.

## Masonry

- توزيع عنصر في أقصر عمود (Pinterest-like).
- يتأثر بعدد الأعمدة والفواصل.

## Stack

- تراكب العناصر (overlay-like).

## Radial

- توزيع العناصر حول دائرة، مناسب لقوائم sci-fi.

## None

- تعطيل وضع التخطيط لتلك الحاوية.
