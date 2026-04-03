# Docs Publishing

This page explains how the hosted documentation build is produced from the repository.

## Public URL

- Hosted docs URL: `https://univiseditor.github.io/univis_ui/`

## Source Of Truth

- the book source lives under `docs/`
- `docs/book.toml` defines the mdBook configuration
- the bilingual chapter trees live under `docs/src/en` and `docs/src/ar`
- `mdbook build docs` produces the static site under `target/book/docs`

## Publishing Model

- validation remains in `.github/workflows/docs_examples_api.yml`
- publishing is handled only by `.github/workflows/docs_publish.yml`
- the hosted site is built from the same `docs/` source tree used locally
- the intended public deployment target is GitHub Pages

## Local Contributor Workflow

```bash
mdbook build docs
mdbook serve docs -n 127.0.0.1 -p 3000
```

Keep this local path as the canonical way to preview changes before pushing.

## What To Verify In The Hosted Build

- the docs home page loads correctly
- the English tree is reachable under the hosted book
- the Arabic tree is reachable under the hosted book
- the language switcher works across mirrored pages
- example gallery links and migration links remain valid

## Related Pages

- [Docs Home](../index.md)
- [Example Gallery](../examples/gallery.md)
- [Testing and Validation](testing.md)
