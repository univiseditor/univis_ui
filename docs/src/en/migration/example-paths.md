# Example Path Migration

This page records the current example locations.

## Current Paths

- Android package demo: `android/android_phone_app`
- root workspace examples: `examples/*.rs`
- grid examples: `examples/grid/*.rs`
- layout examples: `examples/layout/*.rs`
- widget examples: `examples/widgets/*.rs`

## Current Commands

Run the Android-style app:

```bash
cargo run --manifest-path android/android_phone_app/Cargo.toml
```

Check every workspace example registered with Cargo:

```bash
cargo check --workspace --examples
```

Check the standalone Android package:

```bash
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets
```

## Related Guides

- [Examples](../examples/index.md)
- [Example Gallery](../examples/gallery.md)
- [Testing and Validation](../development/testing.md)
