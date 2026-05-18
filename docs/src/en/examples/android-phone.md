# Android App Screen

This page documents the current live `android_phone` demo package.

Run it with:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

The package also keeps an `android_phone` example target locally for Android-oriented packaging,
but the desktop-friendly entry point above is the fastest way to inspect the scene in this branch.

## What It Demonstrates

- a full-height Android-style app surface capped to a narrow mobile width
- a top summary card plus a live `UTextField` search input
- a scrollable feed using `UClip`, `UScrollContainer`, and `UInteraction`
- compact mobile controls built from `UToggle`, `USeekBar`, `UBadge`, and `UButton`
- a bottom navigation row that stays visible while the middle feed grows

## When To Run It

- when you want a concrete mobile layout reference instead of isolated widget demos
- when you want to verify spacing density for phone-sized screens
- when you want one scene that mixes layout, scrolling, and touch-friendly controls

## Practical Notes

- the scene uses `URootUi::screen()`, so the app surface stays viewport-fixed like a HUD
- the outer shell uses `max_width` instead of a fake device frame, so the same scene reads well on desktop and Android
- the center feed is the part meant to scroll; the top summary and bottom navigation remain visible
- for focused widget behavior, compare it with [`widgets_controls`](index.md#widgets), [`widgets_inputs`](index.md#widgets), and [`widgets_containers`](index.md#widgets)

## Related Pages

- [Examples](index.md)
- [Example Gallery](gallery.md)
- [Plugin Setup and First Paths](../first-steps.md)
