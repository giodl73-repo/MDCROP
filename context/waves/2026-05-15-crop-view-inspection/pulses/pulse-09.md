# Pulse 09: Heading inventory

## Scout commands

```powershell
cargo run -q -p mdcrop-cli -- links --root C:\src\mdloom -e md --exclude-dir target
cargo run -q -p mdcrop-cli -- links --view examples\views\whole-repo-view.json -e md
```

## Decision

MDLOOM needs TOC and anchor primitives in addition to link checks. Since MDPATH
already defines heading normalization, MDCROP should expose heading inventories
with canonical `md://` section URIs and duplicate-anchor groups for downstream
documentation tools.

## Checklist

- [x] Add `mdcrop.markdown-heading-inventory.v1` structs.
- [x] Add root-based heading inventory API.
- [x] Add view-based heading inventory API.
- [x] Add `mdcrop headings --root` / `mdcrop headings --view`.
- [x] Include normalized anchors, line numbers, and canonical `md://` URIs.
- [x] Include duplicate-anchor groups.
- [x] Add positive tests for root and view inventories.
- [x] Update README, product plan, and wave docs.
- [x] Run validation gates.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -q -p mdcrop-cli -- headings --root C:\src\mdloom -e md --exclude-dir target
cargo run -q -p mdcrop-cli -- headings --view examples\views\whole-repo-view.json -e md
git diff --check
```
