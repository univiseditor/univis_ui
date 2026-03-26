# Roadmap: Alpha2 Stabilization, Docs Publishing, and Example Gallery

This roadmap covers the next focused wave after the `URootUi` migration, docs unification, example redistribution, and API-doc cleanup.

## Goals

- [ ] Stabilize the `alpha2` line around the current `URootUi` model.
- [ ] Turn the docs flow into a published, easy-to-share experience.
- [ ] Turn the example surface into a clearer gallery instead of a raw list of runnable files.
- [ ] Make release communication simpler through one consistent docs, migration, and release-notes path.

## Non-Goals

- [ ] Do not redesign the root model again unless a real blocker appears.
- [ ] Do not introduce a large new widget family during the stabilization wave.
- [ ] Do not expand scope into a full visual editor or asset pipeline in this roadmap.

## Phase 1: Alpha2 Surface Freeze

- [x] Review the public `alpha2` surface and mark what is intended to remain stable through the rest of the alpha line.
- [x] List all remaining deprecated compatibility wrappers and decide whether each one stays for `alpha2` or gets removed before the next cut.
- [x] Audit the examples for places where they still rely on compatibility behavior rather than the intended `URootUi` path.
- [x] Tighten docs language around what is stable, what is transitional, and what is still experimental.
- [x] Write one short `alpha2` stability note for users who only read the repository root files.

## Phase 2: Docs Publishing

- [x] Publish the unified `docs/` book through GitHub Pages or an equivalent static hosting path.
- [x] Add a clear public docs URL to `README.md`, `README_AR.md`, and `RELEASE_NOTES.md`.
- [x] Add a lightweight docs publishing section that explains how the hosted book is produced from `docs/`.
- [x] Verify that the English and Arabic trees both render correctly in the hosted build.
- [x] Keep the local `mdbook build docs` path as the canonical contributor workflow.

## Phase 3: Example Gallery

- [x] Add a gallery-style docs page that groups flagship examples by purpose:
  - HUD and screen UI
  - world-space UI
  - widgets and forms
  - interaction
  - showcase demos
- [x] Add one-sentence “why you would run this” summaries for every flagship example.
- [x] Add screenshots or visual references for the most important examples where practical.
- [x] Distinguish learning examples from showcase examples more strongly than the current index pages do.
- [x] Add a small “best first examples” list for new users.

## Phase 4: Release Communication Cleanup

- [x] Decide on the long-term root-level release file set:
  - `README.md`
  - `README_AR.md`
  - `MIGRATION.md`
  - `MIGRATION_AR.md`
  - `RELEASE_NOTES.md`
  - `changelog.md`
- [x] Make sure each file has a clearly different purpose and no repeated low-value content.
- [x] Add a short “where to read what” section to the root release/migration files.
- [x] Refresh `RELEASE_NOTES.md` for the next alpha cut once the stabilization work is done.

## Phase 5: CI And Validation Hardening

- [x] Keep docs, API docs, and example checks green while reducing avoidable duplication.
- [x] Add one lightweight visual-validation checklist for the most representative examples.
- [x] Decide whether example screenshots should become part of release prep or remain manual.
- [x] Review the docs/examples workflow output so failures are easy to understand from CI logs.

## Phase 6: Alpha2 Release Readiness

- [x] Recheck the root examples that define the intended mental model:
  - `root_screen_hud`
  - `root_world_scale`
  - `root_fit_content`
  - `root_capsule_overlap`
- [x] Confirm that docs, migration, and release notes all describe the same current reality.
- [x] Cut a final pre-release checklist for the next `alpha2` publish.
- [x] Decide whether any deprecated wrapper should be removed immediately after the next alpha cut.

## Done Criteria

- [ ] A new user can go from `README` to hosted docs to the right example without guessing.
- [ ] The repository root files feel intentional rather than historical.
- [ ] The example surface reads like a curated product story, not just a file inventory.
- [ ] The next alpha release can be explained with fewer ad hoc notes and fewer scattered links.
