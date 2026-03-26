# UNode وقياسات الصندوق

`UNode` هو اللبنة الأساسية لأي عنصر واجهة.

## الحقول الأساسية

- `width: UVal`
- `height: UVal`
- `padding: USides`
- `margin: USides`
- `background_color: Color`
- `border_radius: UCornerRadius`
- `shape_mode: UShapeMode`

## UVal

الوحدات المدعومة:

- `Px(f32)`
- `Percent(f32)`
- `Content`
- `Auto`
- `Flex(f32)`

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
