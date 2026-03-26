# Smoke Test Plan

## Goal

Provide a lightweight manual runtime checklist after passing serial release compile checks.

## Pre-check

Run compile validation first:

```bash
./scripts/verify_serial_release.sh
```

## Manual Runtime Scenarios (Priority Order)

1. Basic startup + root rendering
2. Pointer interaction transitions
3. Scroll behavior
4. Text input behavior
5. Panel resize behavior
6. 3D visual path sanity

## Commands

```bash
cargo run --release -p univis_ui --example hello_world
cargo run --release -p univis_ui_interaction --example interaction
cargo run --release -p univis_ui_widgets --example scroll_view
cargo run --release -p univis_ui_widgets --example text_field
cargo run --release -p univis_ui_widgets --example panel_window
cargo run --release -p univis_ui_engine --features example_bloom --example border_light_3d
```

## Pass Criteria

- No startup panics.
- UI is visible and responsive.
- Expected interaction signals (hover/press/click) are observable.
- `text_field` accepts input and emits expected submit/change behavior.
- `panel_window` resize handles respond to pointer drag.
- `border_light_3d` renders expected 3D-lit visuals.

## Failure Triage

1. Capture example name and failure symptom.
2. Re-run single example with `RUST_BACKTRACE=1`.
3. Classify as compile/runtime/interaction/rendering regression.
4. Add issue note with repro command and environment details.
