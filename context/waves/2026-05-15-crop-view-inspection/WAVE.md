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
- `cargo fmt`, `cargo test --workspace`, inspect smoke runs, and `git diff
  --check` pass.
