# ترحيل الجذور إلى `URootUi`

أصبح `URootUi` هو الواجهة العامة الرسمية للجذور.

## التحويل من القديم إلى الجديد

- `UScreenRoot` -> `URootUi::screen()`
- `UWorldRoot { size, is_3d: false }` -> `URootUi::world_2d(size)`
- `UWorldRoot { size, is_3d: true }` -> `URootUi::world_3d(size)`

## أهم التغييرات الدلالية

- `URootUi::screen()` أصبح جذر واجهة ثابتة على الشاشة وحقيقيًا وثابتًا على منفذ العرض.
- كل `URootUi` يعمل ككبسولة تراكب مغلقة.
- `UVal::Px` يعني وحدات UI منطقية، لا pixels شاشة فعلية.
- الحجم الفيزيائي في فضاء العالم يُحسم عبر `meters_per_unit`.

## ملاحظات التوافق المرحلي

- يبقى `UScreenRoot` و`UWorldRoot` طبقتي توافق مهجورتين.
- إذا أردت نفس الحجم التاريخي لجذور العالم من الأمثلة القديمة، فاضبط `meters_per_unit: 1.0` صراحة.

## صفحات مرتبطة

- [الجذور والمساحات](../layout/roots.md)
- [البدء السريع](../quick-start.md)
- [فهرس الأمثلة](../examples/index.md#مسار-التعلم-المقترح)
