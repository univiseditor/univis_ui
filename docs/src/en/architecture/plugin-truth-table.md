# Plugin Truth Table

This page is the canonical plugin registration truth for the current repository state.

## Root Composition (`UnivisUiPlugin`)

| Plugin | Added by `UnivisUiPlugin` | Notes |
|---|---|---|
| `UnivisInteractionPlugin` | Yes | Registers picking backend and pointer observers. |
| `UnivisNodePlugin` | Yes | Core node/material foundations. |
| `UnivisLayoutPlugin` | Yes | Layout chain + layout resources. |
| `UnivisUiStylePlugin` | Yes | Embedded fonts/icons + `Theme` resource. |
| `UnivisWidgetPlugin` | Yes | Registers built-in widget plugin set. |
| `LayoutProfilingPlugin` | No | Optional diagnostics plugin; add manually when needed. |

## Widget Composition (`UnivisWidgetPlugin`)

| Widget Plugin | Auto-registered via `UnivisUiPlugin` | Notes |
|---|---|---|
| `UnivisTextPlugin` | Yes | Text label and text clipping systems. |
| `UnivisProgressPlugin` | Yes | `UProgressBar`. |
| `UnivisButtonPlugin` | Yes | `UButton`. |
| `UnivisRadioPlugin` | Yes | `URadioButton`, `URadioGroup`. |
| `UnivisIconButtonPlugin` | Yes | `UIconButton`. |
| `UnivisTogglePlugin` | Yes | `UToggle`. |
| `UnivisCheckboxPlugin` | Yes | `UCheckbox`. |
| `UnivisSeekBarPlugin` | Yes | `USeekBar`. |
| `UnivisScrollViewPlugin` | Yes | `UScrollContainer`. |
| `UnivisDividerPlugin` | Yes | `UDivider`. |
| `UnivisPanelPlugin` | Yes | `UPanel`, `UPanelWindow` behavior. |
| `UnivisDragValuePlugin` | Yes | `UDragValue`. |
| `UnivisSelectPlugin` | Yes | `USelect`. |
| `UnivisTextFieldPlugin` | No | Required when using `UTextField` behavior/events. |
| `UnivisBadgePlugin` | No | Required for dynamic `UBadge`/`UTag` update systems. |

## Verification Sources

- `src/lib.rs`
- `src/widget/mod.rs`
- `src/layout/mod.rs`
