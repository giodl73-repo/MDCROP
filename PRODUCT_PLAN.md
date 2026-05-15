# CROP Product Plan

## Product thesis

CROP is a local-first graph engine for AI context. It turns scattered artifacts
into an evidence graph, then crops the graph into a compact, provenance-aware
context pack for a task.

The wedge is not "another index." The wedge is **the right context boundary**:
include dense relevant neighborhoods, bridge nodes, canonical sources, and
provenance paths; exclude stale duplicates and noisy periphery.

## Target users

1. AI builders wiring agents to repositories, docs, and issue trackers.
2. Research-heavy builders using Markdown, Obsidian, PDFs, GitHub, and notes.
3. Technical teams that need reproducible context handoffs across sessions.

## Differentiation

| Capability | Ordinary vector/RAG index | CROP |
|------------|---------------------------|------|
| Retrieval unit | Similar chunks | Typed graph neighborhoods |
| Boundary | Top-k or score threshold | Graph-cut / budgeted crop |
| Missing links | Often missed | Bridge and path search |
| Staleness | Metadata filter | Recency and supersedes edges |
| Provenance | Optional source labels | First-class source trails |
| Context budget | Truncate or rerank | Optimize coverage, diversity, authority, and freshness |

## Waves

1. **Local graph fixture**: Rust workspace, BISECT shared-kernel dependencies,
   fixture evidence graph, crop scoring, JSON output.
2. **Repo and Markdown ingest**: walk local repos/vaults, chunk text, attach
   provenance, build dependency/reference edges.
3. **Crop engine**: task seed expansion, boundary scoring, bridge selection,
   dedupe, token-budget packing.
4. **Agent harness**: CLI commands that emit model-ready context packs and a
   replayable manifest.
5. **Evaluation loop**: fixtures and benchmarks against top-k vector retrieval,
   measuring noise, missing bridges, source coverage, and latency.

## First command shape

```powershell
crop pack --root C:\src\apportionment --task "explain report provenance" --budget 12000 --output context.crop.json
```

## Success criteria

- Produces deterministic JSON context packs with source IDs and crop reasons.
- Demonstrates graph-native wins on fixtures before adding embeddings.
- Keeps local ingest and crop selection fast enough to run before each agent
  turn.
- Maintains GitHub dependencies with a local peer-development patch path.

