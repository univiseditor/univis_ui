# Docs Authoring Workflow

This page defines where new docs and examples should live.

## Where New Docs Go

- add end-user guides to `docs/src/en` and `docs/src/ar`
- keep the same information architecture in both trees
- put migration material under `docs/src/*/migration`
- keep API-signature detail in `rustdoc`, not in long guide prose

## Where New Examples Go

- put a new example in the crate that owns the feature
- keep root `examples/` for facade-level integrated demos only
- update the examples index in both languages when a new example is added
- add a short source comment back-link to the related guide when the example is canonical

## Adding Mirrored Pages

1. create the English page
2. create the Arabic counterpart in the matching path
3. add both to `docs/src/SUMMARY.md`
4. add or update the language switch target if the page is mirrored from an existing chapter

## Adding Public `rustdoc`

When a new public API surface is added:

- add or extend crate-level/module-level docs if the new surface changes discovery
- document the public type or plugin at the declaration site
- prefer short `no_run` examples
- hide internal helpers if they would pollute the generated docs story

## When To Update README And Changelog

- update `README.md` and `README_AR.md` when the landing story changes
- update `changelog.md` for notable docs/example/API surface changes
- use [Naming Conventions](naming-conventions.md) when adding or renaming maintainer-facing modules
