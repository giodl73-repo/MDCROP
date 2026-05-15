# Pulse 12: Markdown Catalog

## Goal

Generate PROOF-ready Markdown README/TOC catalog tables that carry source
metadata and link side-info.

## Scope

- Add reusable `crop-core` catalog report and Markdown rendering APIs.
- Add `crop catalog --root` and `crop catalog --view`.
- Combine the source index with frontmatter fields and backlink reports.
- Carry useful PROOF frontmatter fields such as `status`, `tags`, `title`,
  `wave`, and `version` into rendered tables when present.
- Report inbound link count, outbound link count, and orphan status per page.

## Checklist

- [x] Core catalog structs and root/view APIs.
- [x] CLI command wiring.
- [x] Positive root coverage for frontmatter plus link side-info.
- [x] Named-view filter coverage.
- [x] README, product plan, and wave tracker updates.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] PROOF/root and view smoke runs.
- [x] `git diff --check`

## Outcome

`crop catalog` emits Markdown tables that PROOF can embed directly in generated
README/TOC pages while preserving CROP's structured JSON side-info surfaces for
custom renderers.
