# Pulse 21: PITFALL Use-Case Policy Coverage

Date: 2026-08-29

## Goal

Make MDCROP's open PITFALL entries read as user and consumer failure modes,
then retain executable coverage for context authority, benchmark overclaim,
and consumer-compatibility drift.

## Changes

- Added actor, task, surface, likely mistake, consequence, owner, and test
  fields to `MDCROP-PF-01`, `MDCROP-PF-02`, and `MDCROP-PF-03`.
- Added `crates/mdcrop-core/tests/pitfall_policy.rs` so the policy evidence
  cites the open PITFALL IDs and the docs/specs that constrain them.
- Preserved the existing local-recipe split: portable checked views remain in
  `examples\views`, while machine-local large-corpus recipes live under
  `examples\local-views`.

## Validation

```powershell
C:\Users\giodl\.cargo\bin\cargo.exe fmt --check
C:\Users\giodl\.cargo\bin\cargo.exe test -p mdcrop-core --test pitfall_policy
C:\Users\giodl\.cargo\bin\cargo.exe test --workspace
C:\Users\giodl\.cargo\bin\cargo.exe clippy --workspace --all-targets -- -D warnings
C:\Users\giodl\.cargo\bin\cargo.exe run -p mdcrop-cli -- view --inspect --dir examples\views --strict
C:\Users\giodl\.cargo\bin\cargo.exe run --manifest-path C:\src\TRACKER\repos\standards-protocols\pitfall\Cargo.toml -q -p pitfall-cli -- C:\src\TRACKER\repos\tools-infra\mdcrop --format json
python C:\src\TRACKER\repos\standards-protocols\pitfall\tools\check_pitfall.py C:\src\TRACKER\repos\tools-infra\mdcrop
git diff --check
```

## Result

MDCROP now has retained policy-test citations for all three open repo-local
PITFALL entries. They remain open because they describe recurring user,
research, and consumer-integration risks, not because the test evidence is
missing.
