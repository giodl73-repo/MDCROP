# Pulse 17: Side-info status tables

## Goal

Let PROOF render MDCROP side-info reports directly into generated README/status
pages without losing the stable JSON feeds.

## Changes

- Added Markdown renderers for link audit, backlink, frontmatter, and heading
  reports in `mdcrop-core`.
- Added `--format markdown` to `mdcrop links`, `mdcrop backlinks`,
  `mdcrop frontmatter`, and `mdcrop headings`.
- Documented the Markdown output mode in README and side-info schema notes.
- Added core coverage for the Markdown renderers.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke checks for Markdown output on the PROOF fixture
- `git diff --check`

## Status

Done.
