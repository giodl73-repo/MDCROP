# Pulse 01: Local bench command

## Status

Done.

## Goal

Add a local benchmark surface that runs ingest, task scoring, and mdcrop packing on
a real root and emits stage timing plus pack summary metrics.

## Deliverables

- Add `BenchReport` / `BenchTimings` structs and a `bench_pack` API.
- Add `mdcrop bench --root --task --budget --seed`.
- Keep benchmark output local-only and dependency-free.
- Add unit coverage for benchmark reports on a temporary Markdown corpus.
- Document benchmark usage and timing caveats.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "Pack|Eval|BenchReport" C:\src\MDCROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- bench --root . --task "explain mdcrop benchmark" --budget 12000
git diff --check
```

## Outcome

`mdcrop bench` now emits `mdcrop.bench.v1` JSON with corpus counts, selected pack
summary metrics, and elapsed milliseconds for ingest, scoring, mdcrop, and total
runtime. The API and CLI share the same local ingest/scoring/mdcrop path as
`mdcrop pack`; tests cover a temporary Markdown corpus without hosted services.
