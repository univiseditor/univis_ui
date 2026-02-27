# الجذور والمساحات

## UScreenRoot

- Marker لواجهة الشاشة (HUD).
- عادةً الحجم يُشتق من أبعاد نافذة العرض.
- مناسب لعناصر UI التي تتبع الشاشة مباشرة.

## UWorldRoot

- Marker لواجهة world-space.
- الحقول:
  - `size: Vec2`
  - `is_3d: bool`
  - `resolution_scale: f32`
- عندما `is_3d = true` يتم نشر `UI3d` عبر `auto_propagate_ui3d`.

## الجذر في pass down

في `pass_down`:

- إذا الكيان `UWorldRoot` => يستخدم `size` مباشرة.
- غير ذلك => يستخدم `Window.width/height` كقياس root (عند Screen root).

## متى تستخدم أي واحد؟

- تريد HUD أو menu ثابت: `UScreenRoot`.
- تريد UI داخل العالم (لوحة على جسم/مشهد): `UWorldRoot`.
