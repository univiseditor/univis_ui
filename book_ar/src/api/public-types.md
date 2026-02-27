# جدول الأنواع العامة

هذا الجدول يصنف أهم الأنواع العامة (`pub`) حسب الوحدة.

## Core / Root

- `UnivisUiPlugin`
- `UnivisLayoutPlugin`
- `UnivisInteractionPlugin`
- `UnivisUiStylePlugin`
- `UnivisWidgetPlugin`

## Layout Types

- `UNode`, `UBorder`, `UShapeMode`
- `ULayout`, `USelf`
- `UDisplay`, `UFlexDirection`, `UJustifyContent`, `UAlignItems`, `UAlignSelf`
- `UVal`, `USides`, `UCornerRadius`
- `ComputedSize`, `IntrinsicSize`, `LayoutDepth`, `LayoutTreeDepth`
- `UScreenRoot`, `UWorldRoot`, `UI3d`
- `UClip`, `UPbr`

### Advanced Ext Types

- `ULayoutContainerExt`
- `ULayoutBoxAlignContainer`
- `ULayoutFlexContainer`
- `ULayoutGridContainer`
- `ULayoutItemExt`
- `ULayoutBoxAlignSelf`
- `ULayoutFlexItem`
- `ULayoutGridItem`
- `UAlignSelfExt`, `UAlignItemsExt`, `UContentAlignExt`
- `UOverflowPosition`, `UFlexWrap`, `UTrackSize`, `UGridAutoFlow`

## Interaction Types

- `UInteraction`
- `UInteractionColors`

## Widgets

- `UTextLabel`, `UnivisTextPlugin`
- `UImage`
- `UButton`, `UnivisButtonPlugin`
- `UIconButton`, `UnivisIconButtonPlugin`
- `UCheckbox`, `UnivisCheckboxPlugin`
- `UToggle`, `UnivisTogglePlugin`
- `URadioButton`, `URadioGroup`, `UnivisRadioPlugin`
- `USeekBar`, `UnivisSeekBarPlugin`
- `UProgressBar`, `UnivisProgressPlugin`
- `UTextField`, `UnivisTextFieldPlugin`
- `UScrollContainer`, `UnivisScrollViewPlugin`
- `UDivider`, `UDividerOrientation`, `UnivisDividerPlugin`
- `UPanel`, `UPanelWindow`, `UnivisPanelPlugin`
- `UDragValue`, `UnivisDragValuePlugin`
- `USelect`, `USelectOption`, `UnivisSelectPlugin`
- `UBadge`, `UTag`, `UnivisBadgePlugin`

## Rendering / Materials

- `UNodeMaterial`
- `UNodeMaterial3d`
- `UnivisRenderPlugin`

## Performance

- `LayoutCache`, `LayoutCachePlugin`
- `LayoutProfiler`, `LayoutProfilingPlugin`
- `ProfilerSettings`, `OverlayPosition`

> للحصول على signatures والتفاصيل الدقيقة لكل نوع/دالة، استخدم `cargo doc --no-deps --open`.
