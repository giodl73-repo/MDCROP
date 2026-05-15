# Pulse 10: Backlink Side-Info

## Goal

Expose PROOF-ready inbound-reference and orphan-page side-info from CROP roots
and named views.

## Scope

- Add a reusable `crop-core` backlink report API.
- Add `crop backlinks --root` and `crop backlinks --view`.
- Reuse MDPATH-compatible local link resolution from the Markdown link audit.
- Report per-page inbound links, outbound link counts, and orphan flags.
- Document the command for PROOF README/TOC generation workflows.

## Checklist

- [x] Core backlink report structs and root/view APIs.
- [x] CLI command wiring.
- [x] Positive root coverage for inbound links and orphan pages.
- [x] Named-view filter coverage.
- [x] README, product plan, and wave tracker updates.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] PROOF/root and view smoke runs.
- [x] `git diff --check`

## Outcome

`crop backlinks` emits `crop.markdown-backlinks.v1` JSON that downstream tools
such as PROOF can use to annotate generated indexes with referenced-by context
and identify Markdown pages that no other local page points at.
