# Pulse 02: Benchmark manifest inputs

## Status

Done.

## Goal

Make real-corpus benchmarks repeatable by allowing named benchmark/view cases to
be stored in a JSON manifest and run as one suite.

## Deliverables

- Add `BenchManifest`, `BenchManifestCase`, and `BenchManifestReport` types.
- Add manifest parsing with schema checks and empty-manifest rejection.
- Add `mdcrop bench --manifest` with roots resolved relative to the manifest file.
- Preserve one-off `mdcrop bench --root --task` usage.
- Document manifest shape and view labels.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "BenchReport|bench_pack|Bench" C:\src\MDCROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- bench --root . --task "explain mdcrop benchmark" --budget 12000
cargo run -p mdcrop-cli -- bench --manifest benchmark-manifest.json
git diff --check
```

## Outcome

Benchmark manifests now support named reusable views over local data. Each case
declares an ID, root, task, token budget, seed, and optional view label; MDCROP
runs every case and returns `mdcrop.bench-report.v1` JSON with one benchmark
report per case.
