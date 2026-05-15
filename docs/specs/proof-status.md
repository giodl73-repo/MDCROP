# PROOF Status Page

Command: `crop proof-status`

Output: Markdown

`crop proof-status` is a composition surface, not a new JSON schema. It renders
one generated page containing:

- a health summary table,
- `crop catalog` Markdown,
- `crop links --format markdown`,
- `crop backlinks --format markdown`,
- `crop frontmatter --format markdown`,
- `crop headings --format markdown`,
- `crop artifacts --format markdown` unless `--no-artifacts` is set.

Use the individual commands when PROOF needs stable machine-readable JSON
contracts. Use `crop proof-status` when PROOF needs a generated README/status
page. The documentation side can come from `--docs-root` or from a named
`crop.view.v1` recipe via `--view`; view mode preserves `frontmatter_query`
filters for tag/status-specific pages.
