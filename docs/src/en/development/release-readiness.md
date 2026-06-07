# Release Readiness

This page is the final pre-release checklist for the next alpha cut.

Current local `0.3.0` prep evidence is recorded at the repository root in `RELEASE_PREP_0.3.0.md`.

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

## Current Local Checklist

- [x] `mdbook build docs`
- [x] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets` through `./scripts/check_representative_examples.sh`
- [x] `./scripts/check_representative_examples.sh`
- [x] `./scripts/verify_alpha_release.sh`
- [x] release notes and migration pages mention the same current example-availability story
- [ ] one manual visual pass over the Android package and relevant current examples from [Visual Validation](visual-validation.md)
- [ ] GitHub Actions confirmation for the same gates
- [ ] RC feedback triage and closure

## Wrapper Removal Decision

For the next alpha cut, the decision is:

- do not remove `UScreenRoot` or `UWorldRoot` automatically as part of the cut
- keep them as deprecated compatibility wrappers until another stabilization review is complete
- reconsider removal only after the next alpha cut has shipped and feedback is in

## Root-Level File Roles

- `README.md`: landing story and fastest entry point
- `README_AR.md`: Arabic landing story
- `MIGRATION.md`: upgrade path from older docs and assumptions
- `MIGRATION_AR.md`: Arabic migration path
- `RELEASE_NOTES.md`: current alpha release-scale summary
- `RELEASE_PREP_0.3.0.md`: local evidence for the next `0.3.0` cut
- `changelog.md`: chronological dated history

## Related Pages

- [Testing and Validation](testing.md)
- [Visual Validation](visual-validation.md)
- [Smoke Test Plan](smoke-test-plan.md)
