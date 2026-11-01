# Build and verify the fixture explorer

Use the pinned Rust 1.95.0 toolchain, wasm32-unknown-unknown target, rustfmt and
Clippy, wasm-bindgen CLI 0.2.127, Python 3.11+ and Node 22. The root Cargo lock
retains the existing RLINE/MDPATH/MDPORT/SLICE Git revisions. `mdcrop-web/wasm`
adds an explicit getrandom/js backend only on wasm32; fixture selection does
not call RNG APIs. Native filesystem/timing APIs are not exported.

```text
cargo fmt --all --check
cargo test --locked --workspace --all-features
cargo test --locked --manifest-path tools/wasm-probe/Cargo.toml
cargo clippy --locked -p mdcrop-web --all-features --all-targets -- -D warnings
cargo clippy --locked -p mdcrop-web --lib --features wasm --target wasm32-unknown-unknown -- -D warnings
npm ci
npx playwright install chromium
python -B tools/build-pages.py
npm run test:pages
```

Strict Clippy targets the new adapter; all existing native workspace and scout
tests run. Generated site/dependencies/reports stay ignored. An optional shared
`CARGO_TARGET_DIR` avoids duplicated local build trees. Serve the project prefix
with `python -B tools/serve-pages.py` on loopback port 4177. Set
`MDCROP_BASE_URL=https://giodl73-repo.github.io/MDCROP/` for fresh live replay.

The builder emits release WASM/bindings, a native-generated complete fixture and
static fallback, native references for representative/invalid requests and all
1,024 admitted control pairs, source/provenance/legal routes and an inventory.
Clean builds use `git archive HEAD` and compare core/adapter/lock/build entries
with Git blobs. Dirty previews are labelled and use tracked working files; CI
rejects dirty artifacts. Package notices include Git workspace-root grants when
crates inherit them. Original third-party/Rust grants remain separate from MIT.
The full delivery must stay below 5,000,000 file bytes.

## Shared native and WASM contract

`cargo run --locked -p mdcrop-web --bin mdcrop-explore -- REQUEST.json` emits the
same compact report as the exported WASM `explore` function. A bounded file read
caps the request at 1,024 UTF-8 bytes. `--fixture` prints the exact graph and
`--matrix` prints all admitted native request/report pairs for parity testing.

Request example: `{"seed":"0","budget":"160"}`. Values are decimal text to
preserve user input without JavaScript numeric coercion. Rust requires ASCII
digits, seed 0–3 and budget 1–256; signs, whitespace, fractions, exponents,
hexadecimal, non-finite text, numeric/bool/null fields, unknown/duplicate/missing
fields and excessive envelopes fail. Leading zeroes remain decimal. The response
schema is `mdcrop.fixture-explorer.v1`, owned by this adapter rather than the
native CLI/view/status consumer contracts. Successful reports include normalized
controls, all four units with native selection/reasons, inside/boundary/outside
edges, unchanged native pack and explicit estimated-budget overrun.

Browsers execute the real worker WASM, including full 1,024-control canonical
parity and representative own controls/downloads. Fault tests deliberately
intercept worker/fetch delivery. Fresh contexts, zero retries, keyboard/narrow
layout, deadlines/late events and independent no-JS/source routes are checked.
Four-note readiness observations include network/worker/UI overhead and are not
CPU or large-corpus benchmarks. Inspect screenshots before release.

PRs are validated first; Pages deploys only the tested default-branch artifact
with separate pages/id-token permissions. Release custody compares every live
file against that exact archive before fresh live browser replay.
