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
- Chapters reference concrete paths in `src/` and `examples/`.
- If code and docs diverge, treat the source code as the ground truth.
- English translation is incremental; structure is complete, while some chapter bodies may still be Arabic.

## Requirements

- Rust (recent stable toolchain)
- Bevy `0.17.3` (via `Cargo.toml`)
- Basic ECS knowledge is recommended

## Build Books

```bash
cargo install mdbook
mdbook build book_ar   # Arabic edition
mdbook build book_en   # English edition
```

Serve locally:

```bash
mdbook serve book_ar -n 127.0.0.1 -p 3000     # Arabic
mdbook serve book_en -n 127.0.0.1 -p 3001     # English
```
