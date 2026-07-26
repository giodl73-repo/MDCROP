# Wave: MDCROP Status Contracts

## Goal

Make `mdcrop status` a consumer-ready contract surface, not only a generated
Markdown page, by adding stable JSON status summaries and policy-aware CI gates.

## Thesis

The previous wave made MDCROP useful for generated corpus health pages. Consumers
now need the same combined health view as machine-readable evidence, so CI,
TRACKER-style registries, and downstream generators can depend on one summary
without separately parsing every side-info command.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | JSON status summary | done | Added `mdcrop status --format json` with `mdcrop.corpus-status.v1` summary counts, issue totals, strict-pass state, docs, and tests. |
| 02 | Strict policy selection | done | Added repeatable `--strict-on` policy selectors for broken links, orphan pages, and duplicate anchors while preserving default all-issue strict gates. |
| 03 | Status JSON goldens | done | Added deterministic root and view `mdcrop.corpus-status.v1` goldens under `examples/mdloom-fixture/golden`. |
| 04 | Status contract docs | done | Expanded README, schema, and consumer workflow docs for status JSON, strict policy gates, and golden contracts. |

## Success criteria

- `mdcrop status --format markdown` preserves the existing generated page output.
- `mdcrop status --format json` emits a stable `mdcrop.corpus-status.v1` summary.
- The JSON summary includes the input label, title, issue total, strict-pass
  state, and per-surface count/issue rows.
- Strict status gates can be configured by issue class without weakening the
  default all-issues behavior.
- Root and view status contracts have deterministic fixtures or focused tests.
- `cargo fmt`, focused status tests, workspace tests, smoke runs, and `git diff
  --check` pass.

