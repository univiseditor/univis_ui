# Public Types Table

This page groups the highest-value public types by crate and by job.

## Facade Entry

- `univis_ui::UnivisUiPlugin`
- `univis_ui::prelude`

## Root And Layout Types

- `univis_ui_engine::layout::layout_system::URootUi`
- `univis_ui_engine::layout::layout_system::UiSpace`
- `univis_ui_engine::layout::layout_system::UiCanvasSize`
- `univis_ui_engine::layout::layout_system::UiCameraRef`
- `univis_ui_engine::layout::univis_node::UNode`
- `univis_ui_engine::layout::univis_node::ULayout`
- `univis_ui_engine::layout::univis_node::USelf`
- `univis_ui_engine::layout::univis_node::UBorder`
- `univis_ui_engine::layout::univis_node::UClip`
- `univis_ui_engine::layout::pbr::UPbr`
- `univis_ui_engine::layout::geometry::UVal`
- `univis_ui_engine::layout::geometry::USides`
- `univis_ui_engine::layout::geometry::UCornerRadius`

`UScreenRoot` and `UWorldRoot` are intentionally omitted from the recommended surface.
Use their explicit compatibility path only when migrating older code:
`univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}` or
`univis_ui_engine::layout::layout_system::{UScreenRoot, UWorldRoot}`.

## Interaction Types

- `univis_ui_interaction::interaction::feedback::UInteraction`
- `univis_ui_interaction::interaction::feedback::UInteractionColors`
- `univis_ui_interaction::interaction::UnivisInteractionPlugin`

## Style Types

- `univis_ui_style::style::Theme`
- `univis_ui_style::style::TextStyles`
- `univis_ui_style::style::IconStyles`
- `univis_ui_style::style::Fonts`
- `univis_ui_style::style::UnivisUiStylePlugin`

## Widget Types

- `univis_ui_widgets::widget::text_label::UTextLabel`
- `univis_ui_widgets::widget::button::UButton`
- `univis_ui_widgets::widget::checkbox::UCheckbox`
- `univis_ui_widgets::widget::toggle::UToggle`
- `univis_ui_widgets::widget::radio::{URadioButton, URadioGroup}`
- `univis_ui_widgets::widget::seekbar::USeekBar`
- `univis_ui_widgets::widget::drag_value::UDragValue`
- `univis_ui_widgets::widget::select::{USelect, USelectOption}`
- `univis_ui_widgets::widget::text_field::UTextField`
- `univis_ui_widgets::widget::scroll_view::UScrollContainer`
- `univis_ui_widgets::widget::panel::{UPanel, UPanelWindow}`
- `univis_ui_widgets::widget::progress::UProgressBar`
- `univis_ui_widgets::widget::badge::{UBadge, UTag}`

## Diagnostics And Rendering Types

- `univis_ui_engine::layout::render::UnivisRenderPlugin`
- `univis_ui_engine::layout::profiling::{LayoutProfiler, UnivisLayoutProfilingPlugin}`
- `univis_ui_engine::layout::core::layout_cache::{LayoutCache, UnivisLayoutCachePlugin}`

For exact signatures and field-level detail, use `cargo doc --no-deps`.
