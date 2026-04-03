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
- `cargo test --workspace --lib`

`./scripts/check_representative_examples.sh` checks a curated release-mode set that covers the
facade path, root modes, interaction, panel resize, text input, and the `World3d` render path.

## Documentation Validation

```bash
cargo doc --no-deps
mdbook build docs
```

## CI Validation

GitHub Actions now validates quality, docs, examples, and API docs through:

- `.github/workflows/docs_examples_api.yml`
- `.github/workflows/docs_publish.yml`

It runs:

- `./scripts/check_quality.sh`
- `./scripts/check_representative_examples.sh`
- `mdbook build docs`
- `cargo doc --no-deps` for each public crate
- package-by-package example checking through `./scripts/check_examples_serial_release.sh -p ...`
- one dedicated GitHub Pages publishing path for the hosted docs site on `main`

## Sequential Validation On Low-End Machines

```bash
# all lib tests one by one
./scripts/test_lib_serial_release.sh

# all examples one by one
./scripts/check_examples_serial_release.sh

# representative cross-surface smoke compile pass
./scripts/check_representative_examples.sh

# full validation: lib tests + examples
./scripts/verify_serial_release.sh

# sequential validation for a specific workspace package
./scripts/test_lib_serial_release.sh -p univis_ui_engine
./scripts/check_examples_serial_release.sh -p univis_ui_engine
./scripts/verify_serial_release.sh -p univis_ui_engine

# alpha validation before release: check + lib tests + examples + package
./scripts/verify_alpha_release.sh

# package alpha builds only, without validation
./scripts/package_alpha_serial.sh --no-verify
```

To validate only selected examples:

```bash
./scripts/check_examples_serial_release.sh -p univis_ui hello_world
./scripts/check_examples_serial_release.sh -p univis_ui_interaction interaction
./scripts/check_examples_serial_release.sh -p univis_ui_widgets select
```

## Practical Pre-Merge Strategy

1. run the unit tests related to the change
2. run `./scripts/check_quality.sh`
3. run `./scripts/check_representative_examples.sh`
4. run `./scripts/check_examples_serial_release.sh` for the touched package or before release
5. launch at least one example related to the modified area
6. use [Visual Validation](visual-validation.md) when the change is rendering-, layout-, or interaction-heavy

## Required Before The Next Alpha Cut

- `./scripts/check_quality.sh`
- `mdbook build docs`
- `cargo doc -p univis_ui_style --no-deps`
- `cargo doc -p univis_ui_engine --no-deps`
- `cargo doc -p univis_ui_interaction --no-deps`
- `cargo doc -p univis_ui_widgets --no-deps`
- `cargo doc -p univis_ui --no-deps`
- `./scripts/check_representative_examples.sh`
- `./scripts/check_examples_serial_release.sh -p univis_ui_engine`
- `./scripts/check_examples_serial_release.sh -p univis_ui_widgets`
- `./scripts/check_examples_serial_release.sh -p univis_ui_interaction`
- `./scripts/check_examples_serial_release.sh -p univis_ui`
- one manual pass through the representative examples in [Visual Validation](visual-validation.md)
- one pass through [Alpha2 Release Readiness](alpha2-release-readiness.md)

## Screenshot Policy

- screenshots remain manual release-prep material
- the example gallery can link to static visual references
- screenshot generation is not a required CI gate in the current alpha line
