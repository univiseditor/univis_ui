# `UClip` و`UPbr`

## UClip

`UClip { enabled: bool }` يفرض قصًا على الأبناء.

الاستخدام الشائع:

```rust,no_run
commands.spawn((
    UNode { ..default() },
    UClip { enabled: true },
));
```

### أين يُستخدم القص؟

- في `interaction/picking`: لمنع hit-test خارج منطقة القص.
- في `render/system`: تمرير بيانات القص إلى مادة `UNodeMaterial`.
- في النص: يوجد نظام `sync_text_clip_visibility` لإخفاء النص الذي يخرج خارج الأسلاف القاصين.

## UPbr

عند العمل في 3D UI (`UI3d`):

- `UPbr` يسمح بتخصيص:
  - `metallic`
  - `roughness`
  - `emissive`

هذا يؤثر على مسار المادة ثلاثية الأبعاد (`UNodeMaterial3d`).
