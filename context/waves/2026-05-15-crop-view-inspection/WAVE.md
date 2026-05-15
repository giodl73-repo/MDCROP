# Wave: CROP View Inspection

## Goal

Make named views safer to use on large and optional corpora by adding a preflight
inspection command that validates a recipe and reports what it would scan before
running a full context crop.

## Thesis

Named views are now reusable, filterable recipes. The next product need is trust:
operators should be able to confirm root resolution, filters, and source counts
before paying the cost of ingest/scoring/cropping a large corpus.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | View inspect preflight | done | Added `crop view --inspect --file` and list metadata for active filters. |
| 02 | Store inspect preflight | done | Added `crop view --inspect --dir` with per-view ok/failed status rows. |
| 03 | Strict inspect gate | done | Added `--strict` so store inspection prints JSON and exits non-zero on failed recipes. |
| 04 | Store inspect aggregates | done | Added total and max supported source counts to store inspection reports. |
| 05 | Extension profiles | done | Added per-view and aggregate extension counts for choosing large-corpus filters. |
| 06 | Markdown index API | done | Added reusable README/TOC-style Markdown index generation for PROOF integration. |
| 07 | Markdown link audit | done | Added root/view local Markdown link audit JSON for PROOF and docs curation. |
| 08 | MDPATH link resolution | done | Upgraded link audits to validate heading fragments and `md://` targets using MDPATH. |
| 09 | Heading inventory | done | Added root/view heading inventories with anchors, `md://` section URIs, and duplicate groups. |
| 10 | Backlink side-info | done | Added root/view backlink reports with inbound links, outbound counts, and orphan-page flags. |
| 11 | Frontmatter inventory | done | Added root/view frontmatter inventories with field maps and aggregate key counts. |
| 12 | Markdown catalog | done | Added root/view catalog tables carrying frontmatter tags/status plus link side-info. |

## Success criteria

- `crop view --inspect --file <view>` emits `crop.view-inspect.v1`.
- Inspection validates schema, budget, root existence, filters, and empty matches.
- Inspection returns resolved root, active filters, source count, and sample
  source IDs without producing a context pack.
- Store inspection reports every JSON recipe in a directory with sorted per-file
  status rows.
- Strict store inspection exits non-zero when any recipe failed while still
  printing the inspection artifact.
- Store inspection includes aggregate matched-source counts for quick scope
  comparison.
- Inspection includes extension-count profiles for choosing Markdown, TeX, Bib,
  and source-code filters.
- CROP exposes reusable Markdown index primitives that downstream generators can
  depend on without adopting CROP's CLI.
- CROP exposes reusable Markdown link side-info so downstream generators can
  report local link health.
- Link side-info uses MDPATH-compatible heading and `md://` resolution for PROOF
  workflows.
- Heading inventories expose TOC/anchor primitives for PROOF and other generated
  documentation workflows.
- Backlink reports expose referenced-by and orphan-page primitives for generated
  README and TOC workflows.
- Frontmatter inventories expose metadata primitives for generated README tables,
  HTML sidebars, and status/tag summaries.
- Markdown catalogs combine source titles, frontmatter metadata, and link
  side-info into ready-to-embed README/TOC tables.
- `cargo fmt`, `cargo test --workspace`, inspect smoke runs, and `git diff
  --check` pass.
