# Pulse 04: Aggregate benchmark report

## Status

Done.

## Goal

Summarize benchmark manifest rows across roots, tasks, budgets, and views so
repeated runs have a quick suite-level performance and quality readout.

## Deliverables

- Add `BenchAggregate` to `crop.bench-report.v1`.
- Include corpus totals, selected-token totals, mean quality, mean stage timings,
  and max total runtime.
- Keep per-case benchmark rows unchanged for detailed inspection.
- Add unit coverage for aggregate totals and means.
- Document aggregate fields in README.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "BenchManifestReport|BenchAggregate|bench_manifest" C:\src\CROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- bench --manifest benchmark-manifest.json
git diff --check
```

## Outcome

Manifest benchmark output now contains an `aggregate` block with method `crop`,
case count, total corpus/selection counts, mean selected size, mean quality, mean
stage timings, and max total runtime. This keeps detailed rows while making the
suite-level result easy to compare across local runs.
