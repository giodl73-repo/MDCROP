# Pulse 02: Store inspect preflight

## Scout commands

```powershell
cargo run -q -p crop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
cargo run -q -p crop-cli -- view --list --dir examples\views
```

## Decision

Single-view inspection is useful for one recipe, but real local usage has a
store of named views under `.crop\views` or `examples\views`. This slice adds a
store-level preflight artifact instead of making operators script over files.

## Checklist

- [x] Add `crop.view-store-inspect.v1` output type.
- [x] Add core store inspection API with sorted JSON view entries.
- [x] Add `crop view --inspect --dir`.
- [x] Report per-view `ok` / `failed` status without aborting on the first bad
  recipe.
- [x] Add positive/negative store inspection test coverage.
- [x] Update README, view guide, product plan, and wave docs.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p crop-cli -- view --inspect --dir examples\views
cargo run -q -p crop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
git diff --check
```
