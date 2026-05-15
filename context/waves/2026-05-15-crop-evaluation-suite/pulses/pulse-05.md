# Pulse 05: Competitor adapter spec

## Status

Todo.

## Goal

Prepare for real competitor comparisons without adding hosted dependencies or
vendor lock-in.

## Deliverables

- Define an adapter trait/JSON contract for external retrieval results:
  selected unit IDs, scores, token use, and method metadata.
- Add an import path for saved competitor results so vector/search systems can
  be evaluated offline against the same labels.
- Document candidate competitors: Qdrant/Chroma for vector, Lucene/Tantivy for
  lexical, LlamaIndex/LangChain-style retrievers for RAG orchestration, and
  GraphRAG/Neo4j-style graph systems for graph comparison.
- Keep the first implementation local-only; no network calls in tests.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "EvalMethod|EvalReport" C:\src\CROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
git diff --check
```
