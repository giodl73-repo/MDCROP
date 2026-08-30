# MDCROP Invariants

## MDCROP-I-01: Fixture Graph Selection Remains Deterministic

**Claim:** The built-in fixture graph emits deterministic selected units,
token estimates, boundary counts, and quality scores for the same budget.

**Status:** VERIFIED

**Why it matters:** Deterministic fixtures make context-boundary regressions
reviewable and reproducible across portfolio research passes.

**Test:** `cargo run -p mdcrop-cli -- fixture --budget 170` and
`cargo test --workspace`.

## MDCROP-I-02: View Inspection Reports Stable Contracts

**Claim:** View inspection emits `mdcrop.view-inspect.v1` and store inspection
emits `mdcrop.view-store-inspect.v1` with per-view status, counts, extension
profiles, and deterministic ordering.

**Status:** VERIFIED

**Why it matters:** Downstream agents need a cheap preflight before running a full
context pack over large or generated corpora.

**Test:** `cargo run -p mdcrop-cli -- view --inspect --dir examples\views
--strict` and `cargo test --workspace`.

## MDCROP-I-03: Corpus Status JSON Keeps Its Schema Surface

**Claim:** `mdcrop status --format json` emits `mdcrop.corpus-status.v1` with
input, issue counts, strict policy names, strict pass/fail state, and surface
rows.

**Status:** VERIFIED

**Why it matters:** PROOF-style generators and portfolio trackers can consume
corpus-health output without scraping Markdown.

**Test:** `cargo run -p mdcrop-cli -- status --view
examples\proof-fixture\proof-ready-view.json --format json` and
`cargo test --workspace`.

## MDCROP-I-04: Benchmark Fixtures Compare Methods Through One Scorer

**Claim:** Built-in retrieval evaluations and imported competitor results use
the same quality metrics for gold recall, bridge recall, noise, coverage, and
budget efficiency.

**Status:** VERIFIED

**Why it matters:** Benchmark claims should measure MDCROP's graph-native
boundary value rather than fixture-specific presentation differences.

**Test:** `cargo run -p mdcrop-cli -- eval --case budget-packing --method
mdcrop --budget 160`, `powershell -NoProfile -ExecutionPolicy Bypass -File
examples\bench-fixtures\run.ps1 -Size small -Runs 1`, and
`cargo test --workspace`.

## MDCROP-I-05: CLI Code Must Stay Clippy-Clean Under All Targets

**Claim:** Workspace code, including test targets, passes clippy with warnings
denied.

**Status:** VERIFIED

**Why it matters:** Strict static analysis catches contract drift and API shape
debt that ordinary tests can miss.

**Test:** `cargo clippy --workspace --all-targets -- -D warnings`.
