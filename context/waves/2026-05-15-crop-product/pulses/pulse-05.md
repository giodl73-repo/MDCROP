# Pulse 05: Evaluation fixtures

## Status

In progress.

## Goal

Compare MDCROP against ordinary top-k retrieval fixtures using the same corpus,
scores, token budget, and gold labels.

## Delivered first slice

- [x] Add `EvalCase::MissingBridge` and `EvalMethod::{Mdcrop, ScoreTopK}`.
- [x] Add `eval_fixture(case, method, budget)` in `mdcrop-core`.
- [x] Add `mdcrop eval --case missing-bridge --method mdcrop|score-top-k`.
- [x] Emit JSON metrics for selected units, token use, gold recall, bridge
  recall, noise rate, and quality score.
- [x] Add coverage proving MDCROP recovers the bridge while score top-k misses it
  and includes stale noise under the same budget.

## Remaining fixtures

- Add noisy-periphery, stale-duplicate, budget-packing, and cross-link-chain
  cases.
- Add optional lexical/vector baselines once the scoring and embedding surfaces
  exist.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- eval --case missing-bridge --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case missing-bridge --method score-top-k --budget 160
git diff --check
```
