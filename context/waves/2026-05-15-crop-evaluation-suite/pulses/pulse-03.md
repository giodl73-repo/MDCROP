# Pulse 03: Eval aggregate command

## Status

Done.

## Goal

Make the suite runnable in one command for reproducible comparison reports.

## Deliverables

- [x] Add `crop eval --all --budget 160`.
- [x] Emit deterministic JSON with per-case/per-method rows and aggregate metrics.
- [x] Keep JSON as the only output mode for this slice to avoid unstable table
  formatting.
- [x] Include method ordering and case ordering in tests.
- [x] Document expected command output in README.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
cargo run -p crop-cli -- eval --help
rg "EvalReport" C:\src\CROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- eval --all --budget 160
git diff --check
```
