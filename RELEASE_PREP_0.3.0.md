# `0.3.0` Release Prep Evidence

Date: 2026-05-18

This note records local evidence for the next `0.3.0` stabilization cut. It does not replace GitHub Actions, an RC announcement, or RC feedback triage.

## Status

- Local quality gate: passed.
- Local release rehearsal: passed.
- Local docs build: passed.
- GitHub Actions confirmation: pending.
- Manual visual pass: pending.
- RC feedback window: not started.

## Local Verification

The final local verification run used the ignored local `Cargo.lock` after updating `fastrand` from `2.4.0` to `2.4.1`, removing the yanked-package warning seen during the first package rehearsal. `Cargo.lock` remains ignored for this library workspace.

Commands run:

```bash
cargo update -p fastrand
./scripts/verify_alpha_release.sh
mdbook build docs
```

`./scripts/verify_alpha_release.sh` completed successfully and covered:

- `./scripts/check_quality.sh`
- release-mode library tests for public workspace crates
- standalone Android package check
- release-mode checks for all 15 shipped workspace examples
- package rehearsal and package file-list generation

Release-mode library test counts from the final rehearsal:

- `univis_ui_style`: no lib tests
- `univis_ui_engine`: 87 passed
- `univis_ui_interaction`: 10 passed
- `univis_ui_widgets`: 56 passed
- `univis_ui`: 3 passed

The package rehearsal completed without the previous local yanked `fastrand` warning.

## Remaining Release-Process Work

- Run the same gates in GitHub Actions and link the CI run.
- Open the `0.3.0-rc.1` freeze/feedback window.
- Complete one manual visual pass through the Android-style package and the current representative examples.
- Close or explicitly defer all RC feedback before tagging `0.3.0`.
