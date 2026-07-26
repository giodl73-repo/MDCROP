# Pulse 14: MDLOOM Artifact Manifests

## Goal

Let CROP read MDLOOM generated artifact manifests and report generated-output
health for Markdown, ASCII, HTML, and related targets.

## Scope

- Add `crop artifacts --root` and `crop artifacts --manifest`.
- Read MDLOOM `.mdloom/artifacts.json` records.
- Map source paths to output paths and target/status metadata.
- Report missing outputs, stale outputs, cache status, directive counts, and
  diagnostic counts.
- Keep MDLOOM responsible for generation while CROP provides corpus intelligence
  over generated artifacts.

## Checklist

- [x] Core artifact manifest/report structs.
- [x] Root and explicit manifest APIs.
- [x] CLI command wiring.
- [x] Synthetic coverage for stale and missing outputs.
- [x] README, product plan, and wave tracker updates.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] Artifact CLI smoke run.
- [x] `git diff --check`

## Outcome

`crop artifacts` emits `crop.mdloom-artifacts.v1` JSON that downstream MDLOOM
workflows can use to audit whether generated documentation outputs still match
their sources.
