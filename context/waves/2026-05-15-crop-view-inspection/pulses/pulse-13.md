# Pulse 13: Frontmatter Query Views

## Goal

Make large Markdown corpora such as Maxim queryable by frontmatter metadata so
tagged files can become reusable named views.

## Scope

- Add optional `frontmatter_query` to `mdcrop.view.v1`.
- Filter supported Markdown files by simple SQL/OData-style predicates.
- Support arbitrary top-level frontmatter fields, not only tags.
- Support `eq`, `ne`, and `has` operators joined with `and`.
- Surface the query in view inspection and view listing outputs.

## Example

```json
{
  "schema_version": "mdcrop.view.v1",
  "name": "maxim-computing-ready",
  "root": "C:\\src\\maxim",
  "task": "computing systems context",
  "token_budget": 12000,
  "seed": 0,
  "include_extensions": ["md"],
  "frontmatter_query": "tags has 'computing' and status ne 'draft'"
}
```

## Checklist

- [x] View schema, ingest options, inspect/list output.
- [x] Frontmatter query evaluator over arbitrary metadata fields.
- [x] Tests for positive Maxim-style tag/status/version filtering.
- [x] Tests for invalid predicate rejection.
- [x] README, product plan, and wave tracker updates.
- [x] `cargo fmt`
- [x] `cargo test --workspace`
- [x] View/query smoke run.
- [x] `git diff --check`

## Outcome

Named views can now behave like lightweight SQL/OData views over Markdown
frontmatter, enabling tag-based Maxim slices and richer MDLOOM metadata views.
