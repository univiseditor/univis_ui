# Testing and Validation

## Unit Tests

To reduce load on weaker machines, run tests one target at a time:

```bash
cargo test --release <test_name> --lib
```

## Quality Validation

```bash
./scripts/check_quality.sh
./scripts/check_representative_examples.sh
./scripts/check_examples_serial_release.sh
```

`./scripts/check_quality.sh` runs:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets` with the current CI allowlist for known Bevy-heavy lint debt
- `cargo check --workspace --all-targets`
- `./scripts/check_public_api_surface.sh`
- `./scripts/check_structure_guardrails.sh`
- `./scripts/check_engine_boundary_guardrails.sh`
- `cargo test --workspace --lib`

`./scripts/check_representative_examples.sh` now compiles the standalone Android phone package and then scans any currently shipped workspace example directories.

`./scripts/check_examples_serial_release.sh` remains useful when a branch actually ships `examples/*.rs` targets; in the current branch it may simply report that no workspace examples are present.

## Documentation Validation

```bash
cargo doc --no-deps
mdbook build docs
```

## Performance Baselines

Use these committed artifacts:

- `perf_baselines/current_max/2026-04-05/solver_benchmarks.txt`
- `perf_baselines/current_max/2026-04-05/runtime_benchmarks.txt`
- `perf_baselines/current_max/2026-04-05/manifest.json`

## CI Validation

GitHub Actions still validates quality, docs, API docs, and current example checks through the existing workflows.

## Sequential Validation On Low-End Machines

```bash
# all lib tests one by one
./scripts/test_lib_serial_release.sh

# all current workspace examples known to the script
./scripts/check_examples_serial_release.sh

# Android package plus workspace example scan
./scripts/check_representative_examples.sh

# full validation: lib tests + current examples
./scripts/verify_serial_release.sh

# sequential validation for a specific workspace package
./scripts/test_lib_serial_release.sh -p univis_ui_engine
./scripts/check_examples_serial_release.sh -p univis_ui_engine
./scripts/verify_serial_release.sh -p univis_ui_engine

# Android package directly
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets

# alpha validation before release
./scripts/verify_alpha_release.sh

# package alpha builds only, without validation
./scripts/package_alpha_serial.sh --no-verify
```

## Practical Pre-Merge Strategy

1. run the unit tests related to the change
2. run `./scripts/check_quality.sh`
3. run `./scripts/check_representative_examples.sh`
4. run `cargo check --workspace --examples`
5. launch the Android package when the change affects the live demo surface
6. use [Visual Validation](visual-validation.md) when the change is rendering-, layout-, or interaction-heavy

## Required Before The Next Alpha Cut

- `./scripts/check_quality.sh`
- `mdbook build docs`
- `cargo doc -p univis_ui_style --no-deps`
- `cargo doc -p univis_ui_engine --no-deps`
- `cargo doc -p univis_ui_interaction --no-deps`
- `cargo doc -p univis_ui_widgets --no-deps`
- `cargo doc -p univis_ui --no-deps`
- `cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets`
- `./scripts/check_representative_examples.sh`
- one manual pass through the Android package and relevant current examples
- one pass through [Release Readiness](release-readiness.md)

## Screenshot Policy

- screenshots remain manual release-prep material
- the example gallery can link to static visual references
- screenshot generation is not a required CI gate in the current alpha line
