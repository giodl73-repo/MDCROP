# Pulse 15: PROOF Contracts

## Goal

Close the PROOF support issues by making MDCROP side-info outputs stable,
documented, locally testable, and better modularized.

## Scope

- Add deterministic `examples/proof-fixture` sources, outputs, view recipe, and
  `.proof/artifacts.json` manifest.
- Add golden examples for artifacts, headings, links, backlinks, frontmatter,
  and catalog output.
- Add `docs/specs` contracts for PROOF-facing schemas.
- Add fixture coverage that exercises all PROOF side-info surfaces.
- Split the PROOF artifact core and CLI handling into focused modules while
  preserving public API exports and CLI behavior.

## Related issues

- #1 Expose stable side-info schemas for PROOF consumption.
- #2 Split mdcrop-core and mdcrop-cli into focused modules as surfaces grow.
- #3 Add PROOF integration smoke fixtures for catalog and artifact health.

## Checklist

- [x] PROOF-like fixture corpus.
- [x] Golden side-info examples.
- [x] Schema/contract docs.
- [x] Fixture smoke test for all side-info surfaces.
- [x] `proof_artifacts` core module.
- [x] `artifacts` CLI module.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] Fixture CLI smoke runs.
- [x] `git diff --check`

## Outcome

PROOF can now pin MDCROP integration against a small local fixture instead of a
large real repository, and the artifact surface has an explicit module boundary
for future side-info growth.
