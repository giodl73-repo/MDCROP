# Pulse 05: Extension profiles

## Scout commands

```powershell
cargo run -q -p crop-cli -- view --inspect --dir examples\views --strict
cargo run -q -p crop-cli -- view --inspect --file examples\views\resonance-theory-view.json -e md -e tex
```

## Decision

Maxim and Resonance showed that extension mix drives good view design. Maxim is
mostly Markdown; Resonance often needs Markdown plus TeX and BibTeX. Inspection
should expose that mix directly instead of forcing agents to infer it from sample
paths.

## Checklist

- [x] Add per-view extension counts to `crop.view-inspect.v1`.
- [x] Add aggregate extension counts to `crop.view-store-inspect.v1`.
- [x] Cover extension counts in tests.
- [x] Update README, view guide, product plan, and wave docs.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p crop-cli -- view --inspect --dir examples\views --strict
cargo run -q -p crop-cli -- view --inspect --file examples\views\resonance-theory-view.json -e md -e tex
git diff --check
```
