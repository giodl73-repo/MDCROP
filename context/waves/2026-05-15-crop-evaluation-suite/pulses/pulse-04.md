# Pulse 04: Metrics contract

## Status

Done.

## Goal

Stabilize the metric vocabulary so MDCROP comparisons are clear and hard to game.

## Deliverables

- [x] Add source coverage: distinct selected source IDs divided by gold source IDs.
- [x] Add budget efficiency: gold units recovered per token used.
- [x] Add selected-noise token share, not just selected-noise unit share.
- [x] Add deterministic elapsed time measurement only if it is stable enough for
  local smoke comparisons; otherwise document latency as a future real-corpus
  metric.
- [x] Document all metrics in README and pulse docs.

Latency was deliberately deferred: fixture runs are too small for stable elapsed
time comparisons, so timing belongs with real-corpus benchmark pulses.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "gold_recall|noise_rate|token_estimate" C:\src\MDCROP\crates\mdcrop-core\src\lib.rs -n
cargo run -p mdcrop-cli -- eval --case missing-bridge --method mdcrop --budget 160
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- eval --all --budget 160
git diff --check
```
