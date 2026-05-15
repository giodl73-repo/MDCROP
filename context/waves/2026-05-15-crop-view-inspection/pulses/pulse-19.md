# Pulse 19: View-backed status page

## Goal

Let generated PROOF status pages use named CROP views so tag, status, or other
frontmatter-filtered slices can produce focused status pages.

## Changes

- Added `crop proof-status --view <crop.view.v1>`.
- Preserved `frontmatter_query` and view-relative roots when composing catalog,
  links, backlinks, frontmatter, and headings.
- Kept `--docs-root` support for raw-root status pages and rejected combining it
  with `--view`.
- Documented view-backed status pages.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- PROOF fixture smoke for `crop proof-status --view`
- `git diff --check`

## Status

Done.
