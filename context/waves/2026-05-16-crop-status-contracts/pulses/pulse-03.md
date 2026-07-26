# Pulse 03: Status JSON Goldens

## Objective

Add deterministic root and view JSON goldens for `mdcrop.corpus-status.v1` so
consumer integration tests can detect accidental contract drift.

## Scout commands

```powershell
rg "golden|mdloom_fixture|include_str" crates\mdcrop-core\src\lib.rs crates\mdcrop-cli\src\status.rs examples
cargo test -p mdcrop-cli status -- --test-threads=1
```

## Implementation checklist

- [x] Add a root status JSON golden under `examples\mdloom-fixture\golden`.
- [x] Add a view status JSON golden under `examples\mdloom-fixture\golden`.
- [x] Add focused tests that compare generated `mdcrop.corpus-status.v1` reports
      against those golden files.
- [x] Normalize input labels in tests so goldens are deterministic across local
      checkout paths.
- [x] Update wave and docs references.
- [x] Run `cargo fmt`, focused status tests, workspace tests, smoke runs, and
      `git diff --check`.

## Validation

```powershell
cargo fmt
cargo test -p mdcrop-cli status -- --test-threads=1
cargo test --workspace
cargo run -p mdcrop-cli -- status --root examples\mdloom-fixture\src --format json --output target\mdcrop-status-smoke.json
git diff --check
```

