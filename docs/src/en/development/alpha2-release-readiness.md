# Alpha2 Release Readiness

This page is the final pre-release checklist for the next `alpha2` publish.

## Canonical Root Examples To Recheck

Compile-check these sequentially:

```bash
cargo check -p univis_ui_engine --example root_screen_hud
cargo check -p univis_ui_engine --example root_world_scale
cargo check -p univis_ui_engine --example root_fit_content
cargo check -p univis_ui_engine --example root_capsule_overlap
```

Manual runtime validation is still recommended for at least one pass from the same set.

## Docs, Migration, And Release Alignment

Confirm that these files describe the same current reality:

- `README.md`
- `README_AR.md`
- `MIGRATION.md`
- `MIGRATION_AR.md`
- `RELEASE_NOTES.md`
- `changelog.md`
- `ALPHA2_STATUS.md`
- `ALPHA2_STATUS_AR.md`

## Final Checklist

- [ ] `mdbook build docs`
- [ ] `cargo doc -p univis_ui_style --no-deps`
- [ ] `cargo doc -p univis_ui_engine --no-deps`
- [ ] `cargo doc -p univis_ui_interaction --no-deps`
- [ ] `cargo doc -p univis_ui_widgets --no-deps`
- [ ] `cargo doc -p univis_ui --no-deps`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui_engine`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui_widgets`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui_interaction`
- [ ] `./scripts/check_examples_serial_release.sh -p univis_ui`
- [ ] one manual visual pass over the representative examples from [Visual Validation](visual-validation.md)
- [ ] release notes and migration pages mention the same current public root story

## Wrapper Removal Decision

For the next alpha cut, the decision is:

- do not remove `UScreenRoot` or `UWorldRoot` immediately after the cut
- keep them as deprecated compatibility wrappers through one more stabilization review
- reconsider removal only after the next alpha cut has shipped and feedback is in

## Root-Level File Roles

- `README.md`: landing story and fastest entry point
- `README_AR.md`: Arabic landing story
- `MIGRATION.md`: upgrade path from older docs, examples, and assumptions
- `MIGRATION_AR.md`: Arabic migration path
- `RELEASE_NOTES.md`: current alpha release-scale summary
- `changelog.md`: chronological dated history

## Related Pages

- [Testing and Validation](testing.md)
- [Visual Validation](visual-validation.md)
- [Smoke Test Plan](smoke-test-plan.md)
