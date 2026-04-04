# Benchmark Harness

The repository now includes a CLI baseline suite with two harnesses:

- script: `./scripts/run_perf_baselines.sh`
- solver example: `cargo run -p univis_ui_engine --release --example solver_benchmarks`
- runtime example: `cargo run -p univis_ui_engine --release --example runtime_benchmarks`

## Current Coverage

The current baseline wave covers both algorithm-heavy and runtime-heavy paths.

Solver scenarios:

- `dense_row_512`
- `wrap_cards_400`
- `grid_dashboard_196`

Runtime scenarios:

- `root_capsules_96`
- `text_measure_180`
- `picking_grid_512`
- `widget_panels_240`
- `world3d_panels_48`

The solver harness gives one dense flex row, one wrapped card layout, and one grid-heavy dashboard.
The runtime harness adds root resolution / stacking, text measurement churn, picking traversal,
widget-heavy panels, and world-3d panel scenes.

## How To Run

```bash
./scripts/run_perf_baselines.sh
```

To run only one harness directly:

```bash
cargo run -p univis_ui_engine --release --example solver_benchmarks
cargo run -p univis_ui_engine --release --example runtime_benchmarks
```

To shorten the run while iterating locally:

```bash
UNIVIS_PERF_WARMUP=12 UNIVIS_PERF_ITERATIONS=40 ./scripts/run_perf_baselines.sh
```

To fail the command when a scenario exceeds its current budget:

```bash
./scripts/run_perf_baselines.sh --check
```

## Output

The harness prints:

- scenario name
- item count
- average update / solve time in milliseconds
- `p95` time in milliseconds
- max solve time in milliseconds
- current per-scenario budget
- status (`ok` / `over`)

`p95` is the value used for the current budget gate because it is less noisy than the single worst sample.

## Current Budgets

Solver budgets:

- `dense_row_512`: `1.000ms` p95
- `wrap_cards_400`: `1.400ms` p95
- `grid_dashboard_196`: `1.800ms` p95

Runtime budgets:

- `root_capsules_96`: `6.000ms` p95
- `text_measure_180`: `4.750ms` p95
- `picking_grid_512`: `4.000ms` p95
- `widget_panels_240`: `8.000ms` p95
- `world3d_panels_48`: `6.000ms` p95

These are repository baselines for the current harnesses, not universal promises for every machine.
Treat them as drift detectors for this repo and this benchmark shape.
