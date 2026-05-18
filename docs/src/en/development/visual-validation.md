# Visual Validation

This page defines the lightweight manual visual checks that still matter even after docs and API docs pass.

## Purpose

- catch regressions that compile cleanly but look wrong
- keep the current live demo visually trustworthy
- keep runtime visual checks focused on examples and packages that exist in this branch

## Current Policy

- screenshots remain manual release-prep artifacts, not a required CI output
- the example gallery lists only current runnable sources
- runtime visual checks still matter for layout density, text appearance, widget affordances, and overall polish

## Live Validation Target

Run the current live package when the affected area is visual or interaction-heavy:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## What To Look For

### Android Phone Package

- the centered app surface remains balanced in a narrow viewport
- `UTextField`, `UToggle`, `USeekBar`, and `UButton` remain visually legible and easy to target
- spacing and density still read well without a fake hardware frame

## Release-Prep Expectation

- do at least one manual visual pass when the change affects rendering, layout semantics, or flagship UI presentation
- capture screenshots only when they materially help release communication or gallery quality
- do not block everyday contributor flow on screenshot generation

## Related Pages

- [Smoke Test Plan](smoke-test-plan.md)
- [Release Readiness](release-readiness.md)
- [Example Gallery](../examples/gallery.md)
