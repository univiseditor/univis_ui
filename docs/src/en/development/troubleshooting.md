# Troubleshooting

## 1) Elements Do Not Appear

Check:

- that a camera exists
- that a root entity exists
- that `ComputedSize` is not zero
- that `Visibility` is not `Hidden`

## 2) Interaction Does Not Work

Check:

- that `UInteraction` exists on the interactive entity
- that no unintended child or parent is intercepting picking
- that no ancestor clip removes the hit

## 3) Text Escapes A Clipped Container

- confirm `UClip { enabled: true }` is present on the correct ancestor
- confirm `sync_text_clip_visibility` is active through `UnivisTextPlugin`

## 4) Scrolling Does Not Work

- the container must carry `UScrollContainer` and `UInteraction`
- the mouse must actually hover the container
- the content must overflow the visible area

## 5) Panel Resize Does Not Work

- make sure `UPanelWindow` is present with `UPanel`
- inspect `min_width`, `min_height`, and `border_hit_thickness`
- make sure the UI is reachable from the currently resolved root camera

## Related References

- [Current Limitations](current-limitations.md)
