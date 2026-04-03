# UNode وقياسات الصندوق

`UNode` هو اللبنة الأساسية لأي عنصر واجهة.

## الحقول الأساسية

- `width: UVal`
- `height: UVal`
- `min_width: f32`
- `max_width: f32`
- `min_height: f32`
- `max_height: f32`
- `padding: USides`
- `margin: USides`
- `background_color: Color`
- `border_radius: UCornerRadius`
- `shape_mode: UShapeMode`

## UVal

الوحدات المدعومة:

- `Px(f32)`
- `Percent(f32)`
- `MinContent`
- `MaxContent`
- `Content`
- `Auto`
- `Flex(f32)`

## ملاحظات الأحجام

- `width` و`height` يمثلان طلب الحجم المفضل.
- `min_width` و`max_width` و`min_height` و`max_height` تطبق `clamp` على الناتج النهائي.
- `padding` يدخل في القياس الجوهري، بينما `margin` يؤثر على التموضع ومساحة الالتفاف ولا يغير المحتوى الداخلي المقاس للعقدة.
- `UVal::Content` يبقى الاسم القديم الموافق لـ `UVal::MaxContent`.
- `UVal::Auto` صار نمطًا سياقيًا وليس مرادفًا مباشرًا لـ `Content`.
- استخدم `MaxContent` أو `MinContent` عندما تريد حجمًا جوهريًا صريحًا من دون سلوك التمدد الضمني الخاص بـ `Auto`.
- `border_radius` و`shape_mode` يظلان خصائص بصرية فقط، ولا يغيران قياس التخطيط بحد ذاتهما.

راجع [دلالات الأحجام](sizing-semantics.md) للتفاصيل الحالية الكاملة.

## ComputedSize

بعد الحل النهائي يحصل كل عنصر على:

- `width`
- `height`
- `local_pos`

وهذا هو القياس الذي يستخدمه الرندر.

## UBorder

حدود مرئية مستقلة عن خلفية `UNode`:

- `color`
- `width`
- `radius`
- `offset`

## Shape Modes

- `Round`: زوايا مستديرة SDF.
- `Cut`: قص زوايا بنمط beveled/cut.
