# Pulse 04: Metrics contract

## Status

Todo.

## Goal

Stabilize the metric vocabulary so CROP comparisons are clear and hard to game.

## Deliverables

- Add source coverage: distinct selected source IDs divided by gold source IDs.
- Add budget efficiency: gold units recovered per token used.
- Add selected-noise token share, not just selected-noise unit share.
- Add deterministic elapsed time measurement only if it is stable enough for
  local smoke comparisons; otherwise document latency as a future real-corpus
  metric.
- Document all metrics in README and pulse docs.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "gold_recall|noise_rate|token_estimate" C:\src\CROP\crates\crop-core\src\lib.rs -n
cargo run -p crop-cli -- eval --case missing-bridge --method crop --budget 160
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- eval --all --budget 160
git diff --check
```

