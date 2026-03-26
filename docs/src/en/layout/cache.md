# Cache and Invalidation

`LayoutCache` reduces repeated layout work.

## What It Stores

- intrinsic sizes per entity
- dirty flags
- per-depth entity lists

## When A Node Becomes Dirty

Whenever one of these changes:

- `UNode`
- `ULayout`
- `USelf`
- hierarchy structure

Important exception:

- if the only change is `IntrinsicSize` on a node with children, the cache may ignore it to reduce noise

## Lifecycle

- `track_layout_changes` marks dirty nodes
- `update_depth_cache` rebuilds the depth map on structural changes
- `upward_measure_pass_cached` reads the cache and clears dirty flags afterward

## Performance Diagnostics

- watch `dirty_count` and `dirty_ratio`
- enable the profiler overlay when needed
