# Pulse 01: Frontmatter query adapter

## Goal

Use SLICE for CROP's low-level frontmatter predicate parsing and evaluation.

## Changes

- Add `slice-core` as a dependency.
- Replace local clause splitting with SLICE parse/compile/evaluate.
- Derive a CROP-owned field catalog from parsed clauses.
- Materialize required fields from Markdown frontmatter before evaluation.
- Preserve current CROP behavior for array-like tag strings and missing-field
  `ne`.

## Boundary

CROP still owns Markdown frontmatter extraction, view schemas, graph selection,
ranking/scoring, output rendering, and compatibility policy. SLICE only handles
the row selector kernel.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p crop-cli -- view --file examples\mdloom-fixture\mdloom-ready-view.json`
- `git diff --check`

## Status

Done.
