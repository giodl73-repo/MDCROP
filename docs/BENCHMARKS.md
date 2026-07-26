# MDCROP Benchmark Guide

This guide explains how to run fair, local, apples-to-apples MDCROP benchmarks.
The benchmark suite has two layers:

1. `mdcrop eval` fixture tests: deterministic quality comparisons over known
   failure modes and labels.
2. `mdcrop bench` real-corpus tests: local repository or document-tree
   measurements for ingest, scoring, mdcrop selection, and pack summary metrics.

Use fixture results to make quality claims. Use real-corpus benchmark results to
track local performance and context-shaping behavior on actual data.

## Fair comparison rules

Every comparison should keep these inputs fixed:

| Input | Rule |
|-------|------|
| Corpus | Use the same files, source IDs, chunking, and token estimates. |
| Task | Use the same task string for MDCROP and every competitor. |
| Budget | Use the same token budget and reject over-budget selections. |
| Labels | For fixture scoring, use the same gold, bridge, and noise labels. |
| Timing | Keep timing separate from fixture quality metrics. |
| Dependencies | Do not require hosted services in MDCROP validation. Export external results offline. |

## Fixture comparisons

Run MDCROP and local baselines with:

```powershell
cargo run -p mdcrop-cli -- eval --all --budget 160
```

The fixture suite compares `mdcrop`, `score-top-k`, `lexical-top-k`, and
`hybrid-top-k` on identical units, labels, and token budgets. Metrics include
gold recall, bridge recall, noise rate, source coverage, budget efficiency,
noise-token share, and quality score.

External systems should export `mdcrop.competitor-results.v1` JSON and score it
offline:

```powershell
cargo run -p mdcrop-cli -- eval-import --input examples\competitors\qdrant-vector.missing-bridge.json
```

Checked examples for vector, lexical, hybrid, RAG orchestration, and graph
systems live in `examples\competitors\`.

## Real-corpus one-off benchmarks

Run MDCROP against a local root:

```powershell
cargo run -p mdcrop-cli -- bench --root C:\src\MDCROP --task "explain graph context boundary" --budget 12000
```

The output is `mdcrop.bench.v1` JSON with:

| Field | Meaning |
|-------|---------|
| `source_count`, `unit_count`, `edge_count` | Corpus shape after local ingest. |
| `selected_unit_count`, `selected_token_estimate` | Mdcrop size under the requested budget. |
| `boundary_cut_edges`, `boundary_conductance` | Boundary tightness of the selected mdcrop. |
| `relevance_mean`, `quality_score` | Summary of selected context quality. |
| `timings` | Ingest, scoring, mdcrop, and total elapsed milliseconds. |

Timing is machine-, filesystem-, cache-, and corpus-dependent. Do not compare
timings from different machines as product claims without controlling the
environment.

## Repeatable benchmark views

Use a manifest to define named reusable views over a corpus:

```json
{
  "schema_version": "mdcrop.bench-manifest.v1",
  "cases": [
    {
      "id": "docs-boundary",
      "root": "docs",
      "task": "explain graph context boundary",
      "token_budget": 12000,
      "seed": 0,
      "view": "docs-only"
    },
    {
      "id": "repo-architecture",
      "root": ".",
      "task": "explain MDCROP architecture",
      "token_budget": 12000,
      "seed": 0,
      "view": "whole-repo"
    }
  ]
}
```

Run it with:

```powershell
cargo run -p mdcrop-cli -- bench --manifest benchmark-manifest.json
```

Relative roots resolve from the manifest file location. The optional `view`
field names the mdcrop recipe under test; it is useful for separating docs-only,
whole-repo, architecture, troubleshooting, or issue-focused benchmark cases.

Manifest output is `mdcrop.bench-report.v1` JSON. It includes:

- `cases`: per-view `mdcrop.bench.v1` rows.
- `aggregate`: suite-level totals, mean selected size, mean quality, mean stage
  timings, and max total runtime.

Use the aggregate block for quick run-to-run comparison, and use case rows to
debug individual views.

## Interpreting results

Good MDCROP behavior usually looks like:

- selected token counts stay well under budget without empty mdcrops;
- boundary conductance is low when the task maps to a tight neighborhood;
- quality score rises for task-aligned views compared with broad/noisy views;
- bridge-heavy fixture cases preserve bridge recall better than top-k ranking;
- ingest dominates first-run time, while scoring and mdcrop selection remain small
  enough to run before an agent turn.

Bad or inconclusive behavior usually looks like:

- selected token counts are near zero because the seed or task is wrong;
- high boundary conductance indicates the selected context is leaking across a
  broad graph boundary;
- timing differences are small enough to be filesystem noise;
- competitors were scored on different chunks, labels, or token budgets.

When comparing with vector databases, search indexes, RAG frameworks, or graph
retrievers, export their selected MDCROP fixture unit IDs to JSON and score those
exports with `mdcrop eval-import`. Keep hosted service setup and retrieval outside
MDCROP's own tests so the repository remains local-first and deterministic.
