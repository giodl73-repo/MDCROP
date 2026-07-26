# Markdown Backlinks Schema

Command: `mdcrop backlinks`

Schema version: `mdcrop.markdown-backlinks.v1`

Default output is JSON. `mdcrop backlinks --format markdown` renders the same
report as a Markdown table for generated status pages; it does not change the
JSON schema contract.

## Required fields

| Field | Type | Meaning |
|-------|------|---------|
| `schema_version` | string | Stable schema identifier. |
| `root` | string | Resolved root that was analyzed. |
| `source_count` | number | Markdown pages in the report. |
| `ok_link_count` | number | Resolved outbound links used to build inbound references. |
| `orphan_count` | number | Pages with zero inbound links. |
| `pages` | array | Page rows with title, inbound/outbound counts, orphan flag, and inbound link rows. |

Backlinks are derived only from local links that resolve successfully.

Golden fixture: `examples/mdloom-fixture/golden/backlinks.json`.
