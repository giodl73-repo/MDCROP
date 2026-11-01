# Bounded browser fixture decision

Status: accepted by producer review for a fixture prototype. Compile/link and
native fixture qualification are complete; browser execution, UI, Pages delivery
and live-release verification are not complete. This is not a native-only exception.

## Reader task and public fixture

Choose one of four seeds and an estimated token budget from 1 to 256. Inspect
which units the existing Rust `mdcrop_context` selects, their inclusion reasons,
and which units/edges remain outside the pack. Show source IDs, original text,
content hashes, estimated costs, representative unit and boundary metrics.
Let readers download the fixture and current pack without an account or service.

Use the existing `fixture_graph()` in `crates/mdcrop-core/src/lib.rs`, not an
uploaded repository or knowledge corpus. The four synthetic units are
`task-seed` (80 estimated tokens), `bridge-note` (40), `target-note` (40) and
`stale-noise` (40). Its undirected edges are seed–bridge, bridge–target and
seed–noise. All text is repository-authored synthetic fixture content under MIT;
`fixture:*` source IDs identify synthetic records, not authenticated external
sources. `native-fixture.json` in the wave retains the exact graph and twenty
representative native reports. Hashes are `sha256:` plus 64 hexadecimal digits
over the UTF-8 unit text. Keep that prefix and the original source identities.

At seed 0/budget 160, native output retains seed, bridge and target and excludes
noise. Existing native evaluation tests compare the repository's score-top-k
baseline and show why selecting a low-score bridge matters on this fixture.
This proves a constructed failure mode, not general retrieval superiority.
Other seeds may select noise; the explorer must show the actual output rather
than force the demonstration's preferred result.

## Scope comparison

Ordinary vector retrieval commonly exposes a result-count control: Qdrant's
[`limit` search parameter](https://qdrant.tech/documentation/search/search/)
sets how many similar results are requested. LlamaIndex documents a
[`similarity_top_k` vector retriever control](https://developers.llamaindex.ai/python/framework-api-reference/retrievers/vector/)
alongside filters and graph-based retrieval options. These primary sources were
checked on 2026-10-31. This comparison identifies a different reader question:
which connected evidence survives a budget, and why? It is not a benchmark of
either product, and does not imply those frameworks cannot implement graph
retrieval or context budgeting. No embeddings, hosted competitor, LLM or quality
evaluation is run in this pulse. Keep competitor comparisons out of the first
interactive UI unless the same seed/budget and validated metrics are available.

## Portable and native boundaries

| Surface | First browser prototype |
|---|---|
| Fixed synthetic graph and `mdcrop_context` | Use unchanged Rust in a worker |
| RLINE connector paths, boundary/cluster metrics, budget selection, summary/scoring | Keep existing locked upstream implementations |
| Seed, budget, selected/excluded units, provenance and pack download | Thin bounded adapter; no selection algorithm in JavaScript |
| Corpus ingest, directory scanning, md:// resolution, views, status/artifacts | Remain native; never expose filesystem stubs to browser callers |
| Benchmark `Instant` timers and competitor/corpus evaluation | Remain native; no CPU or production-speed claim |
| SLICE selectors and MDPORT/MDPATH conversion | Keep dependency pins; no new browser input or consumer contract |
| LATTICE closure, answer authority, counterexample coverage | Not established by selection or this fixture |

The production crate currently combines pure selection and filesystem/timing
adapters. A narrow exported entry point can call only `fixture_graph`,
`mdcrop_context` and JSON serialization without moving native APIs or changing
CLI/view/status contracts. A later module/feature split may improve packaging;
this pulse neither rewrites kernels nor treats a successful build as evidence
that arbitrary native APIs work in a browser.

## Dependency result

The unmodified locked core fails `wasm32-unknown-unknown` checking:
`mdcrop-core → rstat-core → rand → rand_core → getrandom 0.2.17` lacks a browser
backend. This is a compile-time dependency requirement; the selected summary and
weighted-score path does not call RNG APIs. The standalone scout harness opts
into `getrandom/js` only for wasm32. With that explicit feature unification it
builds and links a release WASM artifact, without changing upstream source or
the production manifest/lockfile. Do not add randomization to selection.

The probe lock retains exactly the production lock's Git revisions:
RLINE `7a612a3be30e398590787f64cfae097e865ff4b8`, MDPATH
`4f79f8764c2b8b73d3771a87f36a990be30dc536`, MDPORT
`86da46bfe78bd43fa5dd075dd1bb63344a624072` and SLICE
`ffa956cf8704cae5086af676c9044ae10a9a9b15`. Branch declarations remain resolved
by committed locks; do not update those pins just because other demos shipped.
The probe uses a repo-local path to its own core, never a TRACKER-relative path
or sibling checkout. Native builds do not enable the probe's wasm-only backend.

## Budget and authority traps

The native algorithm always retains the seed. At seed 0/budget 79 it returns the
80-token seed: success can exceed the requested budget. Preserve this behavior
and visibly report the overrun. Estimated fixture costs are not tokenizer counts,
API context limits or billing figures. Do not advertise a strict budget guarantee.
Keep zero, invalid seed, non-integer/negative/non-finite controls, oversize
requests and out-of-range budgets as explicit adapter failures. The probe's typed
u32 ABI is a compile scout, not production JavaScript input validation.

## Next implementation pulse gates

1. A reviewed bounded JSON request/response and a dedicated unpublished adapter
   call the same native function as WASM. Controls admit integer seed 0–3 and
   budget 1–256; only this fixed four-node graph is available. No corpus upload.
2. Real browser tests compare native/WASM reports for representative controls,
   including overrun, disconnected/unselected evidence, invalid input and every
   seed. Probe tests already exercise all 1,024 admitted seed/budget pairs natively;
   that does not substitute for browser parity.
3. A worker owns initialization/computation; startup/compute deadlines, retained
   controls, Retry/Restore distinction and generation/revision fences prevent
   stale exports. All selected and excluded evidence stays inspectable.
4. Public fixture/source ZIP, compiled WASM, exact provenance, build commands and
   scoped license notices have independent routes. Handle project-prefix paths,
   no-JavaScript explanation, keyboard/mobile and a 5 MB delivery budget.
5. Validate PRs, deploy only the tested default-branch artifact and compare its
   exact live bytes before a fresh browser replay and TRACKER release snapshot.

Producer role findings and the compact evidence receipt live in
`context/waves/2026-10-31-wasm-boundary/`. No browser or release readiness is
claimed until those implementation gates pass.
