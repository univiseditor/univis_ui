# Univis UI Docs

This unified documentation book contains both language editions:

- [Arabic / العربية](ar/intro.md)
- [English](en/intro.md)

Use the language switcher shown at the top of each page to jump to the mirrored chapter in the other language.

## Structure

- `ar/` contains the Arabic documentation tree
- `en/` contains the English documentation tree
- both trees share the same chapter map and live in one mdBook
- hosted docs URL: `https://univiseditor.github.io/univis_ui/`

## Fast Links

- [Plugin Setup and First Examples](en/first-steps.md)
- [إعداد الإضافات وأولى الأمثلة](ar/first-steps.md)
- [English Example Gallery](en/examples/gallery.md)
- [معرض الأمثلة](ar/examples/gallery.md)
- [Migration and Limitations](en/migration/index.md)
- [الترحيل والقيود](ar/migration/index.md)

## Guides vs API Docs

- Use the guide chapters for mental models, migration notes, and example-driven learning.
- Use generated `rustdoc` for exact type paths, field lists, and signatures.

Generate API docs with:

```bash
cargo doc --no-deps -p univis_ui
```

## Build

```bash
mdbook build docs
```

## Serve

```bash
mdbook serve docs -n 127.0.0.1 -p 3000
```
