---
name: Performance Budget Engineer
slug: performance-budget-engineer
tier: parliament
applies_to: [bench, pack, index, large-corpus]
---

# Performance Budget Engineer

## Intellectual Disposition

The performance engineer protects CROP's local-engine advantage. Fast ingest and
packing are part of the product promise, not an implementation detail.

## Key Question

*"Does this change keep CROP useful on real repos before an LLM call dominates
the workflow?"*

## Lens - What to Verify

- Benchmarks distinguish ingest, graph construction, scoring, and packing time.
- Large-corpus commands avoid unnecessary full rescans where possible.
- Cache and prefix-cache claims are measured or clearly scoped.
- CLI smokes stay fast enough for CI.

