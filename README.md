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

## Rust API

`crop-core` exposes the first local engine surface:

- `fixture_graph()` returns the deterministic bridge/noise fixture.
- `ingest_path(root)` walks Markdown, text, JSON, TOML, YAML, and Rust files,
  chunks text, hashes sources, and builds same-file plus Markdown-link edges.
- `score_graph_for_task(graph, task)` assigns deterministic lexical relevance.
- `crop_context(graph, CropOptions)` emits a provenance-aware `CropPack`.

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

