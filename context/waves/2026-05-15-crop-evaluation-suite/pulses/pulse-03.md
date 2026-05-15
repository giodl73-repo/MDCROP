# Pulse 03: Eval aggregate command

## Status

Todo.

## Goal

Make the suite runnable in one command for reproducible comparison reports.

## Deliverables

- Add `crop eval --all --budget 160`.
- Emit deterministic JSON with per-case/per-method rows and aggregate metrics.
- Add a compact human table mode if it can coexist with JSON safely.
- Include method ordering and case ordering in tests.
- Document expected command output in README.

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

