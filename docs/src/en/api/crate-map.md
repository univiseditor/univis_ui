# Crate Map

Use the smallest public surface that matches your job.

## Recommended Starting Point

- `univis_ui` is the default dependency for most applications.
- `univis_ui::prelude` is the recommended day-to-day import surface.
- The facade prelude includes the canonical root, layout, interaction, style, and built-in widget APIs.
- Deprecated compatibility wrappers are intentionally excluded from the prelude. Use explicit paths such as `univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot}` only when migrating older code.

## When To Depend On Each Crate

- `univis_ui`: use this when you want one plugin and one import story for the full stack.
- `univis_ui_engine`: use this when you need roots, layout, rendering sync, or custom widget/runtime work without the full facade.
- `univis_ui_interaction`: use this when you need picking and `UInteraction`-driven feedback on top of engine roots.
- `univis_ui_widgets`: use this when you want the built-in controls but still manage plugin composition yourself.
- `univis_ui_style`: use this when you only need shared theme resources, fonts, icons, or text styles.

## Prelude Policy

- `prelude` means the stable recommended import surface for everyday use.
- Named modules such as `layout`, `render`, `interaction`, and `widget` are advanced but still public.
- Deprecated compatibility wrappers stay on explicit paths instead of the default import story.
