# Wave: CROP Product Foundation

## Goal

Build CROP from a scaffold into a local-first Rust context-cropping engine that
can ingest evidence, construct graph neighborhoods, select a task crop, and emit
provenance-aware context packs for AI agents.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Repository foundation | done | Rust workspace, BISECT dependency wiring, product docs, skills, and initial crop fixture. |
| 02 | Markdown/repo ingest | done | Added deterministic file ingest, source hashes, chunks, Markdown-link edges, task scoring, and `crop pack`. |
| 03 | Crop boundary engine | todo | Add graph-cut/budgeted crop scoring, bridge selection, and crop reasons. |
| 04 | Context pack manifest | todo | Emit replayable JSON manifests with source trails and deterministic hashes. |
| 05 | Evaluation fixtures | todo | Compare CROP against top-k retrieval fixtures for noise and missing-bridge cases. |

## Validation baseline

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

