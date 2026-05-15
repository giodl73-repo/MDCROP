# Pulse 02: Baseline methods

## Status

Todo.

## Goal

Add fair local baselines that approximate common index behavior without external
services.

## Deliverables

- Add `lexical-top-k` using deterministic task-term overlap.
- Add `hybrid-top-k` combining lexical score and fixture relevance score.
- Keep `score-top-k` as the rank-only baseline.
- Ensure every method uses the same token budget and tie-breaking contract.
- Add tests where lexical/hybrid win or tie where graph structure is irrelevant,
  so the suite is not biased to CROP-only wins.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "EvalMethod|score_top_k_selection|task_terms" C:\src\CROP\crates\crop-core\src\lib.rs -n
cargo test --workspace
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- eval --case missing-bridge --method lexical-top-k --budget 160
cargo run -p crop-cli -- eval --case missing-bridge --method hybrid-top-k --budget 160
git diff --check
```

