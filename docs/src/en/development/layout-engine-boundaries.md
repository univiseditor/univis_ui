# Layout Engine Boundaries

This page is the maintainer reference for where layout responsibilities start and stop inside `univis_ui_engine`.

## Authored Surface

- `layout::layout_system`: public root model and resolved root state
- `layout::univis_node`: public node, layout, and local-position components
- `layout::geometry`: public sizing and spacing primitives
- `layout::image` and `layout::pbr`: public authored helpers tied to rendering inputs

## Internal Layout Pipeline

- `layout::core::hierarchy`: depth tracking and cached ancestry refresh
- `layout::core::layout_cache`: dirty tracking, stage generations, and frontier queues
- `layout::core::pass_up`: intrinsic measurement and content-size propagation
- `layout::core::pass_down`: final solve output and child transform placement
- `layout::core::solver`: size solving and placement bridge integration
- `layout::algorithms`: algorithm-specific placement logic

## Render Boundary

- layout solves logical size and position
- render sync consumes solved results and updates meshes/materials
- text rendering is the main cross-boundary case, so `text_label` owns both measurement and render-facing logic on the widget side

## Scheduling Boundary

- `layout::registration` wires the settlement schedule and stage ordering
- `layout::settlement_loop` owns bounded execution, generation tracking, and per-frame fixed-point checks
- other modules should describe work, not decide global settlement policy

## Dependency Direction

- root resolution may feed hierarchy, solve, render sync, and picking
- hierarchy and cache may invalidate later stages
- render sync must not reintroduce measure/solve work unless a real authored layout input changed
- diagnostics and validation can observe the pipeline but should not silently redefine its semantics
