# CROP Wave Phases

## Active wave

None.

## Completed waves

`2026-05-15-crop-real-corpus-benchmarks` — moved from deterministic fixtures to
local-corpus benchmark commands, manifests, competitor exports, aggregate
reports, and fair benchmark documentation.

`2026-05-15-crop-evaluation-suite` — built the apple-to-apple fixture and
baseline suite, metrics contract, and offline competitor import contract that
prove when graph-cut context cropping beats ordinary ranking/index approaches.

`2026-05-15-crop-product` — established the local-first CROP engine and product
surface for graph-cut context cropping.

## Operating protocol

1. Keep waves small enough to finish with tests and a commit.
2. Every pulse must define scout commands, implementation tasks, documentation
   updates, and validation gates.
3. New graph behavior needs a positive fixture and at least one negative input
   test.
4. Dependencies on BISECT shared kernels must work through GitHub and through
   the local peer override documented in `.cargo/config.local.example.toml`.

