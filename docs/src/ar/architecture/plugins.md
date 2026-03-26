# خريطة الإضافات

## الإضافة الجذرية

في `src/lib.rs`:

- `UnivisUiPlugin` يضيف بالترتيب:
  1. `UnivisInteractionPlugin`
  2. `UnivisNodePlugin`
  3. `UnivisLayoutPlugin`
  4. `UnivisUiStylePlugin`
  5. `UnivisWidgetPlugin`

## التخطيط

- `UnivisNodePlugin`:
  - تسجيل أنواع التخطيط وامتدادات الانعكاس.
- `UnivisLayoutPlugin`:
  - المورد `LayoutTreeDepth`
  - `LayoutCachePlugin`
  - سلسلة التخطيط في `PostUpdate`.
- `UnivisRenderPlugin`:
  - مواد 2D/3D + مزامنة المواد.

## التفاعل

- `UnivisInteractionPlugin`:
  - `univis_picking_backend` في `PreUpdate`.
  - مراقبو الأحداث:
    - `on_pointer_over`
    - `on_pointer_out`
    - `on_pointer_press`
    - `on_pointer_release`
    - `on_pointer_click`

## النمط

- `UnivisUiStylePlugin`:
  - تحميل خطوط مضمّنة.
  - تحميل خط أيقونات Lucide.
  - إنشاء المورد `Theme`.

## الوحدات الجاهزة

`UnivisWidgetPlugin` يضيف مجموعة الوحدات الجاهزة الأساسية. الحالة الحالية:

- مضاف تلقائيًا:
  - `UnivisTextPlugin`
  - `UnivisProgressPlugin`
  - `UnivisButtonPlugin`
  - `UnivisRadioPlugin`
  - `UnivisIconButtonPlugin`
  - `UnivisTogglePlugin`
  - `UnivisCheckboxPlugin`
  - `UnivisSeekBarPlugin`
  - `UnivisScrollViewPlugin`
  - `UnivisDividerPlugin`
  - `UnivisPanelPlugin`
  - `UnivisDragValuePlugin`
  - `UnivisSelectPlugin`

- غير مضاف تلقائيًا:
  - `UnivisTextFieldPlugin`
  - `UnivisBadgePlugin`

> السبب: الحفاظ على اختيارية بعض السلوكيات وعدم فرضها على كل تطبيق.

## مرجع سريع

- راجع أيضًا: [جدول حقيقة الإضافات](plugin-truth-table.md)
