# Visual Validation

This page defines the lightweight manual visual checks that still matter even after docs and API docs pass.

## Purpose

- catch regressions that compile cleanly but look wrong
- keep the current live demo visually trustworthy
- use static references where the historical runtime examples are no longer shipped

## Current Policy

- screenshots remain manual release-prep artifacts, not a required CI output
- the example gallery may link to static references where useful
- runtime visual checks still matter for layout density, text appearance, widget affordances, and overall polish

## Live Validation Target

Run the current live package when the affected area is visual or interaction-heavy:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

## Static References

Open these when you want quick comparison material without starting Bevy:

- [Responsive Dashboard HTML](../../assets/visual-references/responsive_dashboard.html)
- [Complex Dashboard HTML](../../assets/visual-references/complex_dashboard.html)
- [Layout Solver No Widgets HTML](../../assets/visual-references/layout_solver_no_widgets.html)
- [Layout Solver Ultra Complex HTML](../../assets/visual-references/layout_solver_ultra_complex.html)
- [Card Profile Screenshot](../../assets/profile.png)

## What To Look For

### Android Phone Package

- the centered app surface remains balanced in a narrow viewport
- `UTextField`, `UToggle`, `USeekBar`, and `UButton` remain visually legible and easy to target
- spacing and density still read well without a fake hardware frame

### Static References

- dashboard hierarchy still looks intentional and readable
- layout-heavy scenes still resemble the expected solver output
- polished showcase surfaces still match the archived visual language closely enough for release notes

## Release-Prep Expectation

- do at least one manual visual pass when the change affects rendering, layout semantics, or flagship UI presentation
- capture screenshots only when they materially help release communication or gallery quality
- do not block everyday contributor flow on screenshot generation

## Related Pages

- [Smoke Test Plan](smoke-test-plan.md)
- [Release Readiness](release-readiness.md)
- [Example Gallery](../examples/gallery.md)
