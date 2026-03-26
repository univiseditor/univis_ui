# Performance and Diagnostics

The project currently centers around two main performance tools:

1. `LayoutCache` to reduce unnecessary recalculation
2. `LayoutProfilingPlugin` to measure runtime cost and display the overlay

## What To Watch

- dirty ratio and recalculated node count
- `pass_up` and `pass_down` timing
- material sync timing
- newly allocated versus reused materials
