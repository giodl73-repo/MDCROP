# WASM boundary scout harness

This standalone, unpublished workspace probes the existing four-node fixture and
unchanged core. It is not a supported reader-facing API or Pages release.

Use Rust 1.95.0 with wasm32-unknown-unknown. From the repository root:

```text
cargo +1.95.0 fmt --all --check
cargo +1.95.0 test --locked --workspace
cargo +1.95.0 fmt --manifest-path tools/wasm-probe/Cargo.toml --check
cargo +1.95.0 test --locked --manifest-path tools/wasm-probe/Cargo.toml
cargo +1.95.0 build --locked --manifest-path tools/wasm-probe/Cargo.toml --lib --release --target wasm32-unknown-unknown
cargo +1.95.0 run --quiet --locked --manifest-path tools/wasm-probe/Cargo.toml --bin snapshot
```

The final command prints the retained graph and twenty representative native
reports. Four tests exercise provenance, bridge selection, seed-over-budget,
invalid controls, all 1,024 seed/budget pairs and retained snapshot drift. They do not execute WASM in a
browser. The raw release artifact is `mdcrop_wasm_boundary_probe.wasm` beneath
the selected Cargo target. A shared `CARGO_TARGET_DIR` is supported.

The production core's locked WASM check fails without a getrandom browser backend.
This harness supplies getrandom 0.2.17/js only on wasm32 and pins bindings
0.2.127. Its lock was seeded from the production lock before adding these probe
dependencies, preserving all four upstream Git revisions. Use the committed
lock, not a fresh branch resolution or machine-specific patch configuration.
No production source, manifest, lock, native CLI or stable consumer schema changes.

See [`docs/wasm-boundary.md`](../../docs/wasm-boundary.md) for accepted scope,
native-only APIs, known traps and the next pulse's runtime/release gates.
