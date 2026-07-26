# Markdown Catalog Contract

Command: `crop catalog`

Output format: Markdown table, not JSON.

## Stable columns

| Column | Meaning |
|--------|---------|
| `Path` | Relative Markdown source path linked to the source. |
| `Title` | Frontmatter `title` when present, otherwise first heading/file stem. |
| `In` | Inbound resolved local link count. |
| `Out` | Outbound resolved local link count. |
| `Orphan` | `yes` when inbound link count is zero. |
| `Status`, `Tags`, `Wave`, `Version` | Included when those frontmatter keys appear in the catalog. |

MDLOOM can embed this output directly in README/TOC pages. Consumers that need
structured data should combine `frontmatter`, `backlinks`, and `links`.

Golden fixture: `examples/mdloom-fixture/golden/catalog.md`.
