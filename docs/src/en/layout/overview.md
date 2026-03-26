# Layout System

The layout system in `univis_ui` is built around two phases:

- an upward pass for intrinsic measurement
- a downward pass for constraint solving and placement

It operates on three core components:

- `UNode`: box metrics and basic visuals
- `ULayout`: container behavior
- `USelf`: per-child item behavior

## Important Files

- `src/layout/univis_node.rs`
- `src/layout/core/pass_up.rs`
- `src/layout/core/pass_down.rs`
- `src/layout/core/solver.rs`
- `src/layout/core/layout_cache.rs`

## Core Principles

- roots start from `URootUi`
- `LayoutDepth` is derived automatically from hierarchy traversal
- `IntrinsicSize` estimates content-driven sizing
- `ComputedSize` is the final size consumed by rendering
