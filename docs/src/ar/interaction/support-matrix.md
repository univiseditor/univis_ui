# مصفوفة دعم التفاعل (Screen / World2d / World3d)

المعاني:

- `Supported`: مدعوم في المسار الموثق.
- `جزئي`: مدعوم جزئيًا مع قيود.
- `N/A`: غير مقصود لهذا النمط.

| القدرة | Screen UI (`URootUi::screen()`) | World UI (`URootUi::world_2d(...)`) | 3D UI (`URootUi::world_3d(...)`) | الشروط / الملاحظات |
|---|---|---|---|---|
| الرندر الأساسي | Supported | Supported | Supported | `Screen` و`World2d` يستخدمان مسار 2D، بينما `World3d` يستخدم مسار المواد ثلاثي الأبعاد. |
| الالتقاط + أحداث المؤشر | Supported | Supported | Supported | التفاعل يحسم الكاميرا من كل root عبر `ResolvedRootUi`. وفي المشاهد متعددة الكاميرات يُفضّل استخدام `UiCameraRef::Entity`. |
| hit testing مع القص | Supported | Supported | Supported | فحص قص الأسلاف يعمل في كل الفضاءات. وجذور العالم ما تزال تُفحص على مستوى plane الواجهة نفسها. |
| مقابض تغيير حجم `UPanelWindow` | Supported | Supported | Supported | منطق تغيير الحجم يحسب حركة المؤشر عبر كاميرا الجذر ومستوى اللوحة. |
| إدخال/أحداث `UTextField` | Supported | Supported | Supported | مضمّن افتراضيًا عبر `UnivisWidgetPlugin`؛ أضف `UnivisTextFieldPlugin` مباشرة فقط عند تركيب widgets يدويًا بشكل أضيق. |
| تفاعل `UScrollContainer` | Supported | Supported | Supported | التمرير يتبع مسار الكاميرا المحلولة من الجذر. |
| خصائص `UPbr` (`metallic`, `roughness`, `emissive`) | N/A | N/A | Supported | مخصصة فقط لمسار `World3d`. |

## ملاحظات

- `URootUi::screen()` هو مسار HUD حقيقي مربوط بالـ viewport المحلول.
- `URootUi::world_2d(...)` و`URootUi::world_3d(...)` يعتمدان canvas منطقيًا ثابتًا مع world scaling صريح.
- `meters_per_unit` يغيّر الحجم الفيزيائي في العالم من دون تغيير حجم التخطيط المنطقي.

## مصادر التحقق

- `crates/univis_ui_interaction/src/interaction/picking.rs`
- `crates/univis_ui_widgets/src/widget/panel.rs`
- `crates/univis_ui_engine/src/layout/layout_system.rs`
- `crates/univis_ui_engine/src/layout/render/system.rs`
