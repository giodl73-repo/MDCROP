# Pulse 07: Markdown link audit

## Scout commands

```powershell
cargo run -q -p mdcrop-cli -- index --root C:\src\proof -e md --exclude-dir target
cargo run -q -p mdcrop-cli -- index --view examples\views\whole-repo-view.json -e md
```

## Decision

PROOF can own rendering and diagnostics, while MDCROP should expose corpus
side-info. A local Markdown link audit is the next reusable primitive after index
generation: it lets PROOF and other tools consume ok/broken local links from a
root or named view.

## Checklist

- [x] Add `mdcrop.markdown-link-audit.v1` structs.
- [x] Add root-based link audit API.
- [x] Add view-based link audit API.
- [x] Add `mdcrop links --root` / `mdcrop links --view`.
- [x] Add positive tests for ok and broken local links.
- [x] Update README, product plan, and wave docs.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p mdcrop-cli -- links --root C:\src\proof -e md --exclude-dir target
cargo run -q -p mdcrop-cli -- links --view examples\views\whole-repo-view.json -e md
git diff --check
```
