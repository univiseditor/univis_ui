# Compatibility Matrix

Validation baseline date: March 6, 2026.

Legend:
- `Yes`: supported and validated in current baseline.
- `Partial`: supported with known constraints.
- `No`: not supported in the current baseline.

| Capability | Screen UI | World UI (2D) | World UI (3D) | Validation Source |
|---|---|---|---|---|
| Release example compilation | Yes | Yes | Yes | `./scripts/check_examples_serial_release.sh` (28/28 pass) |
| Base rendering path | Yes | Yes | Yes | examples + render plugin setup |
| Pointer interaction | Yes | Yes | Partial | `univis_picking_backend` currently queries `Camera2d` |
| Clipping-aware picking | Yes | Yes | Partial | ancestor clipping checks in picking backend |
| `UPanelWindow` resize | Yes | Yes | Partial | resize path currently queries `Camera2d` |
| `UTextField` behavior/events | Yes | Yes | Partial | requires `UnivisTextFieldPlugin` |
| `UBadge` dynamic visual updates | Yes | Yes | Yes | requires `UnivisBadgePlugin` |
| `UScrollContainer` behavior | Yes | Yes | Partial | interaction path follows current picking limits |
| `UPbr` controls | No | No | Yes | intended for `UI3d` path |

## Related

- [Support Matrix (Screen/World/3D)](../interaction/support-matrix.md)
- [Current Limitations](current-limitations.md)
- [Example Validation Report](example-validation.md)
