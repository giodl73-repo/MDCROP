# Browser fixture boundary

Entry: public main 01bf332a2a39d8c95d6af7389f3fcab23b447f62.
This wave starts with a portability/scope decision, not a public demo.

| Pulse | Scope | Status |
|---|---|---|
| 01 | Inspect kernel/dependencies, compile probe and accept public fixture boundary | locally complete; hosted validation pending |
| 02 | Implement bounded worker/reader task and qualify Pages release | proposed, not executed |

Pulse 01 scout: inspect `fixture_graph`, `mdcrop_context`, locked dependency tree,
native ingestion/benchmark entry points, roles and existing synthetic evaluation
proof. Use unchanged deterministic fixture to show bridge recovery and noise
exclusion under an estimated token budget. Compare only against the repository's
existing score-top-k fixture; do not generalize to vector/RAG/framework wins.

Deliverables: `docs/wasm-boundary.md`, producer role review, reproducible standalone
`tools/wasm-probe` harness, compact probe receipt, native/portable validation.
The probe may enable a browser entropy backend only in its own WASM dependency
closure; no kernel/RLINE/SLICE/MDPORT pin or CLI/schema changes. Compile/link proof
does not establish browser runtime, reader-task, performance or deployment proof.

Validation: pinned Rust 1.95.0, formatting, locked native workspace tests,
standalone native probe tests, release WASM build and diff check. All builds use
one shared laptop target. Commit child first, then an explicit TRACKER readiness
snapshot. Preserve old separate PITFALL branch history and unrelated owner state.
