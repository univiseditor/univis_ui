# Docs Review Checklist

Use this checklist for docs-heavy pull requests.

- [ ] English and Arabic pages were updated together when the change is user-facing
- [ ] the page was added to `docs/src/SUMMARY.md`
- [ ] the example lives in the owning crate
- [ ] the examples index was updated when a new example was added
- [ ] canonical examples link back to the related guide when useful
- [ ] new public API has `rustdoc` at the declaration site
- [ ] internal-only helpers were not promoted accidentally in generated docs
- [ ] commands in docs use package-aware example invocations where needed
- [ ] `README` / `README_AR` were updated if the landing story changed
- [ ] `changelog.md` was updated if the change is notable
