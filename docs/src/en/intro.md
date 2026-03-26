# Introduction

`univis_ui` is a UI framework built on top of Bevy, using SDF-based rendering for crisp 2D and 3D interfaces.

This book is the full operational reference for the project. It covers:

- project architecture and core modules
- runtime composition through `UnivisUiPlugin`
- layout components (`UNode`, `ULayout`, `USelf`) and advanced extensions
- picking and interaction (`UInteraction`)
- built-in widgets, behavior, and emitted events
- rendering, clipping (`UClip`), performance, and profiling
- practical examples and usage patterns

## Scope

- This book documents the **actual repository state**.
- Chapters reference concrete paths in `crates/*/src/`, `crates/*/examples/`, and the root `examples/` directory.
- If code and docs diverge, treat the source code as the ground truth.
- English translation is incremental; structure is complete, while some chapter bodies may still be Arabic.

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
