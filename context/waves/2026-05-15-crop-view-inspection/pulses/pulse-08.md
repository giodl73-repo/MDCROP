# Pulse 08: MDPATH link resolution

## Scout commands

```powershell
Get-Content C:\src\mdpath\src\lib.rs
Get-Content C:\src\mdpath\src\uri.rs
Get-Content C:\src\mdpath\src\resolver.rs
cargo run -q -p mdcrop-cli -- links --root C:\src\mdloom -e md --exclude-dir target
```

## Decision

MDLOOM already uses MDPATH as its stable Markdown addressing layer. MDCROP should
not invent a parallel anchor model. This slice adds a local MDPATH dependency and
uses it to validate `md://` links and heading fragments during MDCROP link audits.

## Checklist

- [x] Add local MDPATH dependency to MDCROP.
- [x] Preserve plain file-link checks.
- [x] Validate relative heading fragments through MDPATH.
- [x] Validate `md://` targets through MDPATH.
- [x] Include link error details in audit rows.
- [x] Add tests for ok/broken heading and `md://` targets.
- [x] Update docs and wave tracker.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p mdcrop-cli -- links --root C:\src\mdloom -e md --exclude-dir target
cargo run -q -p mdcrop-cli -- links --view examples\views\whole-repo-view.json -e md
git diff --check
```
