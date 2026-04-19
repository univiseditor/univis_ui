# Android App Screen

This page documents the current live `android_phone` demo package.

Run it with:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

The package also keeps an `android_phone` example target locally for Android-oriented packaging,
but the desktop-friendly entry point above is the fastest way to inspect the scene in this branch.

## What It Demonstrates

- a centered Android-style application surface without a fake hardware frame
- `UTextField` for top-level search input
- `UToggle` and `USeekBar` inside a minimal settings layout
- compact `UButton` actions in a bottom navigation row
- a narrow viewport composition built around `URootUi::screen()`

## When To Run It

- when you want the current live demo that still ships with the repository
- when you want to test spacing density for settings, launcher, or companion-app scenes
- when you want one scene that mixes layout, widgets, and interaction in a narrow viewport

## Practical Notes

- the scene uses `URootUi::screen()`, so the app surface stays viewport-fixed like a HUD
- the layout is intentionally static so core controls stay reachable on touch devices
- for historical example ownership, see [Examples](index.md)
- for focused widget behavior, compare it with [`text_field`](index.md#text_field), [`scroll_view`](index.md#scroll_view), and [`widgets`](index.md#widgets)

## Related Pages

- [Examples](index.md)
- [Example Gallery](gallery.md)
- [Plugin Setup and First Paths](../first-steps.md)
