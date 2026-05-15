# Markdown Link Audit Schema

Command: `crop links`

Schema version: `crop.markdown-link-audit.v1`

Default output is JSON. `crop links --format markdown` renders the same report
as a Markdown table for generated status pages; it does not change the JSON
schema contract.

## Required fields

| Field | Type | Meaning |
|-------|------|---------|
| `schema_version` | string | Stable schema identifier. |
| `root` | string | Resolved root that was audited. |
| `source_count` | number | Supported sources scanned under filters. |
| `link_count` | number | Local Markdown or `md://` links audited. |
| `ok_count` | number | Links resolved successfully. |
| `broken_count` | number | Links that failed resolution. |
| `links` | array | Link rows with `source`, `target`, `status`, optional `resolved_source`, optional `error`. |

Path fields use CROP source IDs with `/` separators where possible. External
HTTP links are intentionally ignored.

Golden fixture: `examples/proof-fixture/golden/links.json`.
