# Interaction System

The interaction layer is built on:

- a custom picking backend: `univis_picking_backend`
- observer callbacks for pointer events
- component state through `UInteraction`

## Core Components

- `UInteraction`
  - stores the current interaction state
- `UInteractionColors`
  - maps state changes to background colors

## How It Works

1. the backend computes hits
2. Bevy picking emits pointer events
3. observers in `interaction/feedback.rs` update `UInteraction` and visuals

## Note

Interaction depends on `UInteraction` being present on the target entity.

## Related References

- [Support Matrix](support-matrix.md)
- [Current Limitations](../development/current-limitations.md)

## Related Examples

- [`interaction`](../examples/index.md#interaction) in `univis_ui_interaction`
- [`root_capsule_overlap`](../examples/index.md#root_capsule_overlap) in `univis_ui`

## API Entry Points

- `univis_ui_interaction::interaction::feedback::UInteraction`
- `univis_ui_interaction::interaction::feedback::UInteractionColors`
- `univis_ui_interaction::interaction::picking::univis_picking_backend`

## Where To Look Next

- related example: [`interaction`](../examples/index.md#interaction)
- related API index: [API Reference](../api/index.md)
- related migration page: [Current Limitations](../development/current-limitations.md)
