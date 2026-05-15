# Pulse 01: Fixture expansion

## Status

Todo.

## Goal

Expand from one missing-bridge case to a small suite of graph-native retrieval
failure modes.

## Deliverables

- Add `noisy-periphery`: high-scoring nearby noise should be excluded by a tighter
  crop boundary.
- Add `stale-duplicate`: stale high-score duplicate should lose to current or
  canonical evidence.
- Add `budget-packing`: score top-k should waste budget on one large item while
  CROP packs multiple gold units.
- Add `cross-link-chain`: explicit file links should carry CROP through
  intermediate context that top-k misses.
- Add positive/negative tests for every case.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "EvalCase|eval_fixture|score_top_k" C:\src\CROP\crates\crop-core\src\lib.rs -n
cargo run -p crop-cli -- eval --case missing-bridge --method crop --budget 160
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- eval --case noisy-periphery --method crop --budget 160
cargo run -p crop-cli -- eval --case stale-duplicate --method crop --budget 160
cargo run -p crop-cli -- eval --case budget-packing --method crop --budget 160
cargo run -p crop-cli -- eval --case cross-link-chain --method crop --budget 160
git diff --check
```

