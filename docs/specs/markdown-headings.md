# Markdown Heading Inventory Schema

Command: `crop headings`

Schema version: `crop.markdown-heading-inventory.v1`

## Required fields

| Field | Type | Meaning |
|-------|------|---------|
| `schema_version` | string | Stable schema identifier. |
| `root` | string | Resolved root that was inventoried. |
| `source_count` | number | Supported sources scanned under filters. |
| `heading_count` | number | Real Markdown headings found. |
| `duplicate_anchor_count` | number | Anchor groups appearing more than once. |
| `headings` | array | Heading rows with `source`, `level`, `text`, `anchor`, `line`, and `md_uri`. |
| `duplicate_anchors` | array | Duplicate anchor groups. |

`md_uri` values use `md://<source>#<anchor>` and exclude synthetic parser root
headings.

Golden fixture: `examples/proof-fixture/golden/headings.json`.
