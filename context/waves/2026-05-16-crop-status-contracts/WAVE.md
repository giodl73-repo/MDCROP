# Wave: CROP Status Contracts

## Goal

Make `crop status` a consumer-ready contract surface, not only a generated
Markdown page, by adding stable JSON status summaries and policy-aware CI gates.

## Thesis

The previous wave made CROP useful for generated corpus health pages. Consumers
now need the same combined health view as machine-readable evidence, so CI,
TRACKER-style registries, and downstream generators can depend on one summary
without separately parsing every side-info command.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | JSON status summary | done | Added `crop status --format json` with `crop.corpus-status.v1` summary counts, issue totals, strict-pass state, docs, and tests. |
| 02 | Strict policy selection | todo | Add configurable strict policies so CI can gate on selected issue classes. |
| 03 | Status JSON goldens | todo | Add deterministic root/view JSON goldens for consumer integration tests. |
| 04 | Status contract docs | todo | Expand schema docs, README examples, and consumer integration notes for downstream adopters. |

## Success criteria

- `crop status --format markdown` preserves the existing generated page output.
- `crop status --format json` emits a stable `crop.corpus-status.v1` summary.
- The JSON summary includes the input label, title, issue total, strict-pass
  state, and per-surface count/issue rows.
- Strict status gates can be configured by issue class without weakening the
  default all-issues behavior.
- Root and view status contracts have deterministic fixtures or focused tests.
- `cargo fmt`, focused status tests, workspace tests, smoke runs, and `git diff
  --check` pass.

