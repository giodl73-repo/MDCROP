# Wave: CROP Named Views

## Goal

Turn ad hoc task/root/budget crop recipes into persisted named views that can be
reused by agents, benchmark runs, and prefix-cache workflows.

## Thesis

Benchmark manifests proved that views are useful as repeatable recipes. The next
step is to make views a first-class product surface: a named JSON recipe that
produces the same context crop whenever the underlying corpus and task are held
constant.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Minimal view runner | done | Added `crop view --file` for `crop.view.v1` recipes. |
| 02 | View listing and store | done | Added `crop view --list --dir` with `.crop\views` as the conventional local store. |
| 03 | View prefix cache | done | Added `crop view --file ... --prefix-cache generic` for cache-aware named views. |
| 04 | Filtered searchable views | done | Added query, extension, and exclude-dir parameters for large-corpus views. |
| 05 | View docs and examples | done | Added checked recipes for docs-only, whole-repo, troubleshooting, Maxim, and Resonance views. |

## Success criteria

- A persisted JSON view can produce a context pack without restating CLI flags.
- View roots resolve relative to the view file location.
- Invalid schemas and zero budgets fail explicitly.
- View outputs remain local-only and deterministic for a fixed corpus.
- `cargo fmt`, `cargo test --workspace`, view smoke runs, and `git diff --check`
  pass for every implementation pulse.

## Validation baseline

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- view --file examples\views\docs-view.json
git diff --check
```
