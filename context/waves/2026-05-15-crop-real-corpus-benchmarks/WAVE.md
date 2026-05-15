# Wave: CROP Real Corpus Benchmarks

## Goal

Move from deterministic toy fixtures to reproducible local-corpus measurements
that show how CROP behaves on actual repositories and document trees.

## Thesis

Fixture wins prove specific retrieval failure modes. The next proof needs real
local roots, measured ingest/score/crop costs, stable benchmark JSON, and a path
for external competitors to export comparable selections without hosted test
dependencies.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Local bench command | done | Added `crop bench` for local root timing and pack summary JSON. |
| 02 | Benchmark manifest inputs | done | Added repeatable named benchmark/view manifests for multiple benchmark cases. |
| 03 | Competitor export examples | done | Added checked offline JSON examples for vector, lexical, hybrid, RAG, and graph systems. |
| 04 | Aggregate benchmark report | done | Added suite-level aggregate totals, means, and max runtime to manifest benchmark reports. |
| 05 | Real-corpus benchmark docs | todo | Document fair apples-to-apples benchmark setup and interpretation. |

## Success criteria

- `crop bench --root ... --task ... --budget ...` emits JSON with corpus counts,
  selected token counts, quality score, and elapsed milliseconds by stage.
- Benchmark runs remain local-only and do not require hosted services.
- Timing metrics are separated from fixture metrics and documented as
  environment-dependent.
- Future competitor outputs can be compared through the existing
  `crop eval-import` contract or benchmark manifest exports.
- `cargo fmt`, `cargo test --workspace`, benchmark smoke runs, and
  `git diff --check` pass for each implementation pulse.

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| Overclaiming from local timing | Label bench timings as machine/root dependent and keep fixture quality metrics separate. |
| Benchmark drift | Emit structured JSON with schema version and stage timings. |
| Hosted dependency creep | Keep first benchmark command local-only and dependency-free. |
| Too much output | Report summary metrics first; full packs remain available through `crop pack`. |

## Validation baseline

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- bench --root . --task "explain crop benchmark" --budget 12000
git diff --check
```
