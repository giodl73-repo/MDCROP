# Pulse 06: Markdown index API

## Scout commands

```powershell
cargo run -q -p crop-cli -- view --inspect --dir examples\views --strict
Get-ChildItem C:\src\proof
```

## Decision

PROOF should own Markdown/HTML/ASCII rendering, but it can depend on CROP for
corpus indexing primitives. This slice adds a reusable `crop-core` API and a CLI
smoke surface for README/TOC-style source tables without writing into PROOF
outputs by default.

## Checklist

- [x] Add source-index data structures.
- [x] Add root-based Markdown index generation.
- [x] Add view-based Markdown index generation.
- [x] Add CLI `crop index` with optional output path.
- [x] Add positive tests for root and view indexing.
- [x] Update README, product plan, and wave docs.
- [x] Create PROOF-side integration spec.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p crop-cli -- index --root C:\src\proof -e md -e html --exclude-dir target
cargo run -q -p crop-cli -- index --view examples\views\whole-repo-view.json -e md
git diff --check
```
