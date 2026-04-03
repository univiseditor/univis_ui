# خريطة الإضافات

## الإضافة الجذرية

في `src/lib.rs`:

- `UnivisUiPlugin` يضيف بالترتيب:
  1. `UnivisUiStylePlugin`
  2. `UnivisEnginePlugin`
  3. `UnivisInteractionPlugin`
  4. `UnivisWidgetPlugin`

## التخطيط

- `UnivisEnginePlugin`:
  - `UnivisNodePlugin`
  - `UnivisLayoutPlugin`
  - `UnivisRenderPlugin`
  - وتوفر معًا بدائيات العقدة وحسم الجذور وحل التخطيط ومزامنة الرندر.

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
  - `UnivisBadgePlugin`
  - `UnivisDragValuePlugin`
  - `UnivisSelectPlugin`
  - `UnivisTextFieldPlugin`

وتبقى الإضافات المخصصة نفسها متاحة عندما تريد تركيب سطح widgets أضيق من `UnivisWidgetPlugin`.

## مرجع سريع

- راجع أيضًا: [جدول حقيقة الإضافات](plugin-truth-table.md)
