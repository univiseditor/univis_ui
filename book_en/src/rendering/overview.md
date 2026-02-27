# الرندر Rendering

الرندر في Univis يعتمد على مواد SDF مخصصة:

- `UNodeMaterial` لمسار 2D.
- `UNodeMaterial3d` لمسار 3D.

الملفات الأساسية:

- `src/layout/render/system.rs`
- `src/layout/render/material.rs`
- `src/layout/render/material_3d.rs`
- `src/layout/render/shaders/unode.wgsl`
- `src/layout/render/shaders/unode_3d.wgsl`

## Update path

`update_materials_optimized`:

- يقرأ `UNode`, `ComputedSize`, `UBorder`, `UImage`, `UI3d`, `UPbr`.
- يقرر 2D أو 3D حسب وجود `UI3d`.
- يعيد استخدام handles في `MaterialHandles` لتقليل التخصيص.
- يمرر بيانات القص clip إلى مادة 2D.

## لماذا SDF؟

- حواف نظيفة تحت التكبير.
- دعم border radius وقص وتنعيم جيد.
- مرونة لشكل `Round` و`Cut`.
