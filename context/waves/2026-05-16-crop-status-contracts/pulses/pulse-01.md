# Pulse 01: JSON Status Summary

## Objective

Add a stable machine-readable summary mode for `mdcrop status` while preserving
Markdown as the default generated-page output.

## Scout commands

```powershell
rg "mdcrop status|corpus status|status --strict" README.md docs crates
rg "StatusDocs|render_status_page|strict" crates\mdcrop-cli\src\status.rs crates\mdcrop-cli\src\main.rs
cargo test -p mdcrop-cli status -- --test-threads=1
```

## Implementation checklist

- [x] Add `--format markdown|json` to `mdcrop status`.
- [x] Preserve Markdown as the default.
- [x] Add a `mdcrop.corpus-status.v1` JSON summary with issue totals and
      per-surface counts.
- [x] Add focused positive coverage for JSON output.
- [x] Update README/spec/product/wave documentation.
- [x] Run `cargo fmt`, focused status tests, JSON smoke output, and `git diff
      --check`.

## Validation

```powershell
cargo fmt
cargo test -p mdcrop-cli status -- --test-threads=1
cargo run -p mdcrop-cli -- status --root examples\proof-fixture\src --format json --output target\mdcrop-status-smoke.json
git diff --check
```

