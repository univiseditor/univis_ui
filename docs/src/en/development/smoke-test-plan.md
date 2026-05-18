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
2. Workspace example sanity for affected layout or widget areas
3. Text and control readability pass

## Commands

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

For workspace examples, use `cargo run --example <name>` with an entry from [Examples](../examples/index.md).

## Pass Criteria

- No startup panics.
- The Android-style package opens and keeps the centered app surface readable in a narrow viewport.
- `UTextField`, `UToggle`, `USeekBar`, and `UButton` remain visually coherent and interactive.
- Relevant workspace examples still open and remain visually coherent.

## Failure Triage

1. Capture the failing surface and the symptom.
2. Re-run the Android package with `RUST_BACKTRACE=1` if the failure is runtime-related.
3. Classify as compile/runtime/widget/rendering regression.
4. Add an issue note with the repro command and environment details.

See also: [Visual Validation](visual-validation.md) and [Release Readiness](release-readiness.md).
