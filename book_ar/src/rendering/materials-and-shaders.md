# المواد والشيدر

## UNodeMaterial (2D)

أهم الحقول:

- `color`
- `border_color`
- `radius`
- `size`
- `border_width`
- `border_offset`
- `softness`
- `shape_mode`
- `use_texture`
- `texture`
- `clip_center`
- `clip_size`
- `clip_radius`
- `use_clip`

## UNodeMaterial3d

مسار 3D يضيف خصائص PBR:

- `metallic`
- `roughness`
- `emissive`

## الشيدر 2D

في `unode.wgsl`:

- يحسب SDF للشكل الأساسي.
- يحسب mask للحدود والجسم الداخلي.
- إن كان `use_clip = 1` يطبق clip mask قبل إخراج اللون.

## الشيدر 3D

في `unode_3d.wgsl`:

- نفس فكرة SDF للشكل.
- يدمج مع pipeline ثلاثي الأبعاد.
