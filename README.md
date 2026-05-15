# CROP

**Graph-cut context cropping for AI agents.**

CROP trims scattered evidence into focused, provenance-aware context packs from
repos, notes, issues, chats, and documents. Most RAG systems retrieve chunks;
CROP draws the boundary around the right context before an agent answers.

## Why CROP

- **Context boundary**: graph-cut style selection keeps relevant neighborhoods
  and excludes noisy periphery.
- **Missing bridges**: path and connector search can pull in the file, note, or
  issue that links otherwise separate evidence clusters.
- **Provenance first**: context packs carry source IDs, scores, and boundary
  reasons instead of anonymous snippets.
- **Rust local engine**: fast ingest, graph construction, scoring, and packing
  before slower LLM calls begin.

## Commands

Emit the deterministic bridge/noise fixture:

```powershell
cargo run -p crop-cli -- fixture --budget 170
```

Pack a local file or directory into a task-focused JSON context crop:

```powershell
cargo run -p crop-cli -- pack --root C:\src\CROP --task "explain graph context boundary" --budget 12000
```

Add `--output context.crop.json` to write the pack to disk instead of stdout.

Generate a Markdown README/TOC-style source index:

```powershell
cargo run -p crop-cli -- index --root C:\src\proof -e md -e html --exclude-dir target
cargo run -p crop-cli -- index --view examples\views\whole-repo-view.json --output INDEX.md
```

`crop index` emits Markdown tables with an extension profile and one row per
source path, title, directory, extension, and outbound Markdown-link count. It is
designed as a reusable `crop-core` surface for tools such as PROOF, which can own
its compile/render pipeline while depending on CROP for corpus index primitives.

Audit local Markdown links as structured side-info:

```powershell
cargo run -p crop-cli -- links --root C:\src\proof -e md --exclude-dir target
cargo run -p crop-cli -- links --view examples\views\whole-repo-view.json
```

`crop links` emits `crop.markdown-link-audit.v1` JSON with source, target,
status, resolved source, error details, and aggregate ok/broken counts. It uses
MDPATH-compatible resolution for `md://` URIs and heading fragments, while
ignoring external links so PROOF can focus this report on local corpus health.

Run a persisted named view recipe:

```powershell
cargo run -p crop-cli -- view --file examples\views\docs-view.json
```

View files use `crop.view.v1` JSON:

```json
{
  "schema_version": "crop.view.v1",
  "name": "crop-benchmark-docs-view",
  "root": "..\\..\\docs",
  "task": "explain CROP benchmark and view workflows",
  "token_budget": 12000,
  "seed": 0,
  "include_extensions": ["md"],
  "exclude_dirs": []
}
```

Relative roots resolve from the view file location. The output is
`crop.view-run.v1` JSON containing the view metadata and produced context pack.

List persisted views in a store:

```powershell
cargo run -p crop-cli -- view --list --dir examples\views
```

By convention, local user views live in `.crop\views`; `--dir` can point at any
directory of `crop.view.v1` JSON files. Non-JSON files are ignored and output is
sorted by file name.

