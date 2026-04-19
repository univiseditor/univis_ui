# Editorial Rules

These rules keep the English and Arabic documentation trees mirrored, searchable, and easy to maintain.

## Language Separation

- One language per file.
- English pages use English prose only.
- Arabic pages use Arabic prose only.
- Shared API identifiers stay inside backticks, for example `URootUi` or `UTextLabel`.

## Page Structure

Prefer a short and repeatable shape:

1. What this page is for.
2. The minimum mental model.
3. Practical rules or examples.
4. Related examples.
5. Related API types when useful.

## Naming Rules

- Use the public API name exactly as it appears in Rust.
- Do not invent alternative nicknames for public types.
- When an example is mentioned, include both the example name and its owning package.

## Example-Link Rules

When a guide references an example:

- point to the canonical entry in `docs/src/en/examples/index.md`
- mention the owning package
- only use `cargo run -p <package> --example <name>` when that example source is actually shipped in the current branch; otherwise point to the availability catalog or the current live package

## API Doc Rules

- `rustdoc` should explain intent, not repeat field names.
- Short `no_run` examples are preferred over long walls of prose.
- Internal helpers that are not part of the intended public story should be hidden or de-emphasized.

## Mirror Rules

- If an English page is added, add the Arabic counterpart in the same change.
- Keep the same information architecture even when wording differs.
- If only one tree is edited, note why before merging.
