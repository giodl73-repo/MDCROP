# Pulse 05: View docs and examples

## Scout commands

```powershell
Get-ChildItem examples\views
cargo run -p crop-cli -- view --list --dir examples\views
```

## Decision

This slice is fixture/docs only. The view engine and runtime filter support
landed in pulse 04, so pulse 05 closes the loop with reusable recipes and
operator documentation.

## Checklist

- [x] Add docs-only recipe.
- [x] Add whole-repo recipe.
- [x] Add troubleshooting recipe.
- [x] Add Maxim local topic recipe.
- [x] Add Resonance local theory recipe.
- [x] Update README and view guide.
- [x] Mark named-views wave complete.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p crop-cli -- view --list --dir examples\views
cargo run -q -p crop-cli -- view --file examples\views\whole-repo-view.json --query "prefix cache" -e md
cargo run -q -p crop-cli -- view --file examples\views\troubleshooting-view.json --query "diagnose failing view" -e md -e rs
cargo run -q -p crop-cli -- view --file examples\views\maxim-computing-view.json --query "computing systems" -e md
cargo run -q -p crop-cli -- view --file examples\views\resonance-theory-view.json --query "theory context" -e md -e tex
git diff --check
```
