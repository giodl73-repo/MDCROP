# Pulse 18: Corpus status page

## Goal

Give any Markdown corpus a single CROP command that can generate a combined
Markdown status page from the side-info surfaces.

## Changes

- Added `crop status`.
- Composed document catalog, link audit, backlinks, frontmatter, headings, and
  health summary into one Markdown page.
- Kept JSON machine-readable contracts on the individual commands.
- Added CLI crate coverage using the deterministic fixture.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- fixture smoke for `crop status`
- `git diff --check`

## Status

Done.
