# خريطة الـ Plugins

## Plugin الجذر

في `src/lib.rs`:

- `UnivisUiPlugin` يضيف بالترتيب:
  1. `UnivisInteractionPlugin`
  2. `UnivisNodePlugin`
  3. `UnivisLayoutPlugin`
  4. `UnivisUiStylePlugin`
  5. `UnivisWidgetPlugin`

## Layout

- `UnivisNodePlugin`:
  - تسجيل أنواع التخطيط والامتدادات reflection.
- `UnivisLayoutPlugin`:
  - `LayoutTreeDepth` resource
  - `LayoutCachePlugin`
  - سلسلة layout في `PostUpdate`.
- `UnivisRenderPlugin`:
  - مواد 2D/3D + مزامنة المواد.

## Interaction

- `UnivisInteractionPlugin`:
  - `univis_picking_backend` في `PreUpdate`.
  - observers:
    - `on_pointer_over`
    - `on_pointer_out`
    - `on_pointer_press`
    - `on_pointer_release`
    - `on_pointer_click`

## Style

- `UnivisUiStylePlugin`:
  - تحميل خطوط مضمّنة.
  - تحميل خط أيقونات Lucide.
  - إنشاء `Theme` resource.

## Widgets

`UnivisWidgetPlugin` يضيف مجموعة widgets الأساسية. الحالة الحالية:

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
