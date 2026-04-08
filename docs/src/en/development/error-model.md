# Error Model And Diagnostics

The workspace favors typed, recoverable diagnostics over silent failure or opaque `Result<_, ()>` paths.

## Core Rules

- recoverable runtime problems should prefer fallback behavior plus a structured warning
- typed error or issue enums should expose at least a `reason` and, when useful, an `action`
- validation-only mismatches should stay behind rollout or shadow-validation modes
- panic-oriented checks belong in tests, examples, or true invariants, not hot runtime paths

## Warning Shape

Most runtime warnings follow this structure:

- subsystem tag such as `[layout/root_resolution]` or `[widget/text_label_measure]`
- entity or generation context when available
- `reason=...`
- `action=...` when the caller can do something concrete

## Current Canonical Examples

- root resolution uses `RootResolutionIssue` to distinguish missing, ambiguous, or inactive cameras
- text measurement uses `TextMeasureErrorKind` and `TextMeasureError` when Bevy text measurement cannot be created
- select runtime uses `SelectRuntimeError` when the generated child tree is incomplete
- settlement loop and picking validation log generation-aware warnings when rollout checks fail or the bounded loop exhausts its budget

## When To Add A New Error Type

- add a typed enum when multiple failure reasons share one call site
- add a structured warning when the system can continue but maintainers need a breadcrumb
- keep the type local if it only supports one subsystem
- promote the type only when another module genuinely needs to branch on it
