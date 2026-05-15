# Pulse 11: Frontmatter Inventory

## Goal

Expose PROOF-ready Markdown metadata side-info from CROP roots and named views.

## Scope

- Add a reusable `crop-core` frontmatter inventory API.
- Add `crop frontmatter --root` and `crop frontmatter --view`.
- Extract simple top-level YAML-style frontmatter key/value fields.
- Report per-page field maps and aggregate key counts.
- Document the command for PROOF README and HTML generation workflows.

## Checklist

- [x] Core frontmatter report structs and root/view APIs.
- [x] CLI command wiring.
- [x] Positive root coverage for metadata fields and key counts.
- [x] Named-view filter coverage.
- [x] README, product plan, and wave tracker updates.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] PROOF/root and view smoke runs.
- [x] `git diff --check`

## Outcome

`crop frontmatter` emits `crop.markdown-frontmatter.v1` JSON that downstream
tools such as PROOF can use to populate generated README tables, status/tag
summaries, and HTML sidebars from source-document metadata.
