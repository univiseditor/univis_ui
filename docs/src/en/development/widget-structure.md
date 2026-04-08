# Widget Structure Conventions

This page defines how stateful widgets should be organized inside `crates/univis_ui_widgets/src/widget/`.

## Public Surface

- each widget exposes one public authored component such as `USelect`, `UTextField`, or `UPanel`
- each widget exposes one plugin named `Univis<Name>Plugin`
- outward-facing messages live in `events` when the widget emits them

## Internal Split

Use these responsibilities when a widget grows beyond a small single-file implementation:

- `model`: authored data, normalization helpers, and public configuration types
- `runtime`: generated child-tree markers, runtime resources, and setup helpers
- `interaction`: input handling and state transitions
- `visuals`: syncing colors, labels, geometry, and other rendered output
- `events`: emitted messages after state settles

## Schedule Rules

- `Build`: spawn or repair runtime child trees
- `Logic`: respond to input and mutate widget state
- `Visual`: synchronize the live visual tree from the latest state
- `Events`: emit outward-facing messages after logic and visuals settle

## Typical Patterns

- `select/` uses `model`, `runtime`, `interaction`, `visuals`, and `events`
- `panel/` splits resize math, cursor logic, runtime resize state, and visuals
- `text_label/` uses `measure/` and `render/` because text is both a layout and rendering subsystem

## Visibility Rules

- public authored components stay public and documented
- runtime markers and bookkeeping types should usually remain `pub(super)`
- tests should sit next to the widget they validate so structural refactors stay local
