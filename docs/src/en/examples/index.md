# Examples

This page lists only example source files that exist in the current repository.

## Run The Current Demo

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## Standalone Android Package

| Example | Source | Purpose | Command |
| --- | --- | --- | --- |
| `android_phone` | `android/android_phone_app/examples/android_phone.rs` | Android-style app screen with search, toggles, sliders, scroll content, and bottom navigation. | `cargo run --manifest-path android/android_phone_app/Cargo.toml --example android_phone` |

The package also has a native desktop entry point:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## Workspace Examples

| Example | Source | Purpose | Command |
| --- | --- | --- | --- |
| `responsive_layout_test` | `examples/responsive_layout_test.rs` | Stress scene for responsive screen composition. | `cargo run --example responsive_layout_test` |
| `toggle_seekbar` | `examples/toggle_seekbar.rs` | Compact control scene for toggle and seek-bar behavior. | `cargo run --example toggle_seekbar` |
| `z_order_hierarchy` | `examples/z_order_hierarchy.rs` | Checks visual ordering and hierarchy stacking behavior. | `cargo run --example z_order_hierarchy` |

## Grid Layout

| Example | Source | Purpose | Command |
| --- | --- | --- | --- |
| `grid_columns` | `examples/grid/columns.rs` | Grid column sizing and layout behavior. | `cargo run --example grid_columns` |
| `grid_tracks` | `examples/grid/tracks.rs` | Grid track sizing behavior. | `cargo run --example grid_tracks` |
| `grid_auto_flow` | `examples/grid/auto_flow.rs` | Grid auto-flow placement behavior. | `cargo run --example grid_auto_flow` |
| `grid_item_placement` | `examples/grid/item_placement.rs` | Explicit grid item placement. | `cargo run --example grid_item_placement` |

## Layout Modes

| Example | Source | Purpose | Command |
| --- | --- | --- | --- |
| `layout_flex` | `examples/layout/flex.rs` | Flex layout composition. | `cargo run --example layout_flex` |
| `layout_masonry` | `examples/layout/masonry.rs` | Masonry layout composition. | `cargo run --example layout_masonry` |
| `layout_stack` | `examples/layout/stack.rs` | Stack layout composition. | `cargo run --example layout_stack` |
| `layout_radial` | `examples/layout/radial.rs` | Radial layout composition. | `cargo run --example layout_radial` |

## Widgets

| Example | Source | Purpose | Command |
| --- | --- | --- | --- |
| `widgets_controls` | `examples/widgets/controls.rs` | Buttons, toggles, checkboxes, and radio controls. | `cargo run --example widgets_controls` |
| `widgets_inputs` | `examples/widgets/inputs.rs` | Text field, select, drag-value, and seek-bar inputs. | `cargo run --example widgets_inputs` |
| `widgets_display` | `examples/widgets/display.rs` | Text, badges, dividers, panels, and progress display. | `cargo run --example widgets_display` |
| `widgets_containers` | `examples/widgets/containers.rs` | Panel window and scroll-container behavior. | `cargo run --example widgets_containers` |

## Validation

Check every workspace example currently known to Cargo:

```bash
cargo check --workspace --examples
```

Check the Android package:

```bash
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
```

## Related Pages

- [Example Gallery](gallery.md)
- [Android Phone UI](android-phone.md)
- [Testing and Validation](../development/testing.md)
