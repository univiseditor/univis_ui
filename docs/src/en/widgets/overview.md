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
