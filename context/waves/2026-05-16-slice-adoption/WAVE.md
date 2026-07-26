# Wave: SLICE Adoption

## Goal

Replace MDCROP's local low-level predicate parsing with SLICE where the behavior
is already a typed row-selector match, without moving MDCROP view policy or graph
selection into SLICE.

## Thesis

`frontmatter_query` is MDCROP's smallest useful SLICE migration because its
current grammar is already predicate-shaped: top-level field paths, `eq`, `ne`,
`has`, and `and`.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Frontmatter query adapter | done | Wired `frontmatter_query` through `slice-core` while preserving array-like tags and missing-field `ne`. |

## Success criteria

- Existing frontmatter-query view tests keep passing.
- Missing fields still satisfy `ne` and fail `eq`/`has`.
- Unsupported SLICE operators remain invalid for MDCROP frontmatter queries.
- MDCROP still owns view recipes, frontmatter extraction, graph selection, and
  rendering.
