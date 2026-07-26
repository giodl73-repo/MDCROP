# Pulse 02: Strict Policy Selection

## Objective

Let CI users gate only the status issue classes they care about while preserving
the existing default that fails on every corpus-health issue.

## Scout commands

```powershell
rg "strict|strict_pass|issue_count|orphan|duplicate" crates\mdcrop-cli\src\status.rs crates\mdcrop-cli\src\main.rs docs README.md
cargo test -p mdcrop-cli status -- --test-threads=1
```

## Implementation checklist

- [x] Add repeatable `--strict-on` values for `broken-links`, `orphan-pages`,
      and `duplicate-anchors`.
- [x] Preserve default `--strict` behavior when no policy selector is provided.
- [x] Reject `--strict-on` without `--strict` so policy flags cannot silently
      become no-ops.
- [x] Include selected policy names and strict issue totals in
      `mdcrop.corpus-status.v1`.
- [x] Add positive and negative focused tests for selected gates.
- [x] Update README/spec/product/wave documentation.

## Validation

```powershell
cargo fmt
cargo test -p mdcrop-cli status -- --test-threads=1
cargo test --workspace
cargo run -p mdcrop-cli -- status --root examples\mdloom-fixture\src --format json --strict --strict-on duplicate-anchors --output target\mdcrop-status-policy-smoke.json
git diff --check
```

