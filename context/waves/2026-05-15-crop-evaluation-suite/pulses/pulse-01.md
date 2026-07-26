# Pulse 01: Fixture expansion

## Status

Done.

## Goal

Expand from one missing-bridge case to a small suite of graph-native retrieval
failure modes.

## Deliverables

- [x] Add `noisy-periphery`: high-scoring nearby noise should be excluded by a tighter
  mdcrop boundary.
- [x] Add `stale-duplicate`: stale high-score duplicate should lose to current or
  canonical evidence.
- [x] Add `budget-packing`: score top-k should waste budget on one large item while
  MDCROP packs multiple gold units.
- [x] Add `cross-link-chain`: explicit file links should carry MDCROP through
  intermediate context that top-k misses.
- [x] Add positive/negative tests for every case.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "EvalCase|eval_fixture|score_top_k" C:\src\MDCROP\crates\mdcrop-core\src\lib.rs -n
cargo run -p mdcrop-cli -- eval --case missing-bridge --method mdcrop --budget 160
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- eval --case noisy-periphery --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case stale-duplicate --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case budget-packing --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case cross-link-chain --method mdcrop --budget 160
git diff --check
```
