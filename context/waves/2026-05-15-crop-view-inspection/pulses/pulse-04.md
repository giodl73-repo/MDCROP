# Pulse 04: Store inspect aggregates

## Scout commands

```powershell
cargo run -q -p crop-cli -- view --inspect --dir examples\views --strict
```

## Decision

Store inspection already reports each view row, but agents need quick scope
signals without parsing all rows. This slice adds aggregate supported-source
counts to the store-level artifact while preserving the existing per-view rows.

## Checklist

- [x] Add total supported source count to `crop.view-store-inspect.v1`.
- [x] Add max supported source count to `crop.view-store-inspect.v1`.
- [x] Cover aggregates in store inspection tests.
- [x] Update README, view guide, product plan, and wave docs.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p crop-cli -- view --inspect --dir examples\views --strict
git diff --check
```
