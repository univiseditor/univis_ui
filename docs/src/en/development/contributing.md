# Contributing

## Practical Rules Before Any Change

1. Understand system order before editing layout, rendering, or interaction.
2. Do not break reflection on public types without a clear reason.
3. Validate changes against the current live package or equivalent source-level checks, not only unit tests.
4. When adding a new widget, ship:
   - a clear component API
   - a dedicated plugin
   - messages when needed
   - a runnable demo when the branch currently carries example sources
   - documentation in the README and this book

## Docs and Example Placement

- Add new guide pages in both `docs/src/en` and `docs/src/ar`.
- Put a new runnable example inside the crate that owns the feature.
- Keep root `examples/` only for facade-level integrated demos when such demos are present in the branch.
- Follow the conventions in [Editorial Rules](editorial-rules.md).
- Follow [Naming Conventions](naming-conventions.md) when splitting or renaming internal modules.
- Use [Maintainer Map](maintainer-map.md) and [Architecture Rules](architecture-rules.md) before moving ownership boundaries.
- Use [Widget Structure Conventions](widget-structure.md) and [Layout Engine Boundaries](layout-engine-boundaries.md) before splitting large subsystems.
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
