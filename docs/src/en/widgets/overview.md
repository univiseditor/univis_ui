# Widgets

Each Univis widget is an ECS component plus a small plugin that manages:

- shape or child initialization
- interaction logic
- visual synchronization
- message emission when needed

## Quick Mental Model

- Display and text:
  - `UTextLabel`, `UImage`, `UBadge`, `UTag`, `UProgressBar`
- Actions:
  - `UButton`, `UIconButton`, `UToggle`, `UCheckbox`, `URadioButton`
- Inputs:
  - `USeekBar`, `UDragValue`, `USelect`, `UTextField`
- Containers:
  - `UPanel`, `UPanelWindow`, `UScrollContainer`, `UDivider`

## Important Plugins

- included by default through `UnivisUiPlugin` -> `UnivisWidgetPlugin`: the standard widget set, scrolling, panel support, `UTextField` runtime, and `UBadge` runtime
- dedicated widget plugins still remain available when you intentionally want a narrower surface than `UnivisWidgetPlugin`

## Related Examples

- live now: [`widgets_controls`](../examples/index.md#widgets)
- live now: [`widgets_inputs`](../examples/index.md#widgets)
- live now: [`widgets_display`](../examples/index.md#widgets)
- live now: [`widgets_containers`](../examples/index.md#widgets)

## API Entry Points

- `univis_ui_widgets::widget::text_label::UTextLabel`
- `univis_ui_widgets::widget::button::UButton`
- `univis_ui_widgets::widget::text_field::UTextField`
- `univis_ui_widgets::widget::panel::{UPanel, UPanelWindow}`
- `univis_ui_widgets::widget::scroll_view::UScrollContainer`

## Where To Look Next

- related example: [`widgets_controls`](../examples/index.md#widgets)
- related setup page: [Plugin Setup and First Examples](../first-steps.md)
- related API index: [API Reference](../api/index.md)
- related migration page: [Example Path Migration](../migration/example-paths.md)
