# Markdown Frontmatter Schema

Command: `mdcrop frontmatter`

Schema version: `mdcrop.markdown-frontmatter.v1`

Default output is JSON. `mdcrop frontmatter --format markdown` renders the same
report as a Markdown key profile and metadata table for generated status pages;
it does not change the JSON schema contract.

## Required fields

| Field | Type | Meaning |
|-------|------|---------|
| `schema_version` | string | Stable schema identifier. |
| `root` | string | Resolved root that was scanned. |
| `source_count` | number | Markdown sources considered. |
| `page_count` | number | Markdown sources with frontmatter fields. |
| `key_counts` | object | Count of pages containing each top-level key. |
| `pages` | array | Rows with `source`, sorted `keys`, and string-valued `fields`. |

MDCROP records simple top-level YAML-style key/value fields as strings. Array-like
values such as tags remain string values, e.g. `[mdloom, guide]`.

Golden fixture: `examples/mdloom-fixture/golden/frontmatter.json`.
