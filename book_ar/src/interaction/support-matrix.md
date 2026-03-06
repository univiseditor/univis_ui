# مصفوفة دعم التفاعل (Screen / World / 3D)

المعاني:
- `Supported`: مدعوم في المسار الموثق.
- `Partial`: مدعوم جزئيًا مع قيود.
- `N/A`: غير مقصود لهذا النمط.

| القدرة | Screen UI (`UScreenRoot`) | World UI (`UWorldRoot`, `is_3d = false`) | 3D UI (`UWorldRoot`, `is_3d = true`) | الشروط / الملاحظات |
|---|---|---|---|---|
| الرندر الأساسي | Supported | Supported | Supported | مسار 3D يعتمد على نشر `UI3d` واستخدام `UNodeMaterial3d`. |
| الالتقاط + أحداث المؤشر | Supported | Supported | Partial | الـ picking backend الحالي يستعلم `Camera2d`؛ مشاهد 3D التي تعتمد فقط على `Camera3d` ليست ضمن هذا المسار. |
| hit testing مع القص (clipping) | Supported | Supported | Partial | فحص قص الأسلاف يعمل داخل picking backend، مع نفس قيد الكاميرا. |
| مقابض تغيير حجم `UPanelWindow` | Supported | Supported | Partial | مسار تغيير الحجم يستعلم `Camera2d` حاليًا. |
| إدخال/أحداث `UTextField` | Supported | Supported | Partial | يتطلب `UnivisTextFieldPlugin`، والتفاعل ما يزال يتبع مسار `Camera2d`. |
| تفاعل `UScrollContainer` | Supported | Supported | Partial | Plugin التمرير مضاف تلقائيًا، لكن التفاعل يتبع مسار `Camera2d`. |
| خصائص `UPbr` (`metallic`, `roughness`, `emissive`) | N/A | N/A | Supported | مخصصة لمسار الرندر ثلاثي الأبعاد `UI3d`. |

## مصادر التحقق

- `src/interaction/picking.rs`
- `src/widget/panel.rs`
- `src/layout/layout_system.rs`
- `src/layout/render/mod.rs`
