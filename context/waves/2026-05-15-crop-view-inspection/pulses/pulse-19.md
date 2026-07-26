# Pulse 19: View-backed status page

## Goal

Let generated corpus status pages use named MDCROP views so tag, status, or other
frontmatter-filtered slices can produce focused status pages.

## Changes

- Added `mdcrop status --view <mdcrop.view.v1>`.
- Preserved `frontmatter_query` and view-relative roots when composing catalog,
  links, backlinks, frontmatter, and headings.
- Kept `--root` support for raw-root status pages and rejected combining it with
  `--view`.
- Documented view-backed status pages.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- fixture smoke for `mdcrop status --view`
- `git diff --check`

## Status

Done.
