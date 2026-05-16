# Corpus Status Page

Command: `crop status`

Output: Markdown by default; JSON with `--format json`

`crop status` is a generic composition surface. In Markdown mode, it renders one
generated page containing:

- a health summary table,
- `crop catalog` Markdown,
- `crop links --format markdown`,
- `crop backlinks --format markdown`,
- `crop frontmatter --format markdown`,
- `crop headings --format markdown`.

Use `--format json` when a downstream system needs the combined health summary
as a stable machine-readable contract. The input can come from `--root` or from
a named `crop.view.v1` recipe via `--view`; view mode preserves
`frontmatter_query` filters for tag/status-specific pages.

The JSON summary uses schema version `crop.corpus-status.v1`:

| Field | Meaning |
|-------|---------|
| `schema_version` | Literal `crop.corpus-status.v1`. |
| `title` | Status title used for the report. |
| `input` | Root path or view path that produced the report. |
| `issue_count` | Total broken links, orphan pages, and duplicate anchors. |
| `strict_issue_count` | Issues counted by the active strict policy. |
| `strict_pass` | `true` when `strict_issue_count` is zero. |
| `strict_policy` | Active policy names: `broken_links`, `orphan_pages`, and/or `duplicate_anchors`. |
| `surfaces` | Rows for `catalog_sources`, `links`, `orphan_pages`, `frontmatter_pages`, and `headings`, each with `count` and `issues`. |

`--strict` writes the selected output format, then exits non-zero if the health
summary includes issues covered by the active policy. With no selectors, strict
mode gates on broken links, orphan pages, and duplicate heading anchors. Add
repeatable `--strict-on broken-links`, `--strict-on orphan-pages`, or
`--strict-on duplicate-anchors` to gate on selected issue classes. This lets CI
systems publish the status artifact while still gating on corpus health.

Deterministic root and view examples are checked in at
`examples\proof-fixture\golden\status-root.json` and
`examples\proof-fixture\golden\status-view.json`.
