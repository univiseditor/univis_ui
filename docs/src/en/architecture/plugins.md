# Plugin Map

## Root Plugin

In `src/lib.rs`, `UnivisUiPlugin` adds the stack in this order:

1. `UnivisUiStylePlugin`
2. `UnivisEnginePlugin`
3. `UnivisInteractionPlugin`
4. `UnivisWidgetPlugin`

## What Each Layer Adds

- `UnivisEnginePlugin`
  - `UnivisNodePlugin`
  - `UnivisLayoutPlugin`
  - `UnivisRenderPlugin`
  - together these provide node primitives, root resolution, layout solving, and render synchronization

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
- `UnivisBadgePlugin`
- `UnivisDragValuePlugin`
- `UnivisSelectPlugin`
- `UnivisTextFieldPlugin`

Dedicated widget plugins still remain available when you intentionally compose a narrower surface than `UnivisWidgetPlugin`.

## Quick Reference

See also: [Plugin Truth Table](plugin-truth-table.md)
