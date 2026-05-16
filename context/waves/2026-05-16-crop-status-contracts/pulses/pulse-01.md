# Pulse 01: JSON Status Summary

## Objective

Add a stable machine-readable summary mode for `crop status` while preserving
Markdown as the default generated-page output.

## Scout commands

```powershell
rg "crop status|corpus status|status --strict" README.md docs crates
rg "StatusDocs|render_status_page|strict" crates\crop-cli\src\status.rs crates\crop-cli\src\main.rs
cargo test -p crop-cli status -- --test-threads=1
```

## Implementation checklist

- [x] Add `--format markdown|json` to `crop status`.
- [x] Preserve Markdown as the default.
- [x] Add a `crop.corpus-status.v1` JSON summary with issue totals and
      per-surface counts.
- [x] Add focused positive coverage for JSON output.
- [x] Update README/spec/product/wave documentation.
- [x] Run `cargo fmt`, focused status tests, JSON smoke output, and `git diff
      --check`.

## Validation

```powershell
cargo fmt
cargo test -p crop-cli status -- --test-threads=1
cargo run -p crop-cli -- status --root examples\proof-fixture\src --format json --output target\crop-status-smoke.json
git diff --check
```

