# Alpha Release Checklist

Target release: `0.2.0-alpha.1`

## Publish order
1. `univis_ui_style`
2. `univis_ui_engine`
3. `univis_ui_interaction`
4. `univis_ui_widgets`
5. `univis_ui`

## Verification
Run the full pre-publish gate:

```bash
./scripts/verify_alpha_release.sh
```

This repository publishes interdependent crates. Before the leaf crates are on crates.io, only the first leaf crate can be fully packaged offline. The verification script therefore:
- creates a real tarball for `univis_ui_style`
- audits package contents for the dependent crates with `cargo package --list`

If you only want local package rehearsals without verify:

```bash
./scripts/package_alpha_serial.sh --offline --no-verify
```

If you only want to audit packaged file lists:

```bash
./scripts/package_alpha_serial.sh --offline --list-only
```

## Manual checks
1. Confirm README installation snippets still use `0.2.0-alpha.1`.
2. Confirm the example set you care about still runs manually on the target machine.
3. Confirm `target/package/` contains crates for all five public packages.
4. Publish in dependency order only.

## Publish commands
```bash
cargo publish -p univis_ui_style
cargo publish -p univis_ui_engine
cargo publish -p univis_ui_interaction
cargo publish -p univis_ui_widgets
cargo publish -p univis_ui
```
