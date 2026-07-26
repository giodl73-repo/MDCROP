# Pulse 01: View inspect preflight

## Scout commands

```powershell
cargo run -q -p mdcrop-cli -- view --list --dir examples\views
cargo run -q -p mdcrop-cli -- view --file examples\views\whole-repo-view.json --query "prefix cache" -e md
```

## Decision

This slice belongs in `mdcrop-core` and `mdcrop-cli`, not only docs. Large views need
a machine-readable preflight artifact so agents can validate root/filter choices
before a full mdcrop.

## Checklist

- [x] Add `mdcrop.view-inspect.v1` output type.
- [x] Add core inspection API for `mdcrop.view.v1`.
- [x] Add `mdcrop view --inspect --file`.
- [x] Include active filters in view-list summaries.
- [x] Add positive and negative inspection tests.
- [x] Update README, view guide, product plan, and wave registry.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p mdcrop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
cargo run -q -p mdcrop-cli -- view --inspect --file examples\views\resonance-theory-view.json -e md -e tex
cargo run -q -p mdcrop-cli -- view --list --dir examples\views
git diff --check
```
