# Pulse 03: Crop boundary engine

## Status

Todo.

## Goal

Replace the fixture neighbor crop with task-specific graph-boundary selection.

## Deliverables

- Score seed neighborhoods by relevance, authority, freshness, and diversity.
- Add graph-cut or cut-like boundary metrics using `rgraph-core`.
- Add bridge-node selection for missing-link fixtures.
- Emit crop reasons for seed, bridge, authority, freshness, and boundary.
- Add comparison fixtures against plain top-k retrieval.

## Validation

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

