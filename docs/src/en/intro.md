# Introduction

`univis_ui` is a UI framework built on top of Bevy, using SDF-based rendering for crisp 2D and 3D interfaces.

This book is the full operational reference for the project. It covers:

- project architecture and core modules
- runtime composition through `UnivisUiPlugin`
- layout components (`UNode`, `ULayout`, `USelf`) and advanced extensions
- picking and interaction (`UInteraction`)
- built-in widgets, behavior, and emitted events
- rendering, clipping (`UClip`), performance, and profiling
- practical example history, static references, and current runnable paths

## Scope

- This book documents the **actual repository state**.
- Chapters reference concrete paths in `crates/*/src/`, the current Android demo package, and the bilingual examples catalog.
- If code and docs diverge, treat the source code as the ground truth.

## Requirements

- Rust (recent stable toolchain)
- Bevy `0.18.1` (via `Cargo.toml`)
- Basic ECS knowledge is recommended

## Build Books

```bash
cargo install mdbook
mdbook build docs
```

Serve locally:

```bash
mdbook serve docs -n 127.0.0.1 -p 3000
```

Hosted docs URL:

```text
https://univiseditor.github.io/univis_ui/
```

## Guides vs API Docs

- Use this book for concepts, migration notes, and workflow guidance.
- Use generated `rustdoc` for exact type paths, fields, and signatures.

Generate the API docs with:

```bash
cargo doc --no-deps -p univis_ui
```

## Where To Look Next

- example availability map: [Examples](examples/index.md)
- curated availability view: [Example Gallery](examples/gallery.md)
- related API index: [API Reference](api/index.md)
- related migration notes: [Migration and Limitations](migration/index.md)
