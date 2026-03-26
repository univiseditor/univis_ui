# Text Clipping Behavior

## The Problem

Text does not automatically use the same material path as `UNodeMaterial`, so it does not inherit clip masking by default.

## Current Project Solution

In `src/widget/text_label.rs`:

- `sync_text_clip_visibility`
  - computes a world quad for the text
  - walks clip ancestors through `UClip`
  - hides the text via `Visibility::Hidden` if it escapes any clip ancestor

## Benefit

- text no longer appears outside the clip frame
- clipping behavior stays visually aligned with `UClip`

## Known Limitation

- this is still visibility-based, not true per-pixel glyph clipping
- precise glyph clipping would need a dedicated text render path or direct shader clipping in the glyph pipeline
