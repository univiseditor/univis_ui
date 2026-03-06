# جدول حقيقة الـ Plugins

هذه الصفحة هي مرجع الحقيقة الرسمي لحالة تسجيل الـ plugins في المستودع الحالي.

## تركيب الجذر (`UnivisUiPlugin`)

| Plugin | يُضاف عبر `UnivisUiPlugin` | ملاحظات |
|---|---|---|
| `UnivisInteractionPlugin` | نعم | يضيف picking backend وpointer observers. |
| `UnivisNodePlugin` | نعم | أساسيات الـ node والمواد. |
| `UnivisLayoutPlugin` | نعم | سلسلة layout والموارد المرتبطة. |
| `UnivisUiStylePlugin` | نعم | الخطوط/الأيقونات المضمنة + `Theme`. |
| `UnivisWidgetPlugin` | نعم | يسجّل مجموعة widget plugins الأساسية. |
| `LayoutProfilingPlugin` | لا | Plugin اختياري للتشخيص ويضاف يدويًا. |

## تركيب الـ Widgets (`UnivisWidgetPlugin`)

| Widget Plugin | يُسجّل تلقائيًا عبر `UnivisUiPlugin` | ملاحظات |
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
| `UnivisDragValuePlugin` | نعم | `UDragValue`. |
| `UnivisSelectPlugin` | نعم | `USelect`. |
| `UnivisTextFieldPlugin` | لا | مطلوب يدويًا عند استخدام سلوك/أحداث `UTextField`. |
| `UnivisBadgePlugin` | لا | مطلوب يدويًا لأنظمة تحديث `UBadge`/`UTag` الديناميكية. |

## مصادر التحقق

- `src/lib.rs`
- `src/widget/mod.rs`
- `src/layout/mod.rs`
