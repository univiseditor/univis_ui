# Smoke Test Plan

## Goal

Provide a lightweight manual runtime checklist after passing compile validation.

## Pre-check

Run compile validation first:

```bash
./scripts/check_representative_examples.sh
./scripts/verify_serial_release.sh
```

## Manual Runtime Scenarios

1. Android-style narrow-screen package sanity
2. Visual comparison against archived static references
3. Text and control readability pass

## Commands

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

Open as needed for quick comparison:

- `docs/src/assets/visual-references/responsive_dashboard.html`
- `docs/src/assets/visual-references/complex_dashboard.html`
- `docs/src/assets/visual-references/layout_solver_no_widgets.html`
- `docs/src/assets/visual-references/layout_solver_ultra_complex.html`

## Pass Criteria

- No startup panics.
- The Android-style package opens and keeps the centered app surface readable in a narrow viewport.
- `UTextField`, `UToggle`, `USeekBar`, and `UButton` remain visually coherent and interactive.
- Static references still look close enough to the intended design language for release communication.

## Failure Triage

1. Capture the failing surface and the symptom.
2. Re-run the Android package with `RUST_BACKTRACE=1` if the failure is runtime-related.
3. Classify as compile/runtime/widget/rendering regression.
4. Add an issue note with the repro command and environment details.

See also: [Visual Validation](visual-validation.md) and [Release Readiness](release-readiness.md).
