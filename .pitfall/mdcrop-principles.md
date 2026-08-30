# MDCROP Principles

## MDCROP-P-01: Boundary Claims Need Bridge Evidence

**Rationale:** MDCROP's product claim is the context boundary, not just ranked
similarity. A pack that omits bridge evidence can look relevant while failing
the task handoff.

**Decision rule:** Any claim about better task context must cite bridge recall,
source coverage, boundary metrics, or a fixture that demonstrates missing-link
recovery.

**Evidence:** `PRODUCT_PLAN.md`, `docs/BENCHMARKS.md`, `cargo run -p mdcrop-cli
-- eval --case budget-packing --method mdcrop --budget 160`, and
`cargo test --workspace`.

## MDCROP-P-02: Checked View Stores Stay Portable

**Rationale:** `mdcrop view --inspect --dir examples\views --strict` is a
contract gate for agents and CI; machine-local corpus recipes make that gate
depend on a developer's workstation.

**Decision rule:** Recipes under `examples\views\` must resolve inside the
repository and pass strict store inspection on a fresh checkout. Optional local
corpus recipes belong outside the checked store.

**Evidence:** `docs/VIEWS.md`, `README.md`, and `cargo run -p mdcrop-cli --
view --inspect --dir examples\views --strict`.

## MDCROP-P-03: Status Is a Report Before It Is a Gate

**Rationale:** Corpus health output is valuable only if users can inspect the
report even when strict mode fails.

**Decision rule:** Status and view strict modes must render their selected JSON
or Markdown report before returning a non-zero exit.

**Evidence:** `docs/specs/corpus-status.md`,
`crates/mdcrop-cli/src/status.rs`, and `cargo test --workspace`.

## MDCROP-P-04: Source Provenance Is Part of the Pack

**Rationale:** MDCROP exists to hand off evidence, so anonymous snippets or
unstable source identifiers undermine downstream review.

**Decision rule:** Context packs, side-info reports, and mdport outputs must
preserve source IDs, content hashes, or mdpath-style references sufficient for a
reviewer to find the source unit.

**Evidence:** `docs/specs/consumer-integration-options.md`,
`docs/specs/proof-artifacts.md`, and `cargo test --workspace`.

## MDCROP-P-05: Reusable Kernels Stay Below Product UX

**Rationale:** MDCROP consumes shared RLINE/SLICE/MDPATH/MDPORT kernels but owns
its product workflows, CLI packaging, examples, and documentation surfaces.

**Decision rule:** Promote only stable graph, predicate, path, or transfer
logic into shared dependencies; keep MDCROP-specific views, status pages, and
generator affordances local unless another repo has concrete adoption pressure.

**Evidence:** `PRODUCT_PLAN.md`, `Cargo.toml`, `.roles/ROLE.md`, and
`cargo clippy --workspace --all-targets -- -D warnings`.
