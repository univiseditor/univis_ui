# Roadmap to Stable `0.3.0` (without moving to `1.0.0`)

This roadmap defines what must be completed before declaring the `0.2.x` line stable.

## Current Snapshot (2026-05-18)

- Latest documented public release: `0.3.0`.
- Target milestone: first stable contract cut for the `0.2.x` line (`0.3.0`).
- Latest local release-prep evidence: `RELEASE_PREP_0.3.0.md`.
- No roadmap item is considered complete until evidence is linked in release notes, changelog, and CI logs.

## Scope and Intent

- Keep the project on `0.x` while Bevy itself is pre-`1.0`.
- Ship a stable contract for the `0.2.x` line.
- Defer intentional breaking changes to the next breaking window (`0.3.0`).

## Versioning Policy (0.x)

- `0.2.x` is treated as a compatibility contract line for users.
- Any intentional breaking public API change moves to `0.3.0`.
- Backward-compatible fixes and improvements stay in `0.2.x` patch releases.
- Release communication must always state whether a change is `0.2.x`-safe or `0.3.0`-targeted.

## Release Stages

1. `0.3.0-rc.1`
2. `0.3.0-rc.2` (optional, if regressions are found)
3. `0.3.0`
4. `0.2.1+` patch line for non-breaking fixes

## Must-Have Before `0.3.0`

### 1. API Freeze For The `0.2` Line

- Freeze recommended public surfaces and preludes.
- Keep deprecated wrappers explicit-only during `0.2.x`.
- Document compatibility guarantees for `0.2.x` in release notes.

### 2. Documentation Consistency Sweep

- Ensure root docs (`README`, `MIGRATION`, `RELEASE_NOTES`, `changelog`) describe the same current reality.
- Align example availability wording with the actual repository tree.
- Keep Arabic/English release-level claims synchronized.

### 3. Release Quality Gates

- Require full quality scripts to pass in release CI.
- Keep public-surface and structure guardrails mandatory.
- Keep engine-boundary guardrails mandatory.

### 4. Visual Validation Policy

- Maintain a short manual visual checklist for rendering-heavy changes.
- Keep representative scenes/examples documented as release checks.
- Require explicit release notes for known visual caveats.

### 5. Platform/Build Prerequisites Clarity

- Document Linux system dependencies that may be required, including Wayland/X11 development packages as needed per target.
- Keep local build commands and troubleshooting hints in root docs.

## Workboard

Status key: `Not started` / `In progress` / `Done`.

| ID | Task | Owner | Status | Completion evidence |
|---|---|---|---|---|
| R1 | Freeze `0.2` public API surface | Maintainers | In progress | Local public-surface guard passed; RC announcement still pending |
| R2 | Docs parity pass across root docs | Maintainers | Done | Cross-checked wording in `README`, `MIGRATION`, `RELEASE_NOTES`, `ROADMAP`, and `RELEASE_PREP_0.3.0.md` |
| R3 | Guardrails + quality gates green | CI + Maintainers | In progress | Local `./scripts/verify_alpha_release.sh` passed; GitHub Actions confirmation still pending |
| R4 | Release rehearsal run | Maintainers | Done | `RELEASE_PREP_0.3.0.md` records a successful local `./scripts/verify_alpha_release.sh` run |
| R5 | Known limitations signoff | Maintainers | In progress | Release notes and validation docs list caveats; manual visual pass still pending |
| R6 | RC feedback triage and closure | Maintainers | Not started | All RC regressions fixed or explicitly deferred |
| R7 | Final `0.3.0` tag readiness | Maintainers | Not started | Final changelog/release notes signoff |

## Definition Of Done For `0.3.0`

`0.3.0` can be cut only when all conditions are true:

1. No planned breaking changes remain for `0.2.x`.
2. Required release verification scripts pass in CI.
3. Docs and migration pages are consistent and updated.
4. Known limitations are clearly listed in release notes.
5. RC feedback has been triaged and closed by fix or explicit defer.
6. Release owner signs off that remaining work belongs to `0.2.1+` or `0.3.0`.

## Deferred To `0.3.0`

- Removing deprecated compatibility wrappers, if migration feedback remains clean.
- Any public API reshaping that breaks `0.2.x` consumers.
- Large behavior changes that alter established `0.2.x` semantics.

## Execution Checklist

- [ ] **Freeze window opened**: announce `0.2` API freeze and accept only blocker fixes.
- [x] **Docs parity pass**: reconcile `README`, `MIGRATION`, `RELEASE_NOTES`, and examples index pages.
- [x] **Guardrails pass**: run `./scripts/check_quality.sh` and ensure no skipped gates.
- [x] **Release rehearsal**: run `./scripts/verify_alpha_release.sh` and archive logs/artifacts.
- [ ] **Known-limits signoff**: confirm release notes include current caveats and manual visual checks.
- [ ] **RC feedback window closed**: triage all `0.3.0-rc.*` regressions or defer explicitly to `0.2.1+` / `0.3.0`.
- [ ] **Tag readiness**: confirm changelog and release notes wording is final for `0.3.0`.

Checked items above are local repository evidence. `0.3.0` still requires GitHub Actions confirmation and RC feedback closure before tagging.

## Governance Rules During RC

- Only blocker fixes are allowed after `0.3.0-rc.1`; feature work moves to `0.2.1+` or `0.3.0`.
- Every accepted RC fix must include a changelog note, regression test when applicable, and migration/release-note impact review.
- Any proposed breaking change automatically exits the `0.3.0` scope and is queued for `0.3.0`.

## Notes

- This roadmap intentionally does not target `1.0.0`.
- Stability here means a clear contract for `0.2.x`, with the next breaking window at `0.3.0`.
