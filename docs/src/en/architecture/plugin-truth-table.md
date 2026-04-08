# Plugin Truth Table

This page is the canonical plugin registration truth for the current repository state.

## Root Composition (`UnivisUiPlugin`)

| Plugin | Added by `UnivisUiPlugin` | Notes |
|---|---|---|
| `UnivisUiStylePlugin` | Yes | Embedded fonts/icons + `Theme` resource. |
| `UnivisEnginePlugin` | Yes | Adds `UnivisNodePlugin`, `UnivisLayoutPlugin`, and `UnivisRenderPlugin`. |
| `UnivisInteractionPlugin` | Yes | Registers picking backend and pointer observers. |
| `UnivisWidgetPlugin` | Yes | Registers built-in widget plugin set. |
| `UnivisLayoutProfilingPlugin` | No | Optional diagnostics plugin; add manually when needed. |

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
| `UnivisBadgePlugin` | Yes | `UBadge`, plus dynamic badge/tag visual updates. |
| `UnivisDragValuePlugin` | Yes | `UDragValue`. |
| `UnivisSelectPlugin` | Yes | `USelect`. |
| `UnivisTextFieldPlugin` | Yes | `UTextField` behavior/events. |

Dedicated widget plugins still remain available when you intentionally build a narrower widget surface than `UnivisWidgetPlugin`.

## Verification Sources

- `src/lib.rs`
- `crates/univis_ui_engine/src/lib.rs`
- `crates/univis_ui_widgets/src/widget/mod.rs`
