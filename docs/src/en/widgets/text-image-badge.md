# Text, Image, and Badge

## `UTextLabel`

File: `src/widget/text_label.rs`

Core fields:

- `text`
- `font_size`
- `color`
- `justify`
- `linebreak`
- `overflow` defaults to `Ellipsis`; use `Clip` or `Visible` when needed
- `truncate_side` controls whether ellipsis trims the start, end, or middle

Systems:

- `measure_text_label_layout`
- `fit_node_to_text_size`
- `sync_text_label_meshes`
- `sync_text_clipper_materials`

Local overflow is clipped per glyph quad, while ancestor `UClip` data is pushed into the text material for shader-side clipping.

## `UImage`

File: `src/widget/image.rs`

- binds an image into the material path
- `sync_image_geometry` keeps rendered size aligned with layout
- `Auto`, `Content`, `MinContent`, and `MaxContent` resolve to the native texture size once the image asset is available
- `UNode` `min/max` constraints still clamp the final solved image size after native measurement

## `UBadge` and `UTag`

File: `src/widget/badge.rs`

- badge style presets
- dynamic badge styling lives in `UnivisBadgePlugin`
- this runtime is included by default through `UnivisWidgetPlugin`
- the dedicated plugin still remains available when you intentionally compose a narrower widget surface

## Live Example

- use `cargo run --example widgets_display` for `UBadge`, `UDivider`, `UProgressBar`, and `UPanel`
