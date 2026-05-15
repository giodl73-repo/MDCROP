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

## Success criteria

- `crop view --inspect --file <view>` emits `crop.view-inspect.v1`.
- Inspection validates schema, budget, root existence, filters, and empty matches.
- Inspection returns resolved root, active filters, source count, and sample
  source IDs without producing a context pack.
- Store inspection reports every JSON recipe in a directory with sorted per-file
  status rows.
- `cargo fmt`, `cargo test --workspace`, inspect smoke runs, and `git diff
  --check` pass.
