# Upward / Downward Passes and Solver

## Upward Pass: `upward_measure_pass_cached`

- walks from the deepest level back to the root
- computes `IntrinsicSize` for containers from their children
- ignores `Absolute` items that are out of flow
- uses the cache to skip clean nodes

## Downward Pass: `downward_solve_pass_safe`

- walks from the root toward deeper levels
- builds `SolverConfig` and `SolverSpec` for each node
- calls `solve_flex_layout`
- writes results into:
  - `ComputedSize`
  - `Transform`

## Solver

In `src/layout/core/solver.rs`, the solver:

- separates in-flow and absolute items
- resolves main and cross sizes
- applies flex grow and shrink
- applies align and stretch rules
- resolves the absolute box at the end

## `translate_spec` / `translate_config`

- `translate_config` reads container-level data from `ULayout`
- `translate_spec` reads item-level data from `USelf`

This translation layer is the bridge between public layout components and the internal solver.
