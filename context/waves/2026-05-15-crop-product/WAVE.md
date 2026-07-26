# Wave: MDCROP Product Foundation

## Goal

Build MDCROP from a scaffold into a local-first Rust context-mdcropping engine that
can ingest evidence, construct graph neighborhoods, select a task mdcrop, and emit
provenance-aware context packs for AI agents.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Repository foundation | done | Rust workspace, BISECT dependency wiring, product docs, skills, and initial mdcrop fixture. |
| 02 | Markdown/repo ingest | done | Added deterministic file ingest, source hashes, chunks, Markdown-link edges, task scoring, and `mdcrop pack`. |
| 03 | Mdcrop boundary engine | todo | Add graph-cut/budgeted mdcrop scoring, bridge selection, and mdcrop reasons. |
| 04 | Context pack manifest | done | Added prefix-cache-aware pack manifests with stable prefix hashes and task tails. |
| 05 | Evaluation fixtures | in progress | Added first missing-bridge apple-to-apple eval against score top-k. |

## Validation baseline

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

