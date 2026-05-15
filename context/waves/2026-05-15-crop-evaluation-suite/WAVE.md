# Wave: CROP Evaluation Suite

## Goal

Turn the first missing-bridge proof into a repeatable, multi-fixture evaluation
suite that compares CROP against ordinary index/ranking baselines under equal
inputs, labels, budgets, and output metrics.

## Thesis

CROP should win when context quality depends on graph structure rather than raw
rank: bridge nodes, tight boundaries, freshness/canonicality, cross-file links,
and token-budget composition.

## Competitor classes

| Class | Baseline in this wave | Why it matters |
|-------|-----------------------|----------------|
| Score ranking | `score-top-k` | Captures naive vector/RAG truncation behavior when relevance is already available. |
| Lexical search | `lexical-top-k` | Captures BM25/Lucene-style keyword retrieval without adding an external service. |
| Hybrid ranking | `hybrid-top-k` | Captures common vector+keyword reranking shape with deterministic local scores. |
| Graph crop | `crop` | CROP's graph-boundary, bridge, and budget-aware selection. |

External vector DBs and hosted indexes are intentionally deferred until the
offline suite is stable. They should plug into the same fixture/metric contract,
not define a separate benchmark.

## Apple-to-apple rules

1. Every method sees the same evidence units, source IDs, graph edges, scores,
   task string, token budget, and gold/noise labels.
2. No LLM calls are allowed in fixture scoring.
3. Metrics must be deterministic JSON: selected units, token use, gold recall,
   bridge recall, noise rate, source coverage, budget efficiency, and latency.
4. A fixture is only accepted when it has a positive CROP expectation and a
   baseline failure mode documented in the pulse.
5. Baselines must be strong enough to be fair: deterministic tie-breaking,
   budget enforcement, and the same chunk/token accounting as CROP.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Fixture expansion | done | Added noisy-periphery, stale-duplicate, budget-packing, and cross-link-chain cases. |
| 02 | Baseline methods | done | Added lexical and hybrid top-k baselines beside score top-k. |
| 03 | Eval aggregate command | done | Added `crop eval --all` and suite-level JSON aggregate summaries. |
| 04 | Metrics contract | done | Added source coverage, budget efficiency, noise-token share, and metric docs. |
| 05 | Competitor adapter spec | todo | Define external vector/search adapter contract without adding hosted dependencies. |

## Success criteria

- `cargo run -p crop-cli -- eval --all --budget 160` produces deterministic JSON.
- At least four fixture cases compare `crop` and `score-top-k`.
- At least two fixture cases compare `crop`, `score-top-k`, `lexical-top-k`, and
  `hybrid-top-k`.
- The README explains what CROP wins, where it does not yet compete, and how to
  reproduce every comparison locally.
- All evaluation fixtures are covered by unit tests and CLI smoke runs.

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| Cherry-picked benchmark perception | Require multiple failure modes and publish labels/selected units in JSON. |
| Weak baselines | Enforce budget, stable tie-breaks, and identical token accounting for every baseline. |
| Fixture-only overclaiming | Document fixture scope and defer real corpus claims until adapter and corpus pulses. |
| Metric drift | Keep metric computation in `crop-core` and assert exact fixture outputs in tests. |

## Validation baseline

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- eval --all --budget 160
git diff --check
```
