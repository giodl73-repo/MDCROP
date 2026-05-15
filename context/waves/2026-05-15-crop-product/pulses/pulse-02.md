# Pulse 02: Markdown/repo ingest

## Status

Done.

## Goal

Build the first ingest path from a local directory into an evidence graph.

## Deliverables

- [x] Walk files under a root path with deterministic ordering.
- [x] Accept Markdown, text, JSON, TOML, YAML, and Rust files in the first slice.
- [x] Produce source IDs, SHA-256 hashes, unit IDs, and basic token estimates.
- [x] Build edges from same-file adjacency and explicit Markdown links.
- [x] Add positive fixture and malformed/unreadable input coverage.

## Validation

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

