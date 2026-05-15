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
labels for each method. Metrics include gold recall, bridge recall, noise rate,
token use, and quality score. Current cases cover missing bridges, noisy
periphery, stale duplicates, budget packing, and cross-link chains. Current
methods are `crop`, `score-top-k`, `lexical-top-k`, and `hybrid-top-k`. The
`--all` command emits deterministic JSON with ordered cases, methods, per-run
rows, and per-method aggregate means.

## Rust API

`crop-core` exposes the first local engine surface:

- `fixture_graph()` returns the deterministic bridge/noise fixture.
- `ingest_path(root)` walks Markdown, text, JSON, TOML, YAML, and Rust files,
  chunks text, hashes sources, and builds same-file plus Markdown-link edges.
- `score_graph_for_task(graph, task)` assigns deterministic lexical relevance.
- `crop_context(graph, CropOptions)` emits a provenance-aware `CropPack`.
- `prefix_cached_pack(ingested, pack, task, options, profile)` wraps a crop in a
  cache-aware prefix/body/task-tail manifest.
- `eval_fixture(case, method, budget)` runs deterministic apple-to-apple
  retrieval comparisons.
- `eval_suite(budget)` runs every case against every local method and returns
  aggregate means.

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

