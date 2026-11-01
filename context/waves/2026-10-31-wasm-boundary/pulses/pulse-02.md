# Pulse 02: fixed-fixture browser explorer

Status: implemented and locally qualified on 2026-11-01; hosted/live release
qualification follows publication.
Entry: c4557f7485615d0701241f3c91fd49cfd4b0783b. Pulse 01's Windows/Linux
qualification is retained in TRACKER's 2026-10-31 boundary snapshot.

Scout: read accepted `docs/wasm-boundary.md`, fixture/core/probe, role index,
dependency locks, and the preceding SLICE/MDPORT recovery/build patterns.

Deliverables: unpublished `mdcrop-web` adapter, bounded request/report contract,
native CLI parity route, worker-backed own seed/budget controls, complete fixture
and selected/excluded provenance, visible estimated-budget overrun, downloads,
source/build/licenses/provenance, pinned clean Pages workflow and browser proof.
No production kernel, upstream Git pin, native CLI or consumer schema changes.

Controls use decimal text in JSON so Rust, not JavaScript number coercion, owns
admission. Seed must be 0–3; budget 1–256. Envelope cap: 1,024 UTF-8 bytes.
Unknown/duplicate fields, missing/wrong types, signed/fractional/non-finite and
out-of-range values fail explicitly. No user graphs or corpus ingestion.

Validate native workspace/probe, full 1,024-control native/WASM report parity,
invalid controls, own-control/download/overrun/excluded evidence, deadlines and
late results, keyboard/mobile/no-JS, exact source/build/legal routes, <5 MB site,
producer roles, clean hosted PR/default artifact and fresh live release custody.
Commit child first, update only its TRACKER pointer after release proof; retire
the owned child worktree after preserving evidence and keep shared target caches.

Local proof: 65 workspace native tests + four retained scout tests; strict new
adapter host/WASM Clippy; 22 browser cases with zero retries/skips/flaky cases.
One real-worker case compares all 1,024 admitted canonical reports; another
compares 28 representative/invalid raw requests. Eight own-control cases verify
canonical report/request/pack downloads and selected/excluded provenance. Other
cases cover strict text admission, overrun, seed changes, source/fixture/legal
routes, worker startup/compute/late-result faults, UTF-8 budget, keyboard/narrow
layout and no-JavaScript access. Screenshots were inspected. Fixture connectivity
is fixed; no arbitrary disconnected graph input or corpus is admitted.

`local-qualification.json` records preview measurements explicitly as dirty/local,
not final release byte pins. Core/CLI source and the four upstream Git revisions
are unchanged. Producer role review lives in `docs/reviews/wasm-explorer.md`.
