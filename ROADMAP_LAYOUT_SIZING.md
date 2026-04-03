# Roadmap: Layout Sizing Semantics (`min/max`, Intrinsic Sizes, and Content Modes)

This roadmap covers the next focused layout wave after the recent `UTextLabel` overflow and clipping fixes.

The main goal is to make size negotiation clearer and more predictable, especially for flex and wrap layouts, while moving the engine closer to familiar web-style sizing semantics.

## Goals

- [x] Expose user-facing `min_width`, `max_width`, `min_height`, and `max_height` controls on layout nodes.
- [x] Make flex and wrap behavior safer for fixed-size and content-sized items without changing the default flex model.
- [x] Add explicit intrinsic-size concepts such as `min-content` and `max-content` instead of overloading `Content` and `Auto`.
- [x] Clarify and document the intended sizing semantics so examples and widgets follow one consistent model.

## Non-Goals

- [ ] Do not change the default `flex_shrink` behavior from `1.0` to `0.0`.
- [ ] Do not silently turn clipping on by default as part of this roadmap.
- [ ] Do not rewrite the entire solver or replace the current layout model wholesale.
- [ ] Do not introduce CSS parity as a hard requirement for every edge case in this wave.

## Phase 1: Semantics Freeze

- [x] Freeze the intended meaning of these concepts before changing the API:
  - preferred size
  - minimum size
  - maximum size
  - content size
  - auto size
  - `min-content`
  - `max-content`
- [x] Decide whether `UVal::Content` keeps its current meaning or becomes an alias for one of the new intrinsic modes.
- [x] Decide whether `UVal::Auto` should remain equivalent to current content-sizing in some contexts or become a distinct mode.
- [x] Write one short decision note for the sizing vocabulary so follow-up implementation does not drift.

## Phase 2: User-Facing Min/Max Constraints

- [x] Extend `UNode` with:
  - `min_width`
  - `max_width`
  - `min_height`
  - `max_height`
- [x] Define clear defaults:
  - `min_*` defaults to unconstrained zero
  - `max_*` defaults to unbounded
- [x] Thread these fields through the solver translation layer and internal constraints model.
- [x] Clamp solved sizes against explicit min/max constraints for normal layout flow.
- [ ] Recheck how these constraints interact with padding, margin, and border-radius-driven visuals.

## Phase 3: Flex And Wrap Constraint Enforcement

- [x] Make flex shrink/grow respect explicit min/max constraints during size redistribution.
- [ ] Verify that wrapped rows and columns break correctly when items cannot shrink below their minimum size.
- [x] Add targeted coverage for these cases:
  - fixed-width cards in wrapped flex containers
  - content-sized items with explicit `min_width`
  - percent-sized items with explicit `max_width`
  - autosized text labels inside constrained cards
- [x] Keep the default `flex_shrink = 1.0` behavior and rely on explicit constraints or item settings where appropriate.

## Phase 4: Intrinsic Size Modes

- [x] Add explicit intrinsic size modes for width and height:
  - `MinContent`
  - `MaxContent`
- [x] Define how these modes behave in flex, grid, and simple block-like layout.
- [x] Decide whether `Content` remains public, is deprecated, or maps internally to one of the intrinsic modes.
- [ ] Update text, image, and common widget measurement paths so intrinsic sizing behaves consistently across measured widgets.

## Phase 5: `Auto` vs `Content`

- [x] Separate `Auto` from `Content` where the current implementation still maps them to the same solver mode.
- [x] Document where `Auto` means:
  - fill available space
  - defer to layout context
  - behave like intrinsic sizing
- [x] Remove accidental equivalence where it causes confusing behavior in examples or widgets.
- [x] Keep backward compatibility notes for any public semantic change that could affect existing layouts.

## Phase 6: Examples, Docs, And Validation

- [x] Add one focused engine example for min/max constraint behavior.
- [ ] Add one widget-level example that demonstrates:
  - fixed cards with `flex_shrink`
  - explicit `min_width`
  - text with `min-content` / `max-content`
  - wrap breakpoints
- [x] Update docs in English and Arabic to explain the new sizing model.
- [x] Add regression tests for solver behavior and representative widget integrations.
- [ ] Add migration notes if any public defaults or meanings change.

## Done Criteria

- [x] A fixed-width item in a wrapped flex container can opt out of shrink in an explicit, documented way.
- [x] A user can express minimum and maximum node sizes without relying on solver internals.
- [x] Intrinsic sizing no longer depends on vague overlap between `Auto` and `Content`.
- [x] The docs explain sizing decisions clearly enough that example authors stop rediscovering the same traps.