Checked example views live under `examples\views\`:

| File | Use |
|------|-----|
| `docs-view.json` | CROP docs-only context. |
| `whole-repo-view.json` | Full CROP repository architecture context. |
| `troubleshooting-view.json` | Local debugging context for view, benchmark, and import failures. |
| `maxim-computing-view.json` | Local `C:\src\maxim\computing` topic view. |
| `resonance-theory-view.json` | Local `C:\src\resonance\guides\theory` research view. |

Inspect a recipe before running a full crop:

```powershell
cargo run -p crop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
cargo run -p crop-cli -- view --inspect --dir examples\views
cargo run -p crop-cli -- view --inspect --dir examples\views --strict
```

This returns `crop.view-inspect.v1` JSON with the resolved root, active filters,
supported source count, extension counts, and a small source sample. It validates
schemas, budgets, roots, and empty filters without scoring or selecting context
units. Directory inspection returns `crop.view-store-inspect.v1` with one status
row per JSON view recipe plus aggregate source-count and extension-count totals.
Add `--strict` to make CI or agent workflows exit non-zero if any store recipe
reports `failed`; the JSON report is still printed first.

Emit a prefix-cache-aware named view:

```powershell
cargo run -p crop-cli -- view --file examples\views\docs-view.json --prefix-cache generic
```

This returns `crop.view-prefix-cache.v1` JSON with the view name and a
`PrefixCachedPack`. For repeated runs over the same view root, the prefix hash is
stable when only the task changes; task, seed, and budget stay in `task_tail`.

Views can also be searched with runtime parameters:

```powershell
cargo run -p crop-cli -- view --file examples\views\docs-view.json --query "prefix cache" -e md
```

`--query` overrides the view task for one run, `-e/--extension` restricts ingest
to specific file types, and `--exclude-dir` skips matching directory basenames.
See [`docs\VIEWS.md`](docs/VIEWS.md) for large-corpus view candidates for
`C:\src\maxim` and `C:\src\resonance`.

Emit a prefix-cache-aware pack:

```powershell
cargo run -p crop-cli -- pack --root C:\src\CROP --task "explain graph context boundary" --budget 12000 --prefix-cache generic
```

Prefix-cache output separates stable corpus metadata from the selected crop and
task tail:

- `prefix`: stable schema, source index, content hashes, counts, and graph edge
  count.
- `cache_key` / `prefix_hash`: deterministic hash of the prefix block.
- `body`: task-specific selected crop units.
- `task_tail`: volatile task, seed, and budget.

When only `--task` changes against the same root, the prefix hash remains stable
so provider-side prompt prefix caching can reuse the prefix bytes.

Run apple-to-apple fixture evaluations:

```powershell
cargo run -p crop-cli -- eval --case missing-bridge --method crop --budget 160
cargo run -p crop-cli -- eval --case missing-bridge --method score-top-k --budget 160
cargo run -p crop-cli -- eval --case missing-bridge --method lexical-top-k --budget 160
cargo run -p crop-cli -- eval --case missing-bridge --method hybrid-top-k --budget 160
cargo run -p crop-cli -- eval --case noisy-periphery --method crop --budget 160
cargo run -p crop-cli -- eval --case stale-duplicate --method crop --budget 160
cargo run -p crop-cli -- eval --case budget-packing --method crop --budget 160
cargo run -p crop-cli -- eval --case cross-link-chain --method crop --budget 160
cargo run -p crop-cli -- eval --all --budget 160
```

The fixture suite uses the same units, relevance scores, token budget, and gold
labels for each method. Current cases cover missing bridges, noisy periphery,
stale duplicates, budget packing, and cross-link chains. Current methods are
`crop`, `score-top-k`, `lexical-top-k`, and `hybrid-top-k`. The `--all` command
emits deterministic JSON with ordered cases, methods, per-run rows, and
per-method aggregate means.

Evaluation metrics:

| Metric | Meaning |
|--------|---------|
| `gold_recall` | Gold evidence units selected divided by total gold units. |
| `bridge_recall` | Required bridge units selected divided by total bridge units. |
| `noise_rate` | Selected noise units divided by selected units. |
| `source_coverage` | Distinct selected gold source IDs divided by distinct gold source IDs. |
| `budget_efficiency` | Gold units recovered per selected token. |
| `noise_token_share` | Selected noise tokens divided by selected tokens. |
| `quality_score` | Method-specific scalar quality summary for quick comparison. |

Latency is intentionally not part of fixture metrics yet; it should be measured
on real corpora where filesystem and process noise can be handled separately.

Score saved external competitor results offline:

```powershell
cargo run -p crop-cli -- eval-import --input competitor-results.json
```

The competitor JSON contract is intentionally local-only and dependency-free:

```json
{
  "schema_version": "crop.competitor-results.v1",
  "case": "missing-bridge",
  "token_budget": 160,
  "method": {
    "method_id": "qdrant-demo",
    "competitor_class": "vector",
    "implementation": "qdrant",
    "version": null,
    "notes": "offline export; no network call during CROP eval"
  },
  "selected_units": [
    { "unit_id": "task-seed", "score": 0.99, "token_estimate": 80 },
    { "unit_id": "target-note", "score": 0.98, "token_estimate": 40 }
  ]
}
```

Candidate competitor classes are vector databases such as Qdrant and Chroma,
lexical/search indexes such as Lucene and Tantivy, hybrid retrievers, RAG
orchestrators such as LlamaIndex and LangChain, and graph systems such as
GraphRAG or Neo4j-backed retrieval. Their exported unit IDs are scored against
the same fixture labels, canonical token counts, and metrics as CROP and the
local baselines.

Checked export examples live under `examples\competitors\`:

| File | Competitor class | Fixture case |
|------|------------------|--------------|
| `qdrant-vector.missing-bridge.json` | Vector | `missing-bridge` |
| `tantivy-lexical.stale-duplicate.json` | Lexical | `stale-duplicate` |
| `hybrid-reranker.budget-packing.json` | Hybrid | `budget-packing` |
| `llamaindex-rag.noisy-periphery.json` | RAG orchestration | `noisy-periphery` |
| `graphrag-neo4j.cross-link-chain.json` | Graph | `cross-link-chain` |

Run any example with:

```powershell
cargo run -p crop-cli -- eval-import --input examples\competitors\qdrant-vector.missing-bridge.json
```

Benchmark CROP on a local repository or document tree:

```powershell
cargo run -p crop-cli -- bench --root C:\src\CROP --task "explain graph context boundary" --budget 12000
```

`crop bench` runs the same local ingest, scoring, and crop stages as `crop pack`
but emits summary JSON instead of the full context body. The report includes
source/unit/edge counts, selected token counts, boundary and quality metrics,
and elapsed milliseconds for ingest, scoring, crop, and total runtime. Timing is
machine- and root-dependent; use fixture metrics for deterministic quality
comparisons and benchmark metrics for local performance tracking.

Use a benchmark manifest for repeatable named views over the same corpus:

```powershell
cargo run -p crop-cli -- bench --manifest benchmark-manifest.json
```

```json
{
  "schema_version": "crop.bench-manifest.v1",
  "cases": [
    {
      "id": "docs-boundary",
      "root": "docs",
      "task": "explain graph context boundary",
      "token_budget": 12000,
      "seed": 0,
      "view": "docs-only"
    },
    {
      "id": "repo-architecture",
      "root": ".",
      "task": "explain CROP architecture",
      "token_budget": 12000,
      "seed": 0,
      "view": "whole-repo"
    }
  ]
}
```

Manifest roots are resolved relative to the manifest file unless they are
absolute paths. The optional `view` label names the crop recipe being tested
without changing the underlying local-only benchmark mechanics.

Manifest output includes both per-case rows and an `aggregate` block. The
aggregate reports method `crop`, case count, total source/unit/edge counts,
total selected tokens, mean selected size, mean quality score, mean ingest/score
/crop/total milliseconds, and max total milliseconds. Use the aggregate for
quick run-to-run comparison and the case rows for debugging individual views.

See [`docs\BENCHMARKS.md`](docs/BENCHMARKS.md) for fair comparison rules,
manifest view guidance, competitor export scoring, and benchmark interpretation
caveats.

## Rust API

`crop-core` exposes the first local engine surface:

- `fixture_graph()` returns the deterministic bridge/noise fixture.
- `ingest_path(root)` walks Markdown, text, JSON, TOML, YAML, and Rust files,
  chunks text, hashes sources, and builds same-file plus Markdown-link edges.
- `score_graph_for_task(graph, task)` assigns deterministic lexical relevance.
- `crop_context(graph, CropOptions)` emits a provenance-aware `CropPack`.
- `prefix_cached_pack(ingested, pack, task, options, profile)` wraps a crop in a
  cache-aware prefix/body/task-tail manifest.
- `bench_pack(root, task, options)` runs local ingest, scoring, and crop stages
  and returns a benchmark summary with stage timings.
- `bench_manifest_json(json, base_dir)` runs repeatable named benchmark/view
  cases from `crop.bench-manifest.v1` JSON.
- `markdown_index(root, title, options)` and
  `markdown_index_for_view_json(json, base_dir)` generate README/TOC-style
  Markdown source indexes for documentation tools such as PROOF.
- `markdown_link_audit(root, options)` and
  `markdown_link_audit_for_view_json(json, base_dir)` report local Markdown link
  and MDPATH heading health as structured side-info.
- `run_view_json(json, base_dir)` runs a persisted `crop.view.v1` recipe and
  returns `crop.view-run.v1` with the produced context pack.
- `run_view_prefix_cached_json(json, base_dir, profile)` runs a view and returns
  `crop.view-prefix-cache.v1` with a prefix-cache-aware pack.
- `inspect_view_json(json, base_dir)` validates a view and returns
  `crop.view-inspect.v1` root/filter/source metadata without cropping.
- `inspect_view_store(store)` validates every JSON view recipe in a store and
  returns `crop.view-store-inspect.v1` per-view status rows.
- `list_views(store)` lists `crop.view.v1` recipes from a local store directory
  as `crop.view-list.v1` JSON.
- `eval_fixture(case, method, budget)` runs deterministic apple-to-apple
   retrieval comparisons.
- `eval_suite(budget)` runs every case against every local method and returns
   aggregate means.
- `eval_competitor_json(json)` imports saved external retrieval results and
  scores them against the same fixture labels and metric contract.
- `CompetitorAdapter` defines the minimal export trait for adapters that want to
  emit the same competitor result contract from Rust.

## Development

```powershell
cargo fmt
cargo test --workspace
```

By default, CROP depends on BISECT shared packages from GitHub. For local peer
development beside `C:\src\apportionment`, copy:

```powershell
Copy-Item .cargo\config.local.example.toml .cargo\config.toml
```

Then Cargo will patch `rctx-core`, `rgraph-core`, `rstat-core`, and `ropt-core`
to the sibling checkout.

