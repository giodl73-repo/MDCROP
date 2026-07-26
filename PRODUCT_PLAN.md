# CROP Product Plan

## Product thesis

CROP is a local-first graph engine for AI context. It turns scattered artifacts
into an evidence graph, then crops the graph into a compact, provenance-aware
context pack for a task.

The wedge is not "another index." The wedge is **the right context boundary**:
include dense relevant neighborhoods, bridge nodes, canonical sources, and
provenance paths; exclude stale duplicates and noisy periphery.

## Target users

1. AI builders wiring agents to repositories, docs, and issue trackers.
2. Research-heavy builders using Markdown, Obsidian, PDFs, GitHub, and notes.
3. Technical teams that need reproducible context handoffs across sessions.

## Differentiation

| Capability | Ordinary vector/RAG index | CROP |
|------------|---------------------------|------|
| Retrieval unit | Similar chunks | Typed graph neighborhoods |
| Boundary | Top-k or score threshold | Graph-cut / budgeted crop |
| Missing links | Often missed | Bridge and path search |
| Staleness | Metadata filter | Recency and supersedes edges |
| Provenance | Optional source labels | First-class source trails |
| Context budget | Truncate or rerank | Optimize coverage, diversity, authority, and freshness |

## Waves

1. **Local graph fixture**: Rust workspace, BISECT shared-kernel dependencies,
   fixture evidence graph, crop scoring, JSON output. **Done.**
2. **Repo and Markdown ingest**: walk local repos/vaults, chunk text, attach
   provenance, build dependency/reference edges. **First slice done via
   `crop pack`.**
3. **Crop engine**: task seed expansion, boundary scoring, bridge selection,
   dedupe, token-budget packing.
4. **Agent harness**: CLI commands that emit model-ready context packs and a
   replayable manifest. **Prefix-cache-aware pack output is available with
   `--prefix-cache generic`.**
5. **Evaluation loop**: fixtures and benchmarks against top-k vector retrieval,
    measuring noise, missing bridges, source coverage, and later real-corpus
    latency. **Five apple-to-apple fixture cases are available via `crop eval`;
    offline external competitor exports can be scored with `crop eval-import`.**
6. **Real-corpus benchmarks**: local root timing, benchmark manifests,
   competitor export examples, and aggregate reports. **Active wave; `crop bench`
   supports one-off roots and named manifest views with aggregate summaries, and
   checked competitor export examples live under `examples/competitors/`.
   Fair-comparison guidance is documented in `docs/BENCHMARKS.md`. Done.**
7. **Named views**: persisted task/root/budget recipes that produce reusable
   context packs. **Done; `crop view --file` runs recipes and `crop view --list`
   discovers local view stores; views can emit prefix-cache-aware packs with
   `--prefix-cache generic`, and checked examples cover docs-only, whole-repo,
   troubleshooting, Maxim, and Resonance slices.**
8. **View inspection**: safe preflight for named views. **Done;
   `crop view --inspect` validates recipes, resolves roots, applies filters, and
   reports matched source counts before running a full crop. Store-wide
   inspection reports per-view status plus aggregate source-count and extension
   profiles for local view directories, and `--strict` turns failed recipes into
   CI-friendly non-zero exits.**
9. **Docs indexes for generators**: Markdown README/TOC index primitives.
   **First slice; `crop index` and `crop-core` index APIs generate source tables
   that MDLOOM can consume while keeping rendering and compile targets in MDLOOM.**
10. **Docs link side-info**: local Markdown link audits for generators and
    curators. **First slice; `crop links` and `crop-core` link audit APIs report
    ok/broken local links, heading fragments, and `md://` targets for roots or
    views using MDPATH-compatible resolution.**
11. **Heading inventories**: TOC and anchor primitives for documentation tools.
     **First slice; `crop headings` and `crop-core` heading inventory APIs expose
     normalized anchors, line numbers, canonical `md://` section URIs, and
     duplicate-anchor groups.**
12. **Backlink side-info**: inbound reference and orphan-page primitives for
    generated docs. **First slice; `crop backlinks` and `crop-core` backlink APIs
    expose per-page inbound links, outbound link counts, and orphan flags using
    MDPATH-compatible local link resolution.**
13. **Frontmatter inventories**: metadata primitives for generated docs.
    **First slice; `crop frontmatter` and `crop-core` frontmatter APIs expose
    top-level Markdown metadata fields and aggregate key counts for MDLOOM README
    tables and HTML sidebars.**
