# Layout System

The layout system in `univis_ui` is built around two passes:

- an upward pass for intrinsic measurement
- a downward pass for constraint solving and placement

It operates on three core components:

- `UNode`: box metrics and basic visuals
- `ULayout`: container behavior
- `USelf`: per-child item behavior

## Important Files

- `crates/univis_ui_engine/src/layout/univis_node.rs`
- `crates/univis_ui_engine/src/layout/core/pass_up.rs`
- `crates/univis_ui_engine/src/layout/core/pass_down.rs`
- `crates/univis_ui_engine/src/layout/core/solver.rs`
- `crates/univis_ui_engine/src/layout/core/layout_cache.rs`

## Core Principles

- roots start from `URootUi`
- `LayoutDepth` is derived automatically from hierarchy traversal
- `IntrinsicSize` estimates content-driven sizing
- `ComputedSize` is the final size consumed by rendering
