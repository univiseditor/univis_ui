# Visual Validation

This page defines the lightweight manual visual checks that still matter even after docs, API docs, and example compile checks pass.

## Purpose

- catch regressions that compile cleanly but look wrong
- keep the most representative examples visually trustworthy
- avoid turning screenshots into a hard CI gate before the project is ready for that cost

## Current Policy

- screenshots remain manual release-prep artifacts, not a required CI output
- the example gallery may link to static references where useful
- runtime visual checks still matter for HUD behavior, world scaling, text appearance, and root overlap rules

## Representative Examples

Run these examples when the affected area is visual or interaction-heavy:

```bash
cargo run -p univis_ui --example root_screen_hud
cargo run -p univis_ui --example root_world_scale
cargo run -p univis_ui --example root_fit_content
cargo run -p univis_ui --example root_capsule_overlap
cargo run -p univis_ui_widgets --example text_label
cargo run -p univis_ui_widgets --example panel_window
cargo run -p univis_ui --example interaction
```

## What To Look For

### HUD And Roots

- `root_screen_hud`: the screen HUD stays fixed while the world root moves with the camera
- `root_world_scale`: screen HUD remains fixed while world panels differ in physical scale
- `root_fit_content`: world roots wrap measured content without obvious clipping or runaway growth
- `root_capsule_overlap`: protruding children do not visually or interactively escape above another root

### Widgets And Interaction

- `text_label`: clipping, overflow, and autosize remain legible and intentional
- `panel_window`: resize handles and panel movement remain intuitive
- `interaction`: blocking, ignore, and passthrough behavior still match the documented picking model

## Release-Prep Expectation

- do at least one manual visual pass when the change affects rendering, layout semantics, picking, or flagship examples
- capture screenshots only when they materially help release communication or gallery quality
- do not block everyday contributor flow on screenshot generation

## Related Pages

- [Smoke Test Plan](smoke-test-plan.md)
- [Release Readiness](release-readiness.md)
- [Example Gallery](../examples/gallery.md)
