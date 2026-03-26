# Contributing

## Practical Rules Before Any Change

1. Understand system order before editing layout, rendering, or interaction.
2. Do not break reflection on public types without a clear reason.
3. Validate changes against real examples, not only unit tests.
4. When adding a new widget, ship:
   - a clear component API
   - a dedicated plugin
   - messages when needed
   - a demo example
   - documentation in the README and this book

## Docs and Example Placement

- Add new guide pages in both `docs/src/en` and `docs/src/ar`.
- Put a new example inside the crate that owns the feature.
- Keep root `examples/` only for facade-level integrated demos.
- Follow the conventions in [Editorial Rules](editorial-rules.md).
- Use [Docs Authoring Workflow](authoring.md) for the exact bilingual and `rustdoc` steps.
- Use [Docs Review Checklist](review-checklist.md) before merging docs-heavy work.

## Code Style Inside The Project

- favor small components and clear systems
- keep visual behavior driven by `UNode`, `UBorder`, and `UInteractionColors`
- avoid surprising side effects outside the intended schedule

## Suggested Branch Names

- `feat/<name>` for features
- `fix/<name>` for fixes
- `docs/<name>` for documentation work
