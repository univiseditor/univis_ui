# اللوحة ونافذة اللوحة

## UPanel

الملف: `src/widget/panel.rs`

خصائص أساسية:

- `background`
- `border_color`
- `border_width`
- `border_radius`
- `padding`
- `gap`
- `direction`

وظيفته: تقديم container جاهز بواجهة panel.

## `UPanelWindow` الاختيارية

`UPanelWindow` يضيف سلوك resize من الحدود والزوايا إلى `UPanel`.

الحقول:

- `border_hit_thickness`
- `min_width`
- `min_height`

builders:

- `with_min_size(width, height)`
- `with_border_hit_thickness(thickness)`

## السلوك الحالي

- 8 مناطق resize: `N,S,E,W,NE,NW,SE,SW`.
- resize فقط (بدون move).
- cursor icon يتغير عند hover/press على handles فقط.
- عند أول resize يتم تثبيت panel إلى `Absolute + Px` لضمان سلوك متوقع.

## مثال حي

- استخدم `cargo run --example widgets_containers` لرؤية `UPanelWindow` القابل لتغيير الحجم مع منفذ تمرير مرافق

## مثال حي

شغّل `cargo run --example widgets_containers` لرؤية `UPanelWindow` القابل لتغيير الحجم مع منفذ تمرير مرافق.
