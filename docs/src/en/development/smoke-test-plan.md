# Smoke Test Plan

## Goal

Provide a lightweight manual runtime checklist after passing serial release compile checks.

## Pre-check

Run compile validation first:

```bash
./scripts/verify_serial_release.sh
```

## Manual Runtime Scenarios (Priority Order)

1. Screen HUD and world-root separation
2. World scaling and fit-content root behavior
3. Root capsule overlap behavior
4. Pointer interaction transitions
5. Text input and text rendering behavior
6. Panel resize behavior
7. 3D visual path sanity

## Commands

```bash
cargo run --release -p univis_ui_engine --example root_screen_hud
cargo run --release -p univis_ui_engine --example root_world_scale
cargo run --release -p univis_ui_engine --example root_fit_content
cargo run --release -p univis_ui_engine --example root_capsule_overlap
cargo run --release -p univis_ui_interaction --example interaction
cargo run --release -p univis_ui_widgets --example text_field
cargo run --release -p univis_ui_widgets --example text_label
cargo run --release -p univis_ui_widgets --example panel_window
cargo run --release -p univis_ui_engine --features example_bloom --example border_light_3d
```

## Pass Criteria

- No startup panics.
- `root_screen_hud` keeps HUD content fixed while the world root follows camera motion.
- `root_world_scale` keeps logical sizing separate from physical world scale.
- `root_fit_content` wraps measured content without obvious clipping or runaway growth.
- `root_capsule_overlap` preserves closed root stacking.
- Expected interaction signals (hover/press/click) are observable.
- `text_field` accepts input and emits expected submit/change behavior.
- `text_label` remains legible under clipping, overflow, and autosize cases.
- `panel_window` resize handles respond to pointer drag.
- `border_light_3d` renders expected 3D-lit visuals.

## Failure Triage

1. Capture example name and failure symptom.
2. Re-run single example with `RUST_BACKTRACE=1`.
3. Classify as compile/runtime/interaction/rendering regression.
4. Add issue note with repro command and environment details.

See also: [Visual Validation](visual-validation.md) and [Alpha2 Release Readiness](alpha2-release-readiness.md).
