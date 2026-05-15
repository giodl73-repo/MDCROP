# Pulse 18: PROOF status page

## Goal

Give PROOF a single CROP command that can generate a combined Markdown status
page from the side-info and artifact health surfaces.

## Changes

- Added `crop proof-status`.
- Composed document catalog, link audit, backlinks, frontmatter, headings, and
  artifact health into one Markdown page.
- Kept JSON machine-readable contracts on the individual commands.
- Added CLI crate coverage using the deterministic PROOF fixture.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- PROOF fixture smoke for `crop proof-status`
- `git diff --check`

## Status

Done.
