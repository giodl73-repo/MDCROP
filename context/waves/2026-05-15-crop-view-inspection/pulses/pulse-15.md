# Pulse 15: MDLOOM Contracts

## Goal

Close the MDLOOM support issues by making MDCROP side-info outputs stable,
documented, locally testable, and better modularized.

## Scope

- Add deterministic `examples/mdloom-fixture` sources, outputs, view recipe, and
  `.mdloom/artifacts.json` manifest.
- Add golden examples for artifacts, headings, links, backlinks, frontmatter,
  and catalog output.
- Add `docs/specs` contracts for MDLOOM-facing schemas.
- Add fixture coverage that exercises all MDLOOM side-info surfaces.
- Split the MDLOOM artifact core and CLI handling into focused modules while
  preserving public API exports and CLI behavior.

## Related issues

- #1 Expose stable side-info schemas for MDLOOM consumption.
- #2 Split mdcrop-core and mdcrop-cli into focused modules as surfaces grow.
- #3 Add MDLOOM integration smoke fixtures for catalog and artifact health.

## Checklist

- [x] MDLOOM-like fixture corpus.
- [x] Golden side-info examples.
- [x] Schema/contract docs.
- [x] Fixture smoke test for all side-info surfaces.
- [x] `mdloom_artifacts` core module.
- [x] `artifacts` CLI module.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] Fixture CLI smoke runs.
- [x] `git diff --check`

## Outcome

MDLOOM can now pin MDCROP integration against a small local fixture instead of a
large real repository, and the artifact surface has an explicit module boundary
for future side-info growth.
