# Current Max Benchmark Baseline

Captured on `2026-04-05T16:49:37Z`.

This directory freezes the current benchmark baseline for the repo's current maximum built-in load scenarios so they can be compared after frame-zero and performance work lands.

## Scope

The "current maximum level" in this capture means the highest-load scenarios currently encoded in the benchmark harnesses:

- solver:
  - `dense_row_512`
  - `wrap_cards_400`
  - `grid_dashboard_196`
- runtime:
  - `root_capsules_96`
  - `text_measure_180`
  - `picking_grid_512`
  - `widget_panels_240`
  - `world3d_panels_48`

These were run with:

- `UNIVIS_PERF_WARMUP=48`
- `UNIVIS_PERF_ITERATIONS=240`
- git commit: `b9008b419694acb1a35cf0795593e989a80b4593`

## Environment

- OS: `Linux archlinux 6.19.10-arch1-1 x86_64`
- CPU: `Intel(R) Core(TM) i5-7300U CPU @ 2.60GHz`
- logical CPUs: `4`
- `rustc`: `1.94.1 (e408947bf 2026-03-25)`
- `cargo`: `1.94.1 (29ea6fb6a 2026-03-24)`

## Summary

### Solver

| Scenario | Items | Avg ms | p95 ms | Max ms | Budget ms | Status |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `dense_row_512` | 512 | 0.042 | 0.058 | 0.315 | 1.000 | `ok` |
| `wrap_cards_400` | 400 | 0.037 | 0.044 | 0.389 | 1.400 | `ok` |
| `grid_dashboard_196` | 196 | 0.022 | 0.022 | 0.025 | 1.800 | `ok` |

### Runtime

| Scenario | Items | Avg ms | p95 ms | Max ms | Budget ms | Status |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `root_capsules_96` | 96 | 2.133 | 2.844 | 3.437 | 6.000 | `ok` |
| `text_measure_180` | 180 | 2.090 | 3.036 | 3.818 | 4.750 | `ok` |
| `picking_grid_512` | 512 | 0.496 | 0.599 | 1.065 | 4.000 | `ok` |
| `widget_panels_240` | 240 | 2.415 | 3.344 | 3.812 | 8.000 | `ok` |
| `world3d_panels_48` | 48 | 1.039 | 1.375 | 1.760 | 6.000 | `ok` |

## Files

- `solver_benchmarks.txt`: raw solver benchmark output
- `runtime_benchmarks.txt`: raw runtime benchmark output
- `manifest.json`: machine-readable metadata and results

## Comparison Guidance

When re-running after the optimization work:

1. use the same machine when possible
2. use the same warmup/iteration counts
3. compare `p95` first, then `avg`, then `max`
4. treat regressions in `text_measure_180`, `widget_panels_240`, and `root_capsules_96` as the most relevant for frame-zero settlement work
