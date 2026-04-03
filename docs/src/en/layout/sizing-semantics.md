# Sizing Semantics

This page freezes the intended sizing vocabulary for the current alpha line.

## Core Terms

- Preferred size: the size requested by `width` and `height`.
- Minimum size: the floor applied by `min_width` and `min_height`.
- Maximum size: the ceiling applied by `max_width` and `max_height`.
- Intrinsic size: the size measured from content before parent-driven layout adjustments.

## `UVal` Modes

- `Px`: fixed logical size.
- `Percent`: percentage of the parent size on that axis.
- `Flex`: participates in main-axis free-space distribution.
- `MinContent`: smallest intrinsic size that still fits the content.
- `MaxContent`: full intrinsic size without contextual stretch.
- `Content`: legacy public alias for `MaxContent`.
- `Auto`: contextual size. It falls back to intrinsic measurement, but the layout algorithm may stretch or reinterpret it when the surrounding layout calls for that behavior.

## Current Rules

- Explicit `min_*` and `max_*` always clamp the solved size, including flex grow and flex shrink redistribution.
- `Auto` is no longer treated as the same mode as `Content`.
- In grid placement, `Auto` items can stretch to the cell by default. `MinContent` and `MaxContent` keep their intrinsic size unless alignment explicitly asks for stretch.
- In flex cross-axis stretch behavior, `Auto` participates in implicit stretch. Explicit alignment can still force stretch for non-fixed, non-percent modes.
- `UImage` treats `Auto`, `Content`, `MinContent`, and `MaxContent` as native-image measurement modes. Images have one intrinsic size, so these modes all resolve to the texture size once the asset is ready.

## Practical Guidance

- Use `Auto` when the size should adapt to layout context.
- Use `MaxContent` or `Content` when the element should hug its measured content and avoid implicit auto-stretch behavior.
- Use `MinContent` when you want the tightest intrinsic fit.
- Add `min_width` / `max_width` or `min_height` / `max_height` when you need hard bounds around any of the modes above.
