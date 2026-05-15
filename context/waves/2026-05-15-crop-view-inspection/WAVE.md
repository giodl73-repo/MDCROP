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
| 13 | Frontmatter query views | done | Added OData-style `frontmatter_query` view filters over tags and arbitrary metadata fields. |
| 14 | PROOF artifact manifests | done | Added artifact manifest reports for generated output existence, staleness, and diagnostics. |
| 15 | PROOF contracts | done | Added deterministic PROOF fixtures, schema docs/goldens, and a focused artifact module split. |
| 16 | Artifact status tables | done | Added Markdown output for artifact health reports. |
| 17 | Side-info status tables | done | Added Markdown output for links, backlinks, frontmatter, and heading reports. |
| 18 | Corpus status page | done | Added `crop status` to compose catalog and side-info tables for any Markdown corpus. |
| 19 | View-backed status page | done | Added `crop status --view` for tag/frontmatter-filtered status pages. |
| 20 | Strict corpus status | done | Added `crop status --strict` to render then fail on corpus-health issues. |

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
- Views can filter Markdown sources by frontmatter predicates such as tag
  membership, status, wave, version, or other top-level metadata fields.
- Artifact reports map PROOF source files to generated outputs and flag missing
  or stale generated Markdown/ASCII/HTML surfaces.
- PROOF side-info contracts have deterministic fixtures, documented schemas, and
  golden examples for integration tests.
- The PROOF artifact surface has focused core and CLI modules while preserving
  existing public API and CLI behavior.
- Artifact health can render as a Markdown table for generated README/status
  pages.
- Link, backlink, frontmatter, and heading side-info can render as Markdown
  tables for generated README/status pages while preserving JSON defaults.
- Any Markdown corpus can generate one status page that composes catalog and
  side-info sections.
- Status pages can use named views, including `frontmatter_query` filters, to
  generate tag/status-specific pages.
- Status pages can run in strict mode for CI gating on broken links, orphan
  pages, or duplicate heading anchors while still rendering output.
- `cargo fmt`, `cargo test --workspace`, inspect smoke runs, and `git diff
  --check` pass.
