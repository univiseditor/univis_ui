# Compatibility Matrix

Validation baseline date: April 3, 2026.

Legend:
- `Yes`: supported and validated in current baseline.
- `Partial`: supported with known constraints.
- `No`: not supported in the current baseline.
- `Deferred`: planned validation was intentionally skipped in this cycle.

| Capability | Screen UI | World UI (2D) | World UI (3D) | Validation Source |
|---|---|---|---|---|
| Manual runtime smoke (`cargo run --release -p <package> --example ...`) | Deferred | Deferred | Deferred | skipped on March 6, 2026 (resource constraints), see [Smoke Test Plan](smoke-test-plan.md) |
| Release example compilation | Yes | Yes | Yes | package-aware `./scripts/check_examples_serial_release.sh` validation |
| Base rendering path | Yes | Yes | Yes | examples + render plugin setup |
| Pointer interaction | Yes | Yes | Yes | camera resolves from each root through `ResolvedRootUi`; prefer `UiCameraRef::Entity` in multi-camera scenes |
| Clipping-aware picking | Yes | Yes | Yes | ancestor clipping checks in the picking backend |
| `UPanelWindow` resize | Yes | Yes | Yes | resize logic resolves cursor movement through the root camera and panel plane |
| `UTextField` behavior/events | Yes | Yes | Yes | included by default through `UnivisWidgetPlugin` |
| `UBadge` dynamic visual updates | Yes | Yes | Yes | included by default through `UnivisWidgetPlugin` |
| `UScrollContainer` behavior | Yes | Yes | Yes | interaction follows the resolved root-camera path |
| `UPbr` controls | No | No | Yes | intended for `UI3d` path |

## Related

- [Support Matrix (Screen/World/3D)](../interaction/support-matrix.md)
- [Current Limitations](current-limitations.md)
- [Example Validation Report](example-validation.md)
