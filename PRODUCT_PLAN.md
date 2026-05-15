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
   fixture evidence graph, crop scoring, JSON output. **Done.**
2. **Repo and Markdown ingest**: walk local repos/vaults, chunk text, attach
   provenance, build dependency/reference edges. **First slice done via
   `crop pack`.**
3. **Crop engine**: task seed expansion, boundary scoring, bridge selection,
   dedupe, token-budget packing.
4. **Agent harness**: CLI commands that emit model-ready context packs and a
   replayable manifest. **Prefix-cache-aware pack output is available with
   `--prefix-cache generic`.**
5. **Evaluation loop**: fixtures and benchmarks against top-k vector retrieval,
    measuring noise, missing bridges, source coverage, and later real-corpus
    latency. **Five apple-to-apple fixture cases are available via `crop eval`;
    offline external competitor exports can be scored with `crop eval-import`.**
6. **Real-corpus benchmarks**: local root timing, benchmark manifests,
   competitor export examples, and aggregate reports. **Active wave; `crop bench`
   supports one-off roots and named manifest views.**

## Upstream kernel asks

CROP should keep product-specific UX and packaging in this repo, while reusable
graph/stat/math primitives land in BISECT shared kernels:

| BISECT issue | Status | Kernel area | Why CROP needs it |
|--------------|--------|-------------|-------------------|
| [#4](https://github.com/giodl73-repo/BISECT/issues/4) | Done | shared-kernel contract | Keep `rctx-core`, `rgraph-core`, `rstat-core`, and `ropt-core` stable as external Git dependencies. |
| [#5](https://github.com/giodl73-repo/BISECT/issues/5) | Done | `rgraph-core` boundary metrics | Score context crop boundaries with cut size, conductance-like ratios, and internal/external degree. |
| [#6](https://github.com/giodl73-repo/BISECT/issues/6) | Done | `rgraph-core` / `ropt-core` bridge helpers | Recover connector artifacts that plain top-k retrieval misses. |
| [#8](https://github.com/giodl73-repo/BISECT/issues/8) | Done | `rstat-core` evidence scoring | Normalize relevance, authority, freshness, diversity, and provenance confidence. |
| [#9](https://github.com/giodl73-repo/BISECT/issues/9) | Done | `ropt-core` budgeted packing | Optimize context packs under token budgets rather than truncating ranked lists. |
| [#10](https://github.com/giodl73-repo/BISECT/issues/10) | Done | `rgraph-core` clustering | Build cluster-aware context packs from actual evidence neighborhoods. |
| [#11](https://github.com/giodl73-repo/BISECT/issues/11) | Done | graph/stat/math fixtures | Prove graph-native wins against top-k retrieval before adding LLM calls. |

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

