# Layout Cache

File: `src/layout/core/layout_cache.rs`

`LayoutCache` stores:

- cached intrinsic sizes
- dirty flags
- per-depth entity lists

## Best Practices

- avoid mutating `UNode`, `ULayout`, or `USelf` every frame without a real reason
- prefer state-driven updates
- avoid unnecessary hierarchy churn

## When To Extend It

- when node counts grow much larger
- when dirty ratios stay high most of the time

## Related Validation

- `cargo test -p univis_ui_engine --lib layout::core::layout_cache`
