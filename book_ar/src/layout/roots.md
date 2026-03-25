# الجذور والمساحات

`URootUi` هو المدخل العام الوحيد للجذر في الواجهة.
ويحسم كل شجرة UI إلى واحد من ثلاثة فضاءات:

- `UiSpace::Screen`
- `UiSpace::World2d`
- `UiSpace::World3d`

## شكل الجذر

```rust
#[derive(Component, Clone, Reflect)]
pub struct URootUi {
    pub space: UiSpace,
    pub canvas: UiCanvasSize,
    pub camera: UiCameraRef,
    pub meters_per_unit: f32,
    pub resolution_scale: f32,
}
```

## Screen

الاستخدام:

```rust
URootUi::screen()
```

الدلالة:

- يُحسم انطلاقًا من viewport الكاميرا المستهدفة، لا من `Window.width()/height()` مباشرة.
- يتصرف كـ HUD حقيقي.
- حركة الكاميرا والـ zoom والدوران لا يجب أن تحرك واجهة الشاشة بصريًا.
- مناسب للقوائم، والـ overlays، وعناصر HUD الثابتة على الشاشة.

## World2d

الاستخدام:

```rust
URootUi::world_2d(Vec2::new(1280.0, 720.0))
```

الدلالة:

- يستخدم canvas منطقيًا ثابتًا في التخطيط.
- يعيش داخل العالم.
- يُرسم عبر مسار المواد ثنائي الأبعاد.
- مناسب للوحات والشاشات الداخلية والعناصر المربوطة بالمشهد.

## World3d

الاستخدام:

```rust
URootUi::world_3d(Vec2::new(1280.0, 720.0))
```

الدلالة:

- يستخدم نفس نموذج canvas المنطقي الثابت الموجود في `World2d`.
- يعيش داخل العالم.
- يُرسم عبر المسار ثلاثي الأبعاد.
- هنا تصبح إعدادات `UPbr` ذات معنى.

## الـ Canvas المنطقي ووحدات القياس

يبقى `UVal` نوعًا خاصًا بوحدات التخطيط داخل شجرة الواجهة.

- `UVal::Px(f32)` يعني وحدات UI منطقية.
- `UiCanvasSize::Viewport` تعني اتباع viewport الكاميرا المحلولة.
- `UiCanvasSize::Fixed(Vec2)` تعني canvas منطقيًا ثابت الحجم.

في جذور العالم، الحجم الفيزيائي يُشتق صراحةً:

```text
world_size = canvas_size * meters_per_unit
```

وهذا يعني:

- التخطيط يبقى بوحدات UI منطقية
- `meters_per_unit` يتحكم فقط في الحجم الفيزيائي داخل العالم
- `resolution_scale` يتحكم في الجودة البصرية بشكل مستقل عن حجم العالم

## حسم الكاميرا

`UiCameraRef::Auto` مناسب للمشاهد البسيطة التي تحتوي كاميرا متوافقة واحدة فقط.

أما في المشاهد متعددة الكاميرات فالأفضل استخدام:

```rust
UiCameraRef::Entity(camera_entity)
```

حتى يبقى حسم الـ viewport والتفاعل وتثبيت جذور الشاشة واضحًا وغير ملتبس.

## ملاحظة ترحيل لـ alpha2

`UScreenRoot` و`UWorldRoot` موجودان فقط كطبقات توافق deprecated خلال `alpha2`.

قواعد الترحيل:

- `UScreenRoot` -> `URootUi::screen()`
- `UWorldRoot { size, is_3d: false }` -> `URootUi::world_2d(size)`
- `UWorldRoot { size, is_3d: true }` -> `URootUi::world_3d(size)`

إذا احتجت الحفاظ على نفس الحجم الفيزيائي التاريخي لأمثلة world-space القديمة خلال `alpha2`,
فاستخدم:

```rust
URootUi {
    meters_per_unit: 1.0,
    ..URootUi::world_2d(size)
}
```

أو:

```rust
URootUi {
    meters_per_unit: 1.0,
    ..URootUi::world_3d(size)
}
```
