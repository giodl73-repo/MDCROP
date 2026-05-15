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
