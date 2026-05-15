# Pulse 04: Filtered searchable views

## Status

Done.

## Goal

Make named views practical for huge corpora such as `C:\src\maxim` and
`C:\src\resonance` by adding runtime/search parameters and view-level file
filters.

## Deliverables

- Add view-level `include_extensions` and `exclude_dirs`.
- Add `crop view --query`, `-e/--extension`, and `--exclude-dir` overrides.
- Support text-heavy research extensions (`tex`, `bib`, `py`, `html`) in default
  ingest.
- Document candidate Maxim and Resonance view shapes.
- Cover filtered view behavior with unit tests.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
Get-ChildItem C:\src\maxim,C:\src\resonance -Directory
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- view --file examples\views\docs-view.json --query "prefix cache" -e md
git diff --check
```

## Outcome

Views can now act like searchable recipes: keep the persisted root/budget/seed,
override the task with `--query`, restrict file types with `-e`, and exclude
large/generated areas with `--exclude-dir`.
