# Consumer Integration Options

Audience: downstream documentation systems such as PROOF, knowledge bases,
research notebooks, and large Markdown corpora.

CROP exposes reusable corpus-intelligence surfaces. Consumers should call these
surfaces as generic CROP features instead of expecting consumer-specific logic in
CROP core. If a needed option is missing, file an issue describing the generic
capability, expected input, expected output, and why the current surfaces are
insufficient.

## Principles

- CROP owns corpus inspection, graph/context selection, Markdown side-info, named
  views, and generated status surfaces.
- Consumers own their renderers, compile pipelines, publishing targets, and
  product-specific semantics.
- CROP commands should stay useful for any compatible corpus; consumer-specific
  behavior should be expressed as view files, manifests, or documented adapters.
- JSON outputs are the stable machine-readable contracts. Markdown outputs are
  generated status/README conveniences over the same reports.

## Generic surfaces

| Need | Command/API | Output | Notes |
|------|-------------|--------|-------|
| Source inventory | `crop index` | Markdown | README/TOC table with extension profile and outbound link counts. |
| Catalog page | `crop catalog` | Markdown | Combines titles, link counts, orphan flags, and selected frontmatter fields. |
| Combined status page | `crop status` | Markdown | Composes catalog, links, backlinks, frontmatter, and headings for any Markdown corpus. |
| Combined status summary | `crop status --format json` | JSON | Emits `crop.corpus-status.v1` with per-surface counts, issue totals, and strict-pass state. |
| CI corpus gate | `crop status --strict` | Markdown or JSON + exit code | Writes output first, then fails on selected strict issue classes. |
| Local link audit | `crop links` | JSON or Markdown | Validates Markdown and `md://` local targets; ignores external HTTP links. |
| Backlink/orphan report | `crop backlinks` | JSON or Markdown | Reports inbound links, outbound counts, and orphan pages. |
| Frontmatter inventory | `crop frontmatter` | JSON or Markdown | Reports top-level Markdown frontmatter key/value fields. |
| Heading inventory | `crop headings` | JSON or Markdown | Reports anchors, line numbers, `md://` section URIs, and duplicate anchors. |
| Named corpus slice | `crop view --file` | JSON context pack | Runs persisted `crop.view.v1` recipes. |
| View preflight | `crop view --inspect` | JSON | Validates roots, filters, source counts, extension profiles, and samples. |
| Store preflight | `crop view --inspect --dir --strict` | JSON + exit code | Validates every view in a store and fails on broken recipes. |

Most source-oriented commands accept either `--root <path>` or `--view
<crop.view.v1>`. View mode preserves relative roots, extension filters,
directory excludes, and `frontmatter_query` predicates.

## View options

`crop.view.v1` supports:

| Field | Purpose |
|-------|---------|
| `name` | Human-readable view/status title. |
| `root` | Root path; relative roots resolve from the view file location. |
| `task` | Context-crop task prompt for `crop view --file`. |
| `token_budget` | Context-crop budget. |
| `seed` | Seed unit index for crop expansion. |
| `include_extensions` | File extension allow-list, such as `["md"]`. |
| `exclude_dirs` | Directory basenames to skip. |
| `frontmatter_query` | Optional metadata filter. |

`frontmatter_query` supports simple SQL/OData-style predicates:

- `field eq 'value'`
- `field ne 'value'`
- `field has 'value'`
- clauses joined by lowercase ` and `

Examples:

```json
{
  "schema_version": "crop.view.v1",
  "name": "ready-guides",
  "root": "docs",
  "task": "ready guide corpus",
  "token_budget": 12000,
  "seed": 0,
  "include_extensions": ["md"],
  "frontmatter_query": "tags has 'guide' and status eq 'ready'"
}
```

## Recommended consumer workflows

### Publish a generated corpus status page

```powershell
crop status --root C:\path\to\corpus --output STATUS.md
crop status --view .crop\views\ready-guides.json --output READY_GUIDES.md
```

### Gate CI while preserving the Markdown artifact

```powershell
crop status --view .crop\views\ready-guides.json --strict --output READY_GUIDES.md
crop status --view .crop\views\ready-guides.json --format json --strict --output READY_GUIDES.status.json
crop status --view .crop\views\ready-guides.json --strict --strict-on broken-links --output READY_GUIDES.md
```

Strict mode writes the status page before failing, so CI can upload the page as
an artifact. By default it gates on broken links, orphan pages, and duplicate
anchors. Add repeatable `--strict-on` selectors to gate only `broken-links`,
`orphan-pages`, or `duplicate-anchors`. Root and view JSON examples live in
`examples\proof-fixture\golden\status-root.json` and
`examples\proof-fixture\golden\status-view.json`.

### Feed a registry or tracker

```powershell
crop status --root C:\path\to\corpus --format json --output STATUS.json
```

Registries should read `schema_version`, `issue_count`, `strict_pass`,
`strict_policy`, and the `surfaces` rows from `crop.corpus-status.v1`. If a
registry needs the underlying row-level evidence, run the individual side-info
commands listed below and link those artifacts from the registry record.

### Generate machine-readable side-info

```powershell
crop links --view .crop\views\ready-guides.json --output links.json
crop backlinks --view .crop\views\ready-guides.json --output backlinks.json
crop frontmatter --view .crop\views\ready-guides.json --output frontmatter.json
crop headings --view .crop\views\ready-guides.json --output headings.json
```

Add `--format markdown` to any of those commands when a README/status table is
needed instead of JSON.

## Optional artifact-health adapter

If a consumer emits a generated-artifact manifest compatible with
`crop.proof-artifacts.v1`, CROP can report missing, stale, cached, or diagnostic
artifact rows:

```powershell
crop artifacts --manifest .proof\artifacts.json
crop artifacts --manifest .proof\artifacts.json --format markdown --output ARTIFACTS.md
```

This is an adapter over an external manifest. It should not be required for
generic corpus status pages.

## Filing follow-up issues

Consumers should file an issue when they need behavior not covered above. Good
issues should include:

| Field | Guidance |
|-------|----------|
| Generic capability | Describe the reusable CROP feature, not only the consumer-specific use case. |
| Current workaround | Name the closest existing command or schema. |
| Input contract | Provide a small fixture or view/manifest example. |
| Output contract | Describe JSON fields, Markdown table shape, or exit-code behavior. |
| Acceptance tests | State L0/L1/L2 expectations when applicable. |

Examples of issue scopes:

- richer `frontmatter_query` grammar, such as `or`, parentheses, or numeric
  comparisons;
- additional status-page sections that are useful for any Markdown corpus;
- additional configurable strict-mode policies beyond `broken-links`,
  `orphan-pages`, and `duplicate-anchors`;
- adapters for non-Markdown source metadata;
- richer JSON reports for combined status summaries beyond the
  `crop.corpus-status.v1` health summary.
