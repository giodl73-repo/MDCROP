# Pulse 16: Artifact Status Tables

## Goal

Make MDLOOM artifact health embeddable in generated README/status pages without
requiring a downstream JSON renderer.

## Scope

- Add a reusable Markdown renderer for `MdloomArtifactReport`.
- Add `crop artifacts --format markdown`.
- Keep JSON as the default output format.
- Cover the Markdown table in tests.
- Update MDLOOM-facing docs.

## Checklist

- [x] Core Markdown renderer.
- [x] CLI format flag.
- [x] Unit coverage for table rows.
- [x] README, spec, product plan, and wave updates.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] Fixture JSON and Markdown CLI smoke runs.
- [x] `git diff --check`

## Outcome

`crop artifacts --format markdown` emits a ready-to-embed artifact health table
with source/output mappings, target/status, existence, staleness, cache,
directive, and diagnostic columns.
