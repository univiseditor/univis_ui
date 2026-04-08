# Naming Conventions

This page defines the internal naming rules that keep the workspace predictable for contributors.

## Goals

- make it easy to guess where code should live before opening files
- keep plugin, set, state, and component names aligned across crates
- reserve compatibility wording for real compatibility behavior only

## Module Names

- use `model` for the authored data model or pure helpers that validate and normalize it
- use `runtime` for ECS-only runtime markers, spawned child-entity bookkeeping, and systems that own the live widget tree
- use `state` for long-lived mutable engine or widget state that multiple systems read and update
- use `events` for emitted messages and the code that translates state transitions into outward-facing notifications
- use `translation` for authored-to-runtime or authored-to-solver conversion logic
- use `placement` for child placement after sizes are known
- avoid vague buckets such as `types`, `helpers`, or `display` when a more specific domain name is available

## Type Names

- plugins should follow `Univis<Name>Plugin`
- schedule sets should follow `Univis<Domain>Set` or `Univis<Stage>Set`
- public authored UI components should keep the `U*` prefix
- internal runtime markers should use explicit nouns such as `SelectTrigger`, `PanelResizeHandle`, or `UiWorkState`
- resources that tune behavior should end in `Settings`
- resources or components that track live progress should end in `State`

## Comments

- keep implementation comments in English
- prefer comments that explain constraints, invariants, or why a branch exists
- remove comments that only restate what the next line already says

## Legacy And Compatibility Names

- use `legacy_*` only when code preserves old semantics on purpose
- keep deprecated compatibility wrappers explicit in naming and documentation
- avoid introducing new transitional aliases unless they protect a real public migration path

## Rename Checklist

Before creating or renaming a module, check:

1. does the filename describe responsibility rather than implementation detail?
2. would a new contributor know whether the code belongs in `model`, `runtime`, `state`, `events`, `translation`, or `placement`?
3. does the new name match existing plugin, component, and state patterns in sibling modules?
