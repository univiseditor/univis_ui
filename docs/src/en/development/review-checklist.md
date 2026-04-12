# Docs Review Checklist

Use this checklist for docs-heavy pull requests.

- [ ] `./scripts/check_quality.sh` completed before merge
- [ ] `./scripts/check_representative_examples.sh` completed before merge
- [ ] `mdbook build docs` completed when docs, examples, or root copy changed
- [ ] English and Arabic pages were updated together when the change is user-facing
- [ ] the page was added to `docs/src/SUMMARY.md`
- [ ] the example lives in the owning crate
- [ ] the examples index was updated when a new example was added
- [ ] canonical examples link back to the related guide when useful
- [ ] new public API has `rustdoc` at the declaration site
- [ ] internal-only helpers were not promoted accidentally in generated docs
- [ ] commands in docs use package-aware example invocations where needed
- [ ] visual validation notes were updated when representative examples or rendering behavior changed
- [ ] `README` / `README_AR` were updated if the landing story changed
- [ ] `changelog.md` was updated if the change is notable

## Engineering Quality

- [ ] `./scripts/check_public_api_surface.sh` completed when facade imports, plugin wiring, or deprecated path exposure changed
- [ ] `./scripts/check_engine_boundary_guardrails.sh` completed when moving logic across `engine`, `interaction`, or `widgets`
- [ ] the change did not introduce a new large source hotspot without a clear reason
- [ ] the change did not widen public API or coupling without a user-facing need
- [ ] the change did not add a new dependency on `univis_ui_engine::internal` or hidden engine modules outside the engine crate
- [ ] naming stayed consistent with the maintainer naming and architecture pages
