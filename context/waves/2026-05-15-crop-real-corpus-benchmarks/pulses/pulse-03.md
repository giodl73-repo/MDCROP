# Pulse 03: Competitor export examples

## Status

Done.

## Goal

Give external retrieval systems concrete offline JSON examples they can copy,
fill, and score through `mdcrop eval-import` without adding hosted dependencies to
MDCROP.

## Deliverables

- Add checked JSON examples for vector, lexical, hybrid, RAG orchestration, and
  graph competitor classes.
- Keep examples fixture-based and local-only.
- Add unit coverage that parses and scores every checked example.
- Document how to run all examples through `mdcrop eval-import`.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "CompetitorClass|eval_competitor_json|eval-import" C:\src\MDCROP -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- eval-import --input examples\competitors\qdrant-vector.missing-bridge.json
git diff --check
```

## Outcome

`examples/competitors/` now contains five valid offline competitor exports:
Qdrant-style vector, Tantivy/Lucene-style lexical, hybrid reranker,
LlamaIndex-style RAG orchestration, and GraphRAG/Neo4j-style graph retrieval.
Each uses the same `mdcrop.competitor-results.v1` schema and is scoreable through
`mdcrop eval-import`.
