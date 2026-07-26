# Pulse 02: Baseline methods

## Status

Done.

## Goal

Add fair local baselines that approximate common index behavior without external
services.

## Deliverables

- [x] Add `lexical-top-k` using deterministic task-term overlap.
- [x] Add `hybrid-top-k` combining lexical score and fixture relevance score.
- [x] Keep `score-top-k` as the rank-only baseline.
- [x] Ensure every method uses the same token budget and tie-breaking contract.
- [x] Add tests where lexical/hybrid win or tie where graph structure is irrelevant,
  so the suite is not biased to MDCROP-only wins.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "EvalMethod|score_top_k_selection|task_terms" C:\src\MDCROP\crates\mdcrop-core\src\lib.rs -n
cargo test --workspace
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- eval --case missing-bridge --method lexical-top-k --budget 160
cargo run -p mdcrop-cli -- eval --case missing-bridge --method hybrid-top-k --budget 160
git diff --check
```
