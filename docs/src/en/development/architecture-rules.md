# Architecture Rules

These rules define which APIs are considered stable, which ones are internal, and when the workspace should grow new crates or folders.

## Public API

- `prelude` is the recommended stable import story
- named modules such as `layout`, `render`, `interaction`, and `widget` are advanced but public
- public authored components keep their declaration-site `rustdoc`
- compatibility wrappers stay on explicit paths and do not belong in recommended preludes

## Internal API

- `internal` and `internal_prelude` are workspace implementation tools, not application-facing contracts
- `layout::core`, `layout::algorithms`, `layout::components`, `layout::pipeline`, and similar hidden modules may change to support engine refactors
- runtime marker components, caches, and generated child-tree bookkeeping should stay crate-private or `pub(super)` unless another crate truly needs them
- the immediate workspace boundary policy is tracked in the root file `ENGINE_BOUNDARY_RULES.md`
- `./scripts/check_engine_boundary_guardrails.sh` prevents new external imports from engine internals while current legacy exceptions are being removed

## When To Add A Crate

Add a new crate only when at least one of these is true:

1. the code introduces a reusable dependency boundary that applications may choose independently
2. the feature needs a separate public import story or plugin lifecycle
3. the dependency graph becomes cleaner by isolating a subsystem

Do not add a crate just to split files. Prefer folders and modules first.

## When To Add A Folder Or Module

- add a folder when one responsibility has multiple collaborating files with clear roles
- add a module when it makes ownership or discovery more explicit than one long file
- prefer responsibility names such as `roots`, `state`, `runtime`, `events`, or `placement`
- avoid generic buckets when a domain-specific name is available

## Dependency Rules

- widgets depend on engine, interaction, and style; engine must not depend on widgets
- interaction depends on engine state for roots and geometry; engine must not depend on interaction
- facade crates re-export curated public surfaces but should not leak internal helpers accidentally
