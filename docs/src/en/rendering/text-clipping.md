# Text Clipping Behavior

## Current Model

`UTextLabel` uses two separate clipping stages:

- local overflow handling inside the label itself
- optional ancestor clipping through `UClip`

## Local Label Clipping

Local clipping happens in `sync_text_label_meshes`.

- the system generates glyph quads from the measured text
- when `overflow` is `Clip` or `Ellipsis`, each quad is clipped against the label content rect before the mesh is built
- when `overflow` is `Visible`, this local clip is disabled

If `Ellipsis` is active, text is measured first, shortened if needed, then the remaining glyph quads are clipped to the label bounds.

Autosized labels still respect parent constraints when the parent has explicit bounds, so overflow handling can stop at the parent edge instead of growing forever.

## Ancestor `UClip`

Ancestor clipping happens in `sync_text_clipper_materials`.

- the system walks upward from each generated text mesh
- it finds the nearest enabled `UClip`
- it copies that clip center, size, and corner radius into the text SDF material
- the shader clips the text against that rounded rect

This means text clipping now matches the material-based clip path used by normal nodes instead of hiding the entire text entity.

## What Changed

Older docs described a visibility-only fallback that hid text when it escaped a clipped ancestor.

That is no longer the current behavior:

- text is clipped at the glyph-quad level locally
- ancestor clip data is applied in the material/shader path
- partial text visibility is supported instead of all-or-nothing hiding

## Known Limitation

Today the text material uses the nearest enabled `UClip` ancestor. Nested clip ancestors are not combined into a single intersection for text.
