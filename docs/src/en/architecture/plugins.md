# Plugin Map

## Root Plugin

In `src/lib.rs`, `UnivisUiPlugin` adds the stack in this order:

1. `UnivisInteractionPlugin`
2. `UnivisNodePlugin`
3. `UnivisLayoutPlugin`
4. `UnivisUiStylePlugin`
5. `UnivisWidgetPlugin`

## What Each Layer Adds

- `UnivisNodePlugin`
  - core node components
  - shared materials and render-side assets
- `UnivisLayoutPlugin`
  - layout type registration
  - `LayoutCachePlugin`
  - the `PostUpdate` layout chain
- `UnivisRenderPlugin`
  - 2D and 3D material paths
  - material synchronization systems

## Interaction

`UnivisInteractionPlugin` adds:

- `univis_picking_backend` in `PreUpdate`
- pointer feedback observers
- interaction state transitions

## Style

`UnivisUiStylePlugin` adds:

- bundled fonts
- Lucide icon font loading
- the shared `Theme` resource

## Widgets

`UnivisWidgetPlugin` registers the standard widget set.

Automatically included today:

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

Still optional:

- `UnivisTextFieldPlugin`
- `UnivisBadgePlugin`

These remain opt-in so applications only pay for the behavior they actually use.

## Quick Reference

See also: [Plugin Truth Table](plugin-truth-table.md)
