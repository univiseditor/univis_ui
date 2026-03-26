## Summary

- What changed:

## Validation

- [ ] `mdbook build docs`
- [ ] `cargo doc --no-deps -p <affected-crate>`
- [ ] package-aware example validation if examples or user-facing behavior changed

## Docs And Examples Checklist

- [ ] English and Arabic docs were updated together when needed
- [ ] New pages were added to `docs/src/SUMMARY.md`
- [ ] New examples live in the owning crate
- [ ] `README` / `README_AR` were updated when the landing story changed
- [ ] `changelog.md` was updated when the change is notable
