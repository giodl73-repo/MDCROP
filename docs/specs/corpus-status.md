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
| `strict_pass` | `true` when `issue_count` is zero. |
| `surfaces` | Rows for `catalog_sources`, `links`, `orphan_pages`, `frontmatter_pages`, and `headings`, each with `count` and `issues`. |

`--strict` writes the selected output format, then exits non-zero if the health
summary includes broken links, orphan pages, or duplicate heading anchors. This
lets CI systems publish the status artifact while still gating on corpus health.
