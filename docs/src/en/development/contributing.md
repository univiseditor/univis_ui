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

## Code Style Inside The Project

- favor small components and clear systems
- keep visual behavior driven by `UNode`, `UBorder`, and `UInteractionColors`
- avoid surprising side effects outside the intended schedule

## Suggested Branch Names

- `feat/<name>` for features
- `fix/<name>` for fixes
- `docs/<name>` for documentation work
