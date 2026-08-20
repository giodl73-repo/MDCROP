# Pulse 04: Status Contract Docs

## Objective

Make the completed `mdcrop status` contract discoverable for downstream consumers
that need generated status pages, machine-readable summaries, CI gates, and
deterministic examples.

## Scout commands

```powershell
rg "mdcrop status|mdcrop.corpus-status.v1|strict-on|status-root" README.md docs context\waves
cargo test -p mdcrop-cli status -- --test-threads=1
```

## Implementation checklist

- [x] Expand README examples to show Markdown, JSON, strict gates, selected
      strict policies, and deterministic goldens.
- [x] Expand `docs\specs\corpus-status.md` with CLI option behavior, JSON field
      semantics, surface rows, and example output.
- [x] Expand consumer integration notes with CI and registry usage guidance.
- [x] Mark the status-contracts wave complete in `PHASES.md`.
- [x] Run `cargo fmt`, focused status tests, workspace tests, smoke runs, and
      `git diff --check`.

## Validation

```powershell
cargo fmt
cargo test -p mdcrop-cli status -- --test-threads=1
cargo test --workspace
cargo run -p mdcrop-cli -- status --root examples\proof-fixture\src --format json --output target\mdcrop-status-smoke.json
git diff --check
```

