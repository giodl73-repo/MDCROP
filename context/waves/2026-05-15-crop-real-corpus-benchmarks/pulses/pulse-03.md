# Pulse 03: Competitor export examples

## Status

Done.

## Goal

Give external retrieval systems concrete offline JSON examples they can copy,
fill, and score through `crop eval-import` without adding hosted dependencies to
CROP.

## Deliverables

- Add checked JSON examples for vector, lexical, hybrid, RAG orchestration, and
  graph competitor classes.
- Keep examples fixture-based and local-only.
- Add unit coverage that parses and scores every checked example.
- Document how to run all examples through `crop eval-import`.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "CompetitorClass|eval_competitor_json|eval-import" C:\src\CROP -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- eval-import --input examples\competitors\qdrant-vector.missing-bridge.json
git diff --check
```

## Outcome

`examples/competitors/` now contains five valid offline competitor exports:
Qdrant-style vector, Tantivy/Lucene-style lexical, hybrid reranker,
LlamaIndex-style RAG orchestration, and GraphRAG/Neo4j-style graph retrieval.
Each uses the same `crop.competitor-results.v1` schema and is scoreable through
`crop eval-import`.
