# Testing and Validation

## Unit Tests

To reduce load on weaker machines, run tests one target at a time:

```bash
cargo test --release <test_name> --lib
```

## Build Validation

```bash
cargo check --workspace --all-targets
./scripts/check_examples_serial_release.sh
```

## Documentation Validation

```bash
cargo doc --no-deps
mdbook build docs
```

## Sequential Validation On Low-End Machines

```bash
# all lib tests one by one
./scripts/test_lib_serial_release.sh

# all examples one by one
./scripts/check_examples_serial_release.sh

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
2. run `cargo check --workspace --all-targets`
3. run `./scripts/check_examples_serial_release.sh`
4. launch at least one example related to the modified area
