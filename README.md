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

## Current status

This repository is scaffolded for implementation waves. The first product wave
builds a minimal local engine that ingests Markdown and repository text,
constructs an evidence graph, computes a task-specific crop, and emits a JSON
context pack.

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

