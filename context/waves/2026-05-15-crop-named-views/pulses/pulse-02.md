# Pulse 02: View listing and store

## Status

Done.

## Goal

Add a conventional local view store and a list command so persisted views can be
discovered without knowing every JSON file path.

## Deliverables

- Add `MdcropViewSummary` and `MdcropViewList` output types.
- Add `list_views(store)` in `mdcrop-core`.
- Add `mdcrop view --list --dir <path>`, defaulting to `.mdcrop\views`.
- Ignore non-JSON files and return views sorted by file name.
- Document the conventional `.mdcrop\views` store and example listing command.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "MdcropView|run_view|view --file" C:\src\MDCROP -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- view --list --dir examples\views
cargo run -p mdcrop-cli -- view --file examples\views\docs-view.json
git diff --check
```

## Outcome

View stores now emit `mdcrop.view-list.v1` JSON with file names, view names, roots,
tasks, budgets, and seeds. The CLI defaults list operations to `.mdcrop\views`
while still allowing `--dir examples\views` or any other local store.
