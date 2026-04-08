# Performance and Diagnostics

The project currently centers around three main performance tools:

1. `LayoutCache` to reduce unnecessary recalculation
2. `UnivisLayoutProfilingPlugin` to measure runtime cost and display the overlay
3. `./scripts/run_perf_baselines.sh` for repeatable CLI baseline runs

## What To Watch

- dirty ratio and recalculated node count
- `pass_up` and `pass_down` timing
- material sync timing
- newly allocated versus reused materials
- baseline drift in the solver and runtime benchmark scenarios
