# Pulse 05: Real-corpus benchmark docs

## Status

Done.

## Goal

Document how to run fair apples-to-apples CROP benchmarks, how to interpret
fixture versus real-corpus metrics, and how named views help repeat comparisons.

## Deliverables

- Add a benchmark guide covering fixture comparisons, real-corpus one-off runs,
  manifest views, aggregate reports, and competitor exports.
- Link the guide from README.
- Close the real-corpus benchmark wave.
- Keep documentation aligned with current commands and JSON schemas.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "bench --manifest|eval-import|crop.bench" C:\src\CROP -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- bench --root . --task "explain crop benchmark" --budget 12000
git diff --check
```

## Outcome

`docs/BENCHMARKS.md` now explains fair comparison rules, fixture quality
comparisons, real-corpus timing caveats, named benchmark views, aggregate
interpretation, and offline competitor export scoring.
