# جدول حقيقة الإضافات

هذه الصفحة هي مرجع الحقيقة الرسمي لحالة تسجيل الإضافات في المستودع الحالي.

## تركيب الجذر (`UnivisUiPlugin`)

| الإضافة | تُضاف عبر `UnivisUiPlugin` | ملاحظات |
|---|---|---|
| `UnivisUiStylePlugin` | نعم | الخطوط/الأيقونات المضمنة + `Theme`. |
| `UnivisEnginePlugin` | نعم | يضيف `UnivisNodePlugin` و`UnivisLayoutPlugin` و`UnivisRenderPlugin`. |
| `UnivisInteractionPlugin` | نعم | يضيف خلفية الالتقاط ومراقبي المؤشر. |
| `UnivisWidgetPlugin` | نعم | يسجّل مجموعة الإضافات الأساسية للوحدات الجاهزة. |
| `UnivisLayoutProfilingPlugin` | لا | إضافة اختيارية للتشخيص وتُضاف يدويًا. |

## تركيب الوحدات الجاهزة (`UnivisWidgetPlugin`)

| إضافة الوحدة الجاهزة | تُسجّل تلقائيًا عبر `UnivisUiPlugin` | ملاحظات |
|---|---|---|
| `UnivisTextPlugin` | نعم | `UTextLabel` وأنظمة text clipping. |
| `UnivisProgressPlugin` | نعم | `UProgressBar`. |
| `UnivisButtonPlugin` | نعم | `UButton`. |
| `UnivisRadioPlugin` | نعم | `URadioButton`, `URadioGroup`. |
| `UnivisIconButtonPlugin` | نعم | `UIconButton`. |
| `UnivisTogglePlugin` | نعم | `UToggle`. |
| `UnivisCheckboxPlugin` | نعم | `UCheckbox`. |
| `UnivisSeekBarPlugin` | نعم | `USeekBar`. |
| `UnivisScrollViewPlugin` | نعم | `UScrollContainer`. |
| `UnivisDividerPlugin` | نعم | `UDivider`. |
| `UnivisPanelPlugin` | نعم | `UPanel` وسلوك `UPanelWindow`. |
| `UnivisBadgePlugin` | نعم | `UBadge` مع تحديثات المرئيات الديناميكية للـ badge/tag. |
| `UnivisDragValuePlugin` | نعم | `UDragValue`. |
| `UnivisSelectPlugin` | نعم | `USelect`. |
| `UnivisTextFieldPlugin` | نعم | سلوك/أحداث `UTextField`. |

وتبقى الإضافات المخصصة نفسها متاحة عندما تريد بناء سطح widgets أضيق من `UnivisWidgetPlugin`.

## مصادر التحقق

- `src/lib.rs`
- `crates/univis_ui_engine/src/lib.rs`
- `crates/univis_ui_widgets/src/widget/mod.rs`
