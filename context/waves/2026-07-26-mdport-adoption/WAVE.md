---
wave: mdport-adoption
date_open: 2026-07-26
status: done
source_request: "Rename PEBBLE and pebble.v1 to MDPORT and mdport.v1."
---

# Wave: MDPORT adoption

## Mission

Move CROP's portable Markdown document and corpus-slice boundary from PEBBLE to
MDPORT without changing CROP's graph selection, indexing, or corpus ownership.

## Changes

- Depend on `mdport` 0.2.0 from the renamed MDPORT repository.
- Emit and ingest `mdport.v1`.
- Rename Rust APIs, CLI format names, fixtures, specs, and generated examples.
- Align old PROOF fixture terminology to MDLOOM.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`
