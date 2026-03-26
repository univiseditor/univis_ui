## Summary

- What changed:

## Validation

- [ ] `mdbook build docs`
- [ ] `cargo doc --no-deps -p <affected-crate>`
- [ ] package-aware example validation if examples or user-facing behavior changed
- [ ] manual visual validation note added if rendering/layout/interaction semantics changed

## Docs And Examples Checklist

- [ ] English and Arabic docs were updated together when needed
- [ ] New pages were added to `docs/src/SUMMARY.md`
- [ ] New examples live in the owning crate
- [ ] `README` / `README_AR` were updated when the landing story changed
- [ ] `RELEASE_NOTES.md` / `MIGRATION.md` were updated when the root release story changed
- [ ] `changelog.md` was updated when the change is notable
