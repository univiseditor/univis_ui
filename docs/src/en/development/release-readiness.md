# Release Readiness

This page is the final pre-release checklist for the next alpha cut.

## Compile Checks To Re-run

Compile-check these sequentially:

```bash
cargo check --workspace --all-targets
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
./scripts/check_representative_examples.sh
```

Manual runtime validation is still recommended for at least one pass through the current live package.

## Docs, Migration, And Release Alignment

Confirm that these files describe the same current reality:

- `README.md`
- `README_AR.md`
- `MIGRATION.md`
- `MIGRATION_AR.md`
- `RELEASE_NOTES.md`
- `changelog.md`

## Final Checklist

- [ ] `mdbook build docs`
- [ ] `cargo doc -p univis_ui_style --no-deps`
- [ ] `cargo doc -p univis_ui_engine --no-deps`
- [ ] `cargo doc -p univis_ui_interaction --no-deps`
- [ ] `cargo doc -p univis_ui_widgets --no-deps`
- [ ] `cargo doc -p univis_ui --no-deps`
- [ ] `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets`
- [ ] `./scripts/check_representative_examples.sh`
- [ ] one manual visual pass over the Android package and the archived static references from [Visual Validation](visual-validation.md)
- [ ] release notes and migration pages mention the same current example-availability story

## Wrapper Removal Decision

For the next alpha cut, the decision is:

- do not remove `UScreenRoot` or `UWorldRoot` automatically as part of the cut
- keep them as deprecated compatibility wrappers until another stabilization review is complete
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
