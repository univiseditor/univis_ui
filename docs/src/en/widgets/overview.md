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

- included by default through `UnivisWidgetPlugin`: most widgets, scrolling, and panel support
- still optional:
  - `UnivisTextFieldPlugin`
  - `UnivisBadgePlugin`

## Related Examples

- [`widgets`](../examples/index.md#widgets) in `univis_ui_widgets`
- [`text_label`](../examples/index.md#text_label) in `univis_ui_widgets`
- [`text_field`](../examples/index.md#text_field) in `univis_ui_widgets`
- [`panel_window`](../examples/index.md#panel_window) in `univis_ui_widgets`
- [`scroll_view`](../examples/index.md#scroll_view) in `univis_ui_widgets`

## API Entry Points

- `univis_ui_widgets::widget::text_label::UTextLabel`
- `univis_ui_widgets::widget::button::UButton`
- `univis_ui_widgets::widget::text_field::UTextField`
- `univis_ui_widgets::widget::panel::{UPanel, UPanelWindow}`
- `univis_ui_widgets::widget::scroll_view::UScrollContainer`

## Where To Look Next

- related example: [`widgets`](../examples/index.md#widgets)
- related API index: [API Reference](../api/index.md)
- related migration page: [Example Path Migration](../migration/example-paths.md)