14. **Markdown catalogs**: ready-to-embed README/TOC tables for generators.
    **First slice; `crop catalog` combines source titles, frontmatter metadata
    such as `tags` and `status`, backlink counts, outbound counts, and orphan
    flags into a Markdown table.**
15. **Frontmatter query views**: tag and metadata slices for large corpora.
    **First slice; `crop.view.v1` supports `frontmatter_query` predicates like
    `tags has 'computing' and status ne 'draft'` over arbitrary top-level
    frontmatter fields for Maxim/MDLOOM-style reusable views. The predicate
    kernel now uses SLICE while CROP keeps frontmatter extraction, view policy,
    graph selection, and rendering.**
16. **MDLOOM artifact manifests**: generated-output health for docs pipelines.
    **First slice; `crop artifacts` reads MDLOOM `.mdloom/artifacts.json` records
    and reports source/output mappings, missing outputs, stale outputs, cache
    status, directive counts, and diagnostics. Markdown status-table output is
    available with `--format markdown`.**
17. **MDLOOM contract hardening**: fixtures, schema docs, and module boundaries.
    **First slice; deterministic `examples/mdloom-fixture` goldens and
    `docs/specs` contracts cover MDLOOM side-info outputs, with the artifact
    surface split into focused core/CLI modules.**
18. **Markdown status surfaces**: embeddable tables for side-info reports.
    **First slice; `crop links`, `crop backlinks`, `crop frontmatter`, and
    `crop headings` support `--format markdown` while keeping JSON as the stable
    machine-readable default.**
19. **Corpus status page composition**: one command for generated health pages.
    **First slice; `crop status` composes catalog, link, backlink, frontmatter,
    and heading tables into a single Markdown status page, and can source
    documentation side-info from a named view. `--strict` can gate CI on broken
    links, orphan pages, or duplicate anchors after rendering the page.**
20. **Status contracts**: machine-readable combined corpus health. **Done;
    `crop status --format json` emits `crop.corpus-status.v1` with per-surface
    counts, issue totals, strict policy names, and strict-pass state for CI and
    downstream registries. `--strict-on` can limit CI gates to broken links,
    orphan pages, or duplicate heading anchors, and root/view JSON goldens live
    under `examples/mdloom-fixture/golden`.**

## Upstream kernel asks

CROP should keep product-specific UX and packaging in this repo, while reusable
graph/stat/math primitives land in BISECT shared kernels:

| BISECT issue | Status | Kernel area | Why CROP needs it |
|--------------|--------|-------------|-------------------|
| [#4](https://github.com/giodl73-repo/BISECT/issues/4) | Done | shared-kernel contract | Keep `rctx-core`, `rgraph-core`, `rstat-core`, and `ropt-core` stable as external Git dependencies. |
| [#5](https://github.com/giodl73-repo/BISECT/issues/5) | Done | `rgraph-core` boundary metrics | Score context crop boundaries with cut size, conductance-like ratios, and internal/external degree. |
| [#6](https://github.com/giodl73-repo/BISECT/issues/6) | Done | `rgraph-core` / `ropt-core` bridge helpers | Recover connector artifacts that plain top-k retrieval misses. |
| [#8](https://github.com/giodl73-repo/BISECT/issues/8) | Done | `rstat-core` evidence scoring | Normalize relevance, authority, freshness, diversity, and provenance confidence. |
| [#9](https://github.com/giodl73-repo/BISECT/issues/9) | Done | `ropt-core` budgeted packing | Optimize context packs under token budgets rather than truncating ranked lists. |
| [#10](https://github.com/giodl73-repo/BISECT/issues/10) | Done | `rgraph-core` clustering | Build cluster-aware context packs from actual evidence neighborhoods. |
| [#11](https://github.com/giodl73-repo/BISECT/issues/11) | Done | graph/stat/math fixtures | Prove graph-native wins against top-k retrieval before adding LLM calls. |

## First command shape

```powershell
crop pack --root C:\src\apportionment --task "explain report provenance" --budget 12000 --output context.crop.json
```

## Success criteria

- Produces deterministic JSON context packs with source IDs and crop reasons.
- Demonstrates graph-native wins on fixtures before adding embeddings.
- Keeps local ingest and crop selection fast enough to run before each agent
  turn.
- Maintains GitHub dependencies with a local peer-development patch path.

