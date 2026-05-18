# Example Validation Report

## Current Validation Scope

The current branch validates only example sources that exist in the repository.

## Commands

Check workspace examples:

```bash
cargo check --workspace --examples
```

Check the standalone Android package:

```bash
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
```

Run the release-mode representative pass:

```bash
./scripts/check_representative_examples.sh
```

## Current Example Sources

- `android/android_phone_app/examples/android_phone.rs`
- `examples/responsive_layout_test.rs`
- `examples/toggle_seekbar.rs`
- `examples/z_order_hierarchy.rs`
- `examples/grid/auto_flow.rs`
- `examples/grid/columns.rs`
- `examples/grid/item_placement.rs`
- `examples/grid/tracks.rs`
- `examples/layout/flex.rs`
- `examples/layout/masonry.rs`
- `examples/layout/radial.rs`
- `examples/layout/stack.rs`
- `examples/widgets/containers.rs`
- `examples/widgets/controls.rs`
- `examples/widgets/display.rs`
- `examples/widgets/inputs.rs`

Runtime behavior still requires manual windowed smoke checks for interaction and visual correctness.
