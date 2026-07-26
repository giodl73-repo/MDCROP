---
wave: mdcrop-rename
date_open: 2026-07-26
status: done
source_request: "Rename CROP to MDCROP as part of the Markdown product family."
---

# Wave: MDCROP rename

## Mission

Establish MDCROP as the Markdown corpus discovery and selection layer between
MDPATH addressing and MDLOOM compilation.

## Changes

- Repository and product: MDCROP.
- Crates: `mdcrop-core` and `mdcrop-cli`.
- Binary: `mdcrop`.
- State: `.mdcrop/`.
- View schema: `mdcrop.view.v1`.
- Generated report schemas: `mdcrop.*`.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`
