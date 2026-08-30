# MDCROP

**Index, graph, and select Markdown corpora.**

**Series:** [Tools & Infrastructure](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/tools-infrastructure.md).

**Review roles:** This repo uses
[ROLES](https://github.com/giodl73-repo/ROLES), the `.roles` convention for
repository-local review panels.

## MD family

MDCROP is the corpus-selection layer in the MD family:

```text
Markdown → MDPATH → MDCROP → PROOF → MDPORT
             address    select     build      transfer
```

| Repo | Responsibility |
|------|----------------|
| [MDPATH](https://github.com/giodl73-repo/MDPATH) | Stable `md://` addresses for Markdown elements. |
| **MDCROP** | Corpus indexing, graph selection, and bounded context. |
| [PROOF](https://github.com/giodl73-repo/PROOF) | Validation, compilation, rendering, and publication. |
| [MDPORT](https://github.com/giodl73-repo/MDPORT) | Compact portable `mdport.v1` records. |

## Context & Harness family

MDCROP is also the candidate-selection layer in a broader context-control
family:

```text
Sources → FLETCH → MDCROP → LATTICE → WITNESS
           fetch     select     close       replay
```

| Repo | Responsibility |
|------|----------------|
| [FLETCH](https://github.com/giodl73-repo/FLETCH) | Acquire, verify, cache, partition, and bundle source material. |
| **MDCROP** | Index and select bounded, provenance-aware candidate context. |
| [LATTICE](https://github.com/giodl73-repo/LATTICE) | Apply closure, meet/join, budgets, frontiers, packs, and receipts. |
| [WITNESS](https://github.com/giodl73-repo/WITNESS) | Capture harness events, checkpoints, context deltas, and deterministic replay. |

MDCROP proposes the bounded candidate context. It does not fetch sources,
guarantee semantic closure, or control an agent harness.

MDCROP trims Markdown evidence into focused, provenance-aware context packs from
repos, notes, issues, chats, and document corpora. Most RAG systems retrieve chunks;
MDCROP draws the boundary around the right context before an agent answers.

## Why MDCROP

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
cargo run -p mdcrop-cli -- fixture --budget 170
```

Pack a local file or directory into a task-focused JSON context selection:

```powershell
cargo run -p mdcrop-cli -- pack --root C:\src\MDCROP --task "explain graph context boundary" --budget 12000
```

Add `--output context.mdcrop.json` to write the pack to disk instead of stdout.

Generate a Markdown README/TOC-style source index:

```powershell
cargo run -p mdcrop-cli -- index --root C:\src\proof -e md -e html --exclude-dir target
cargo run -p mdcrop-cli -- index --view examples\views\whole-repo-view.json --output INDEX.md
```

`mdcrop index` emits Markdown tables with an extension profile and one row per
source path, title, directory, extension, and outbound Markdown-link count. It is
designed as a reusable `mdcrop-core` surface for tools such as PROOF, which can own
its compile/render pipeline while depending on MDCROP for corpus index primitives.

Generate a richer Markdown catalog table for README/TOC pages:

```powershell
cargo run -p mdcrop-cli -- catalog --root C:\src\proof -e md --exclude-dir target --output CATALOG.md
cargo run -p mdcrop-cli -- catalog --view examples\views\whole-repo-view.json
```

`mdcrop catalog` emits Markdown that combines source titles, inbound/outbound link
counts, orphan flags, and selected frontmatter fields such as `status`, `tags`,
`title`, `wave`, and `version`. It is a ready-to-embed table surface for PROOF
while the structured JSON side-info remains available through `links`,
`backlinks`, `frontmatter`, and `headings`.

Report PROOF generated artifact health:

```powershell
cargo run -p mdcrop-cli -- artifacts --root C:\src\proof
cargo run -p mdcrop-cli -- artifacts --manifest C:\src\proof\.proof\artifacts.json
cargo run -p mdcrop-cli -- artifacts --root C:\src\proof --format markdown --output ARTIFACTS.md
```

`mdcrop artifacts` emits `mdcrop.proof-artifacts.v1` JSON from PROOF's
`.proof\artifacts.json` contract. It maps source files to generated outputs and
reports missing outputs, stale outputs, cache status, directive counts, and
diagnostic counts so PROOF can keep generated Markdown, ASCII, and HTML
surfaces aligned with source. Use `--format markdown` for a ready-to-embed
artifact health table.

Generate a combined corpus status page or machine-readable summary:

```powershell
cargo run -p mdcrop-cli -- status --root C:\src\maxim --output STATUS.md
cargo run -p mdcrop-cli -- status --view examples\proof-fixture\proof-ready-view.json --output STATUS.md
cargo run -p mdcrop-cli -- status --root C:\src\maxim --strict --output STATUS.md
cargo run -p mdcrop-cli -- status --root C:\src\maxim --strict --strict-on broken-links --output STATUS.md
cargo run -p mdcrop-cli -- status --root C:\src\maxim --format json --output STATUS.json
```

`mdcrop status` composes the catalog, link audit, backlinks, frontmatter, and
headings into one Markdown page for any Markdown corpus. Use `--format json` for
the combined `mdcrop.corpus-status.v1` summary contract with per-surface counts,
issue totals, selected strict policy names, and strict-pass state. Use `--view`
to generate status from a named `mdcrop.view.v1` recipe, including frontmatter/tag
filters. Add `--strict` in CI to exit non-zero after writing the selected output
format. By default strict mode gates on broken links, orphan pages, and duplicate
anchors; repeat `--strict-on` to gate only `broken-links`, `orphan-pages`, or
`duplicate-anchors`.

`frontmatter_query` predicates are evaluated through SLICE's shared selector
kernel while MDCROP still owns view recipes, frontmatter extraction, graph
selection, and output rendering.

Stable schema and consumer-integration notes live in [`docs\specs`](docs/specs),
with deterministic status contract goldens under
`examples\proof-fixture\golden`.

## Reuse contract

MDCROP 0.2.0 supports a bounded cross-repository contract through the `mdcrop`
CLI and its named `*.v1` JSON schemas. The contract covers `index`, `catalog`,
`status`, `links`, `backlinks`, `frontmatter`, `headings`, and `view`; command
semantics, strict failures, and schema-version fields are compatibility
surfaces. Internal graph-selection types and unversioned Rust APIs are not part
of this contract.

PROOF is the reference adopter. Its
[`docs/dependencies/mdcrop.json`](https://github.com/giodl73-repo/PROOF/blob/main/docs/dependencies/mdcrop.json)
pins MDCROP 0.2.0 at an immutable revision, while PROOF's integration suite
launches the real sibling `mdcrop` binary and verifies generated index and
frontmatter outputs. Compatibility changes therefore require both MDCROP
goldens and PROOF's consumer-owned adapter proof to pass.

### Maturity improvement: prove existing downstream use

Leverage was MDCROP's weakest core rubric dimension at **35.6**, with PROOF as
its only explicit downstream dependent. The bounded improvement records MAXIM's
existing use instead of adding provider scope: MAXIM tracks 2,725
`mdcrop.view.v1` recipes, pins MDCROP 0.2.0 in a dependency manifest, and runs
two representative views through the real provider with exact source-count
assertions.

With PROOF and MAXIM as two explicit dependents, the next offline portfolio
inventory result is **56.5 leverage**. Proving the compatibility of an existing
large consumer matters more than another command or schema because it closes
the adoption gap that produced the weak baseline.

Audit local Markdown links as structured side-info:

```powershell
cargo run -p mdcrop-cli -- links --root C:\src\proof -e md --exclude-dir target
cargo run -p mdcrop-cli -- links --view examples\views\whole-repo-view.json
cargo run -p mdcrop-cli -- links --root C:\src\proof -e md --format markdown --output LINK_AUDIT.md
```

`mdcrop links` emits `mdcrop.markdown-link-audit.v1` JSON with source, target,
status, resolved source, error details, and aggregate ok/broken counts. It uses
MDPATH-compatible resolution for `md://` URIs and heading fragments, while
ignoring external links so PROOF can focus this report on local corpus health.
Use `--format markdown` for a ready-to-embed local link health table.

Report backlinks and orphan pages:

```powershell
cargo run -p mdcrop-cli -- backlinks --root C:\src\proof -e md --exclude-dir target
cargo run -p mdcrop-cli -- backlinks --view examples\views\whole-repo-view.json
cargo run -p mdcrop-cli -- backlinks --root C:\src\proof -e md --format markdown --output BACKLINKS.md
```

`mdcrop backlinks` emits `mdcrop.markdown-backlinks.v1` JSON with one page row per
Markdown source, title, inbound link list, outbound link count, and orphan flag.
It reuses MDCROP's MDPATH-compatible link resolution so PROOF can build indexes
and README tables with "referenced by" side-info. Use `--format markdown` for a
page-level inbound/orphan table.

Inventory Markdown frontmatter:

```powershell
cargo run -p mdcrop-cli -- frontmatter --root C:\src\proof -e md --exclude-dir target
cargo run -p mdcrop-cli -- frontmatter --view examples\views\whole-repo-view.json
cargo run -p mdcrop-cli -- frontmatter --root C:\src\proof -e md --format markdown --output FRONTMATTER.md
```

`mdcrop frontmatter` emits `mdcrop.markdown-frontmatter.v1` JSON with source paths,
top-level frontmatter key/value fields, and aggregate key counts. This gives
PROOF a small metadata feed for generated README tables, HTML sidebars, and
status/tag summaries without taking over PROOF's renderer. Use
`--format markdown` for a key profile and metadata table across arbitrary
frontmatter fields.

Inventory headings and MDPATH anchors:

```powershell
cargo run -p mdcrop-cli -- headings --root C:\src\proof -e md --exclude-dir target
cargo run -p mdcrop-cli -- headings --view examples\views\whole-repo-view.json
cargo run -p mdcrop-cli -- headings --root C:\src\proof -e md --format markdown --output HEADINGS.md
```

`mdcrop headings` emits `mdcrop.markdown-heading-inventory.v1` JSON with source,
level, text, normalized MDPATH/GitHub anchor, line number, canonical `md://`
section URI, and duplicate-anchor groups. This is the raw TOC and anchor
inventory surface for PROOF. Use `--format markdown` for an embeddable heading
and duplicate-anchor table.

Run a persisted named view recipe:

```powershell
cargo run -p mdcrop-cli -- view --file examples\views\docs-view.json
```

View files use `mdcrop.view.v1` JSON:

```json
{
  "schema_version": "mdcrop.view.v1",
  "name": "mdcrop-benchmark-docs-view",
  "root": "..\\..\\docs",
  "task": "explain MDCROP benchmark and view workflows",
  "token_budget": 12000,
  "seed": 0,
  "include_extensions": ["md"],
  "exclude_dirs": [],
  "frontmatter_query": "tags has 'computing' and status ne 'draft'"
}
```

Relative roots resolve from the view file location. The default output is
`mdcrop.view-run.v1` JSON containing the view metadata and produced context pack.
Use `--format mdport` to emit the same selected context as a compact
`mdport.v1` transfer pack that PROOF and other tools can exchange:

```powershell
cargo run -p mdcrop-cli -- view --file examples\views\docs-view.json --format mdport
```

MDCROP also reads `mdport.v1` JSON files during normal ingest, using each Mdport
section as an evidence unit while preserving the original Mdport `source` path.

`frontmatter_query` is optional and filters Markdown sources using simple
SQL/OData-style predicates over any top-level frontmatter field: `field eq
'value'`, `field ne 'value'`, and `field has 'value'`, joined with `and`. This
supports Maxim-style tag views such as `tags has 'computing' and status eq
'ready'` while also working for fields like `wave`, `version`, or `source`.

List persisted views in a store:

```powershell
cargo run -p mdcrop-cli -- view --list --dir examples\views
```

By convention, local user views live in `.mdcrop\views`; `--dir` can point at any
directory of `mdcrop.view.v1` JSON files. Non-JSON files are ignored and output is
sorted by file name.

Checked example views live under `examples\views\`:

| File | Use |
|------|-----|
| `docs-view.json` | MDCROP docs-only context. |
| `whole-repo-view.json` | Full MDCROP repository architecture context. |
| `troubleshooting-view.json` | Local debugging context for view, benchmark, and import failures. |

Machine-local large-corpus recipes live under `examples\local-views\`. They are
templates for workspaces that have those corpora at the documented paths, not
members of the strict checked example store.

Inspect a recipe before running a full mdcrop:

```powershell
cargo run -p mdcrop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
cargo run -p mdcrop-cli -- view --inspect --dir examples\views
cargo run -p mdcrop-cli -- view --inspect --dir examples\views --strict
```

This returns `mdcrop.view-inspect.v1` JSON with the resolved root, active filters,
supported source count, extension counts, and a small source sample. It validates
schemas, budgets, roots, and empty filters without scoring or selecting context
units. Directory inspection returns `mdcrop.view-store-inspect.v1` with one status
row per JSON view recipe plus aggregate source-count and extension-count totals.
Add `--strict` to make CI or agent workflows exit non-zero if any store recipe
reports `failed`; the JSON report is still printed first.

Emit a prefix-cache-aware named view:

```powershell
cargo run -p mdcrop-cli -- view --file examples\views\docs-view.json --prefix-cache generic
```

This returns `mdcrop.view-prefix-cache.v1` JSON with the view name and a
`PrefixCachedPack`. For repeated runs over the same view root, the prefix hash is
stable when only the task changes; task, seed, and budget stay in `task_tail`.

Views can also be searched with runtime parameters:

```powershell
cargo run -p mdcrop-cli -- view --file examples\views\docs-view.json --query "prefix cache" -e md
```

`--query` overrides the view task for one run, `-e/--extension` restricts ingest
to specific file types, and `--exclude-dir` skips matching directory basenames.
See [`docs\VIEWS.md`](docs/VIEWS.md) for large-corpus view candidates for
`C:\src\maxim` and `C:\src\resonance`.

Emit a prefix-cache-aware pack:

```powershell
cargo run -p mdcrop-cli -- pack --root C:\src\MDCROP --task "explain graph context boundary" --budget 12000 --prefix-cache generic
```

Prefix-cache output separates stable corpus metadata from the selected mdcrop and
task tail:

- `prefix`: stable schema, source index, content hashes, counts, and graph edge
  count.
- `cache_key` / `prefix_hash`: deterministic hash of the prefix block.
- `body`: task-specific selected mdcrop units.
- `task_tail`: volatile task, seed, and budget.

When only `--task` changes against the same root, the prefix hash remains stable
so provider-side prompt prefix caching can reuse the prefix bytes.

Run apple-to-apple fixture evaluations:

```powershell
cargo run -p mdcrop-cli -- eval --case missing-bridge --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case missing-bridge --method score-top-k --budget 160
cargo run -p mdcrop-cli -- eval --case missing-bridge --method lexical-top-k --budget 160
cargo run -p mdcrop-cli -- eval --case missing-bridge --method hybrid-top-k --budget 160
cargo run -p mdcrop-cli -- eval --case noisy-periphery --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case stale-duplicate --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case budget-packing --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --case cross-link-chain --method mdcrop --budget 160
cargo run -p mdcrop-cli -- eval --all --budget 160
```

The fixture suite uses the same units, relevance scores, token budget, and gold
labels for each method. Current cases cover missing bridges, noisy periphery,
stale duplicates, budget packing, and cross-link chains. Current methods are
`mdcrop`, `score-top-k`, `lexical-top-k`, and `hybrid-top-k`. The `--all` command
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

Latency is intentionally not part of the built-in eval fixture metrics; use the
sized MD-vs-Mdport benchmark fixtures when comparing source formats:

```powershell
.\examples\bench-fixtures\generate.ps1 -Size all -Force
.\examples\bench-fixtures\run.ps1 -Size all -Runs 5
```

Those generated fixtures cover small, medium, and large corpora under
`examples\bench-fixtures\out\` so ingest, scoring, and mdcrop timings can be
compared with the same task and budget.

Score saved external competitor results offline:

```powershell
cargo run -p mdcrop-cli -- eval-import --input competitor-results.json
```

The competitor JSON contract is intentionally local-only and dependency-free:

```json
{
  "schema_version": "mdcrop.competitor-results.v1",
  "case": "missing-bridge",
  "token_budget": 160,
  "method": {
    "method_id": "qdrant-demo",
    "competitor_class": "vector",
    "implementation": "qdrant",
    "version": null,
    "notes": "offline export; no network call during MDCROP eval"
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
the same fixture labels, canonical token counts, and metrics as MDCROP and the
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
cargo run -p mdcrop-cli -- eval-import --input examples\competitors\qdrant-vector.missing-bridge.json
```

Benchmark MDCROP on a local repository or document tree:

```powershell
cargo run -p mdcrop-cli -- bench --root C:\src\MDCROP --task "explain graph context boundary" --budget 12000
```

`mdcrop bench` runs the same local ingest, scoring, and mdcrop stages as `mdcrop pack`
but emits summary JSON instead of the full context body. The report includes
source/unit/edge counts, selected token counts, boundary and quality metrics,
and elapsed milliseconds for ingest, scoring, mdcrop, and total runtime. Timing is
machine- and root-dependent; use fixture metrics for deterministic quality
comparisons and benchmark metrics for local performance tracking.

Use a benchmark manifest for repeatable named views over the same corpus:

```powershell
cargo run -p mdcrop-cli -- bench --manifest benchmark-manifest.json
```

```json
{
  "schema_version": "mdcrop.bench-manifest.v1",
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
      "task": "explain MDCROP architecture",
      "token_budget": 12000,
      "seed": 0,
      "view": "whole-repo"
    }
  ]
}
```

Manifest roots are resolved relative to the manifest file unless they are
absolute paths. The optional `view` label names the mdcrop recipe being tested
without changing the underlying local-only benchmark mechanics.

Manifest output includes both per-case rows and an `aggregate` block. The
aggregate reports method `mdcrop`, case count, total source/unit/edge counts,
total selected tokens, mean selected size, mean quality score, mean ingest/score
/mdcrop/total milliseconds, and max total milliseconds. Use the aggregate for
quick run-to-run comparison and the case rows for debugging individual views.

See [`docs\BENCHMARKS.md`](docs/BENCHMARKS.md) for fair comparison rules,
manifest view guidance, competitor export scoring, and benchmark interpretation
caveats.

## Rust API

`mdcrop-core` exposes the first local engine surface:

- `fixture_graph()` returns the deterministic bridge/noise fixture.
- `ingest_path(root)` walks Markdown, text, JSON, TOML, YAML, and Rust files,
  chunks text, hashes sources, and builds same-file plus Markdown-link edges.
- `score_graph_for_task(graph, task)` assigns deterministic lexical relevance.
- `mdcrop_context(graph, MdcropOptions)` emits a provenance-aware `MdcropPack`.
- `prefix_cached_pack(ingested, pack, task, options, profile)` wraps a mdcrop in a
  cache-aware prefix/body/task-tail manifest.
- `bench_pack(root, task, options)` runs local ingest, scoring, and mdcrop stages
  and returns a benchmark summary with stage timings.
- `bench_manifest_json(json, base_dir)` runs repeatable named benchmark/view
  cases from `mdcrop.bench-manifest.v1` JSON.
- `markdown_index(root, title, options)` and
  `markdown_index_for_view_json(json, base_dir)` generate README/TOC-style
  Markdown source indexes for documentation tools such as PROOF.
- `markdown_link_audit(root, options)` and
  `markdown_link_audit_for_view_json(json, base_dir)` report local Markdown link
  and MDPATH heading health as structured side-info.
- `markdown_heading_inventory(root, options)` and
  `markdown_heading_inventory_for_view_json(json, base_dir)` report heading,
  anchor, duplicate-anchor, and `md://` section URI inventories.
- `run_view_json(json, base_dir)` runs a persisted `mdcrop.view.v1` recipe and
  returns `mdcrop.view-run.v1` with the produced context pack.
- `run_view_mdport_json(json, base_dir)` runs the same recipe and returns a
  `mdport.v1` transfer pack for compact AI/cross-tool interchange.
- `run_view_prefix_cached_json(json, base_dir, profile)` runs a view and returns
  `mdcrop.view-prefix-cache.v1` with a prefix-cache-aware pack.
- `inspect_view_json(json, base_dir)` validates a view and returns
  `mdcrop.view-inspect.v1` root/filter/source metadata without mdcropping.
- `inspect_view_store(store)` validates every JSON view recipe in a store and
  returns `mdcrop.view-store-inspect.v1` per-view status rows.
- `list_views(store)` lists `mdcrop.view.v1` recipes from a local store directory
  as `mdcrop.view-list.v1` JSON.
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

By default, MDCROP depends on BISECT shared packages from GitHub. For local peer
development beside `C:\src\apportionment`, copy:

```powershell
Copy-Item .cargo\config.local.example.toml .cargo\config.toml
```

Then Cargo will patch `rctx-core`, `rgraph-core`, `rstat-core`, and `ropt-core`
to the sibling checkout.

## License

MIT. See [`LICENSE`](LICENSE).
