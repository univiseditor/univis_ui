# Troubleshooting

## 1) Elements Do Not Appear

Check:

- that a compatible camera exists
- that a root entity built from `URootUi` exists
- that `ComputedSize` is not zero
- that `Visibility` is not `Hidden`

## 2) Interaction Does Not Work

Check:

- that `UInteraction` exists on the interactive entity
- that no unintended child or parent is intercepting picking
- that no ancestor clip removes the hit

## 3) Text Escapes A Clipped Container

- confirm `UClip { enabled: true }` is present on the correct ancestor
- confirm `UnivisTextPlugin` is active so `sync_text_label_meshes` and `sync_text_clipper_materials` can apply local and ancestor clipping

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
- [Error Model And Diagnostics](error-model.md)
