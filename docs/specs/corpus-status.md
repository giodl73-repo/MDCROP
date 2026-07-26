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

## CLI contract

| Option | Behavior |
|--------|----------|
| `--root <path>` | Scan a directory or file directly. Cannot be combined with `--view`. |
| `--view <path>` | Scan the root and filters from a `crop.view.v1` recipe. Cannot be combined with `--root`. |
| `--format markdown` | Render the combined status page. This is the default. |
| `--format json` | Render the combined `crop.corpus-status.v1` summary. |
| `--strict` | Write the selected output format, then exit non-zero if the active strict policy has issues. |
| `--strict-on <policy>` | Repeatable selector for `broken-links`, `orphan-pages`, or `duplicate-anchors`; requires `--strict`. |

With no `--strict-on` selectors, `--strict` preserves the historical behavior
and gates on all three issue classes.

## JSON contract

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

The `surfaces` rows are intentionally summary-level. Use `crop links`,
`crop backlinks`, `crop frontmatter`, or `crop headings` when a consumer needs
the detailed per-source rows behind a status count.

Example:

```json
{
  "schema_version": "crop.corpus-status.v1",
  "title": "Corpus Status",
  "input": "examples\\mdloom-fixture\\src",
  "issue_count": 3,
  "strict_issue_count": 3,
  "strict_pass": false,
  "strict_policy": ["broken_links", "orphan_pages", "duplicate_anchors"],
  "surfaces": [
    {"name": "catalog_sources", "count": 3, "issues": 0},
    {"name": "links", "count": 2, "issues": 1},
    {"name": "orphan_pages", "count": 3, "issues": 2},
    {"name": "frontmatter_pages", "count": 3, "issues": 0},
    {"name": "headings", "count": 5, "issues": 0}
  ]
}
```

`--strict` writes the selected output format, then exits non-zero if the health
summary includes issues covered by the active policy. With no selectors, strict
mode gates on broken links, orphan pages, and duplicate heading anchors. Add
repeatable `--strict-on broken-links`, `--strict-on orphan-pages`, or
`--strict-on duplicate-anchors` to gate on selected issue classes. This lets CI
systems publish the status artifact while still gating on corpus health.

Deterministic root and view examples are checked in at
`examples\mdloom-fixture\golden\status-root.json` and
`examples\mdloom-fixture\golden\status-view.json`.
