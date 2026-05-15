# Corpus Status Page

Command: `crop status`

Output: Markdown

`crop status` is a generic composition surface, not a new JSON schema. It
renders one generated page containing:

- a health summary table,
- `crop catalog` Markdown,
- `crop links --format markdown`,
- `crop backlinks --format markdown`,
- `crop frontmatter --format markdown`,
- `crop headings --format markdown`.

Use the individual commands when a downstream system needs stable
machine-readable JSON contracts. Use `crop status` when any Markdown corpus
needs a generated README/status page. The input can come from `--root` or from a
named `crop.view.v1` recipe via `--view`; view mode preserves
`frontmatter_query` filters for tag/status-specific pages.

`--strict` renders the same Markdown output, then exits non-zero if the health
summary includes broken links, orphan pages, or duplicate heading anchors. This
lets CI systems publish the status page as an artifact while still gating on
corpus health.
