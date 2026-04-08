## Summary

- What changed:

## Validation

- [ ] `./scripts/check_quality.sh` (Required before merge)
- [ ] `./scripts/check_representative_examples.sh` (Required before merge)
- [ ] `mdbook build docs` when docs, root copy, or example docs changed
- [ ] `cargo doc --no-deps -p <affected-crate>` when public API changed
- [ ] package-aware example validation if examples or user-facing behavior changed
- [ ] manual visual validation note added if rendering/layout/interaction semantics changed

Pull requests should not merge with required validation left unchecked.

## Docs And Examples Checklist

- [ ] English and Arabic docs were updated together when needed
- [ ] New pages were added to `docs/src/SUMMARY.md`
- [ ] New examples live in the owning crate
- [ ] `README` / `README_AR` were updated when the landing story changed
- [ ] `RELEASE_NOTES.md` / `MIGRATION.md` were updated when the root release story changed
- [ ] `changelog.md` was updated when the change is notable

## Engineering Quality Checklist

- [ ] `./scripts/check_public_api_surface.sh` completed when facade imports, plugin wiring, or deprecated path exposure changed
- [ ] the change did not add avoidable coupling or widen public API without a clear reason
- [ ] the change did not introduce a new source hotspot without a written justification
- [ ] naming stayed consistent with maintainer docs and existing module boundaries
