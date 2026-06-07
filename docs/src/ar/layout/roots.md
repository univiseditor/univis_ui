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

## الشاشة

الاستخدام:

```rust
URootUi::screen()
```

الدلالة:

- يُحسم انطلاقًا من مساحة رؤية الكاميرا المستهدفة، لا من `Window.width()/height()` مباشرة.
- يتصرف كـ الواجهة الثابتة حقيقي.
- حركة الكاميرا والـ zoom والدوران لا يجب أن تحرك واجهة الشاشة بصريًا.
- يشكّل كبسولة تراكب مغلقة: الترتيب المحلي لا يخرج فوق جذر آخر.
- مناسب للقوائم، والعناصر العائمة، وعناصر الواجهة الثابتة الثابتة على الشاشة.

## العالم ثنائي الأبعاد

الاستخدام:

```rust
URootUi::world_2d(Vec2::new(1280.0, 720.0))
```

أو:

```rust
URootUi::world_2d_fit_content()
```

الدلالة:

- يدعم مساحة رسم منطقية ثابتة أو مساحة رسم تُقاس من المحتوى.
- يعيش داخل العالم.
- يُرسم عبر مسار المواد ثنائي الأبعاد.
- يشكّل كبسولة تراكب مغلقة: الأبناء يبقون داخل الطبقة البصرية للجذر.
- مناسب للوحات والشاشات الداخلية والعناصر المربوطة بالمشهد.

## العالم ثلاثي الأبعاد

الاستخدام:

```rust
URootUi::world_3d(Vec2::new(1280.0, 720.0))
```

أو:

```rust
URootUi::world_3d_fit_content()
```

الدلالة:

- يستخدم نفس نموذج مساحة الرسم المنطقية الموجود في `World2d`، بما فيه القياس من المحتوى.
- يعيش داخل العالم.
- يُرسم عبر المسار ثلاثي الأبعاد.
- يشكّل كبسولة تراكب مغلقة بالنسبة إلى جذور UI الأخرى.
- هنا تصبح إعدادات `UPbr` ذات معنى.

## مساحة الرسم المنطقية ووحدات القياس

يبقى `UVal` نوعًا خاصًا بوحدات التخطيط داخل شجرة الواجهة.

- `UVal::Px(f32)` يعني وحدات UI منطقية.
- `UiCanvasSize::Viewport` تعني اتباع مساحة رؤية الكاميرا المحلولة.
- `UiCanvasSize::Fixed(Vec2)` تعني مساحة رسم منطقية ثابتة الحجم.
- `UiCanvasSize::FitContent { min, max }` تعني قياس مساحة الرسم المنطقية من المحتوى ثم تطبيق حدود `min/max` عند الحاجة.

في جذور العالم، الحجم الفيزيائي يُشتق صراحةً:

```text
world_size = canvas_size * meters_per_unit
```

وهذا يعني:

- التخطيط يبقى بوحدات UI منطقية
- `meters_per_unit` يتحكم فقط في الحجم الفيزيائي داخل العالم
- `resolution_scale` يتحكم في الجودة البصرية بشكل مستقل عن حجم العالم
- الجذور المعتمدة على المحتوى تعمل أفضل عندما يكون المحتوى الداخلي intrinsic أو fixed؛ الاعتماد الكبير على `%` أو flex المرتبط بحجم الجذر قد يخلق دورة توقعات

## حسم الكاميرا

`UiCameraRef::Auto` مناسب للمشاهد البسيطة التي تحتوي كاميرا متوافقة واحدة فقط.

أما في المشاهد متعددة الكاميرات فالأفضل استخدام:

```rust
UiCameraRef::Entity(camera_entity)
```

حتى يبقى حسم مساحة الرؤية والتفاعل وتثبيت جذور الشاشة واضحًا وغير ملتبس.

## ملاحظة التوافق مع السلوك القديم

`UScreenRoot` و`UWorldRoot` موجودان فقط كطبقات توافق مهجورة.

وهما الآن مساران صريحان للترحيل فقط، وهدف الإزالة الحالي هو أول alpha بعد
`0.3.0` لا يعود يحتاج دعم الترحيل المبني على هذه الطبقات.

قواعد الترحيل:

- `UScreenRoot` -> `URootUi::screen()`
- `UWorldRoot { size, is_3d: false }` -> `URootUi::world_2d(size)`
- `UWorldRoot { size, is_3d: true }` -> `URootUi::world_3d(size)`

إذا احتجت حجمًا فيزيائيًا محددًا داخل العالم، فاستخدم:

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

## أمثلة مرتبطة

- [`responsive_layout_test`](../examples/index.md#أمثلة-مساحة-العمل)
- [`z_order_hierarchy`](../examples/index.md#أمثلة-مساحة-العمل)
- [`layout_stack`](../examples/index.md#أنماط-التخطيط)

## صفحات ترحيل مرتبطة

- [ترحيل الجذور إلى `URootUi`](../migration/root-api.md)
- [حالة التوافق القديم](../migration/legacy-compatibility.md)

## نقاط الدخول الرسمية في `API`

- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::layout_system::UiSpace`
- `univis_ui_engine::layout::layout_system::UiCanvasSize`
- `univis_ui_engine::layout::layout_system::UiCameraRef`
- `univis_ui_engine::layout::pbr::UPbr`

## إلى أين بعد ذلك؟

- المثال المرتبط: [`responsive_layout_test`](../examples/index.md#أمثلة-مساحة-العمل)
- فهرس `API`: [مرجع الواجهة العامة](../api/index.md)
- صفحة الترحيل: [ترحيل الجذور إلى `URootUi`](../migration/root-api.md)
