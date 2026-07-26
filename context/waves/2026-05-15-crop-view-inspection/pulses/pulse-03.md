# Pulse 03: Strict inspect gate

## Scout commands

```powershell
cargo run -q -p mdcrop-cli -- view --inspect --dir examples\views
cargo run -q -p mdcrop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
```

## Decision

Store inspection is useful for humans, but CI and autonomous agents need a gate
that fails when any recipe is invalid. This slice keeps the JSON artifact visible
and adds a non-zero exit only when requested with `--strict`.

## Checklist

- [x] Add `--strict` flag to the view command.
- [x] Require `--strict` to be paired with `--inspect`.
- [x] Make store inspection print JSON before exiting non-zero on failures.
- [x] Leave successful store and single-file inspection behavior unchanged.
- [x] Update README, view guide, product plan, and wave docs.
- [x] Run validation gates, including a negative strict smoke.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p mdcrop-cli -- view --inspect --dir examples\views --strict
cargo run -q -p mdcrop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md --strict
# plus a temporary bad store to confirm strict exits non-zero after JSON output
git diff --check
```
