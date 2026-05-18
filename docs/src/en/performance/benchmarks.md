# Benchmark Harness

The committed benchmark reports are the current reference for performance comparisons.

## Current Reference Artifacts

- `perf_baselines/current_max/2026-04-05/solver_benchmarks.txt`
- `perf_baselines/current_max/2026-04-05/runtime_benchmarks.txt`
- `perf_baselines/current_max/2026-04-05/manifest.json`
- `perf_baselines/current_max/2026-04-05/README.md`

## Current Coverage

The committed baseline wave still covers both algorithm-heavy and runtime-heavy paths:

- solver scenarios such as dense rows, wrapped cards, and grid dashboards
- runtime scenarios such as root capsules, idle-after-settle, render-only churn, text measurement, picking traversal, widget-heavy panels, and `World3d` panels

## How To Use It Now

- read the committed `.txt` reports for the raw scenario output
- read `manifest.json` for the structured metadata
- use `scripts/render_benchmark_report.py` if you want a rendered report from the stored data

## Status Of `run_perf_baselines.sh`

`./scripts/run_perf_baselines.sh` currently points maintainers to the committed baseline data used by this branch.
