# CROP Views

Views are persisted context recipes: root, task, budget, seed, and optional
filters. They let a large corpus expose repeatable, searchable slices without
restating CLI flags each time.

## View parameters

`crop.view.v1` supports:

| Field | Meaning |
|-------|---------|
| `name` | Stable human-readable view name. |
| `root` | File or directory root, resolved relative to the view JSON file. |
| `task` | Default search/scoring task for the crop. |
| `token_budget` | Maximum selected context tokens. |
| `seed` | Seed unit index for graph expansion. |
| `include_extensions` | Optional file-type allow-list such as `["md"]` or `["md", "tex"]`. |
| `exclude_dirs` | Directory basenames to skip, such as `["_archive"]` or `["site"]`. |

CLI overrides make a view searchable at runtime:

```powershell
cargo run -p crop-cli -- view --file examples\views\docs-view.json --query "prefix cache" -e md
cargo run -p crop-cli -- view --file examples\views\docs-view.json --query "theory" -e md -e tex --exclude-dir site
```

Use `--prefix-cache generic` with the same view to split stable corpus prefix
metadata from the volatile task tail.

## Checked example recipes

Reusable examples live under `examples\views\`:

| File | Root | Purpose |
|------|------|---------|
| `docs-view.json` | `..\..\docs` | CROP documentation search and benchmark/view workflow context. |
| `whole-repo-view.json` | `..\..` | Full repository context filtered to source, docs, manifests, and JSON examples. |
| `troubleshooting-view.json` | `..\..` | Debugging failures across views, benchmarks, and competitor imports. |
| `maxim-computing-view.json` | `C:\src\maxim\computing` | Local Maxim computing topic slice. |
| `resonance-theory-view.json` | `C:\src\resonance\guides\theory` | Local Resonance theory slice. |

Run any recipe directly:

```powershell
cargo run -p crop-cli -- view --file examples\views\whole-repo-view.json --query "prefix cache"
cargo run -p crop-cli -- view --file examples\views\maxim-computing-view.json -e md
```

Inspect first when a view points at a large or optional corpus:

```powershell
cargo run -p crop-cli -- view --inspect --file examples\views\whole-repo-view.json -e md
cargo run -p crop-cli -- view --inspect --file examples\views\resonance-theory-view.json -e md -e tex
cargo run -p crop-cli -- view --inspect --dir examples\views
cargo run -p crop-cli -- view --inspect --dir examples\views --strict
```

Inspection emits `crop.view-inspect.v1` with the resolved root, active task and
filters, supported source count, extension counts, and up to ten sample source
IDs. It fails on bad schemas, zero budgets, missing roots, and filters that match
no supported files, without building a full context crop.
Store inspection emits `crop.view-store-inspect.v1` with sorted per-file `ok` or
`failed` rows plus total/max supported source counts and aggregate extension
counts, so a local `.crop\views` directory can be audited in one command. Use
`--strict` for CI or agent gates: CROP prints the inspection artifact, then exits
non-zero when any recipe failed.

Use the whole-repo and troubleshooting views as templates for `.crop\views` in
other repositories. Use the Maxim and Resonance examples as local large-corpus
recipes; they require those sibling checkouts to exist at `C:\src`.

## Maxim view candidates

`C:\src\maxim` is a broad Markdown encyclopedia. Scouting showed thousands of
Markdown files and many topic directories. Useful view families:

| View | Root | Parameters | Use |
|------|------|------------|-----|
| Encyclopedia map | `C:\src\maxim` | `-e md --exclude-dir _archive` | Cross-topic navigation and concept lookup. |
| Topic slice | `C:\src\maxim\computing` or another topic directory | `-e md` | Focused topic context. |
| Theory slice | `C:\src\maxim` | `--query "information theory" -e md` | Search across theory-heavy directories. |
| Puzzle/codes | `C:\src\maxim\puzzle-hunt` or `C:\src\maxim\codes` | `-e md` | Puzzle and symbolic systems context. |
| Project/meta | `C:\src\maxim\context` | `-e md` | Operational notes and planning context. |

## Resonance view candidates

`C:\src\resonance` is a research lab. It has Markdown plus a large LaTeX
research layer, so filtered views need both Markdown and TeX support.

| View | Root | Parameters | Use |
|------|------|------------|-----|
| Research lab | `C:\src\resonance\research` | `-e md -e tex -e bib` | Paper/research retrieval. |
| Theory | `C:\src\resonance\guides\theory` or `C:\src\resonance\site\theory` | `-e md` | Conceptual/theory context. |
| Book | `C:\src\resonance\book` | `-e md -e tex` | Manuscript context. |
| Signals | `C:\src\resonance\signals` | `-e md -e yaml` | Signal/evidence records. |
| Sources | `C:\src\resonance\sources` | `-e md -e bib` | Bibliography and source context. |

For both corpora, prefer a narrow root first. Use whole-corpus views only when
the task truly needs cross-domain bridges.
