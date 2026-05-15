# Pulse 02: Benchmark manifest inputs

## Status

Done.

## Goal

Make real-corpus benchmarks repeatable by allowing named benchmark/view cases to
be stored in a JSON manifest and run as one suite.

## Deliverables

- Add `BenchManifest`, `BenchManifestCase`, and `BenchManifestReport` types.
- Add manifest parsing with schema checks and empty-manifest rejection.
- Add `crop bench --manifest` with roots resolved relative to the manifest file.
- Preserve one-off `crop bench --root --task` usage.
- Document manifest shape and view labels.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "BenchReport|bench_pack|Bench" C:\src\CROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- bench --root . --task "explain crop benchmark" --budget 12000
cargo run -p crop-cli -- bench --manifest benchmark-manifest.json
git diff --check
```

## Outcome

Benchmark manifests now support named reusable views over local data. Each case
declares an ID, root, task, token budget, seed, and optional view label; CROP
runs every case and returns `crop.bench-report.v1` JSON with one benchmark
report per case.
