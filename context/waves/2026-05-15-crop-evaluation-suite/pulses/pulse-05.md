# Pulse 05: Competitor adapter spec

## Status

Done.

## Goal

Prepare for real competitor comparisons without adding hosted dependencies or
vendor lock-in.

## Deliverables

- Define an adapter trait/JSON contract for external retrieval results:
  selected unit IDs, scores, token use, and method metadata. **Done** via
  `CompetitorEvalInput`, `CompetitorMethodMetadata`, and
  `CompetitorSelectedUnit`, plus the `CompetitorAdapter` trait for exporters.
- Add an import path for saved competitor results so vector/search systems can
  be evaluated offline against the same labels. **Done** via
  `eval_competitor_json`, `eval_competitor_results`, and
  `mdcrop eval-import --input`.
- Document candidate competitors: Qdrant/Chroma for vector, Lucene/Tantivy for
  lexical, LlamaIndex/LangChain-style retrievers for RAG orchestration, and
  GraphRAG/Neo4j-style graph systems for graph comparison. **Done** in
  `README.md`.
- Keep the first implementation local-only; no network calls in tests. **Done**.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "EvalMethod|EvalReport" C:\src\MDCROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

## Outcome

External systems can now export saved fixture selections using
`mdcrop.competitor-results.v1`; MDCROP validates selected unit IDs, rejects duplicate
or unknown units, and scores the imported results with the same metric contract
used by local fixture methods.
