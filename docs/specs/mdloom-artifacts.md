# MDLOOM Artifact Report Schema

Command: `mdcrop artifacts`

Schema version: `mdcrop.mdloom-artifacts.v1`

Default output is JSON. `mdcrop artifacts --format markdown` renders the same
report as a README/status-page table with source, output, target, status,
existence, stale, cache, directive, and diagnostic columns.

## Required fields

| Field | Type | Meaning |
|-------|------|---------|
| `schema_version` | string | Stable schema identifier. |
| `manifest` | string | Artifact manifest path used for the report. |
| `generated_by` | string | Producer recorded by MDLOOM. |
| `artifact_count` | number | Artifact records in the manifest. |
| `ok_count` | number | Records without missing or stale output. |
| `missing_output_count` | number | Records whose output path does not exist. |
| `stale_count` | number | Records whose source mtime is newer than output mtime. |
| `diagnostic_count` | number | Total diagnostics across artifacts. |
| `artifacts` | array | Rows with source/output paths, target, status, existence/stale flags, cache flag, directive count, and diagnostic count. |

The input contract mirrors MDLOOM `.mdloom/artifacts.json`: `source_path`,
`output_path`, `target`, `status`, `directives_resolved`, `from_cache`, and
`diagnostics`.

Golden fixture: `examples/mdloom-fixture/golden/artifacts.json`.
