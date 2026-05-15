# Pulse 02: View listing and store

## Status

Done.

## Goal

Add a conventional local view store and a list command so persisted views can be
discovered without knowing every JSON file path.

## Deliverables

- Add `CropViewSummary` and `CropViewList` output types.
- Add `list_views(store)` in `crop-core`.
- Add `crop view --list --dir <path>`, defaulting to `.crop\views`.
- Ignore non-JSON files and return views sorted by file name.
- Document the conventional `.crop\views` store and example listing command.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "CropView|run_view|view --file" C:\src\CROP -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- view --list --dir examples\views
cargo run -p crop-cli -- view --file examples\views\docs-view.json
git diff --check
```

## Outcome

View stores now emit `crop.view-list.v1` JSON with file names, view names, roots,
tasks, budgets, and seeds. The CLI defaults list operations to `.crop\views`
while still allowing `--dir examples\views` or any other local store.
