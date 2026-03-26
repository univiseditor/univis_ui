# Text, Image, and Badge

## `UTextLabel`

File: `src/widget/text_label.rs`

Core fields:

- `text`
- `font_size`
- `color`
- `justify`
- `linebreak`

Systems:

- text measurement
- glyph geometry generation
- clip visibility sync

`sync_text_clip_visibility` prevents text from appearing outside clip ancestors.

## `UImage`

File: `src/widget/image.rs`

- binds an image into the material path
- `sync_image_geometry` keeps rendered size aligned with layout

## `UBadge` and `UTag`

File: `src/widget/badge.rs`

- badge style presets
- dynamic badge styling lives in `UnivisBadgePlugin`
- this plugin remains optional and is not auto-registered by `UnivisWidgetPlugin`
