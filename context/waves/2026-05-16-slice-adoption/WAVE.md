# Wave: SLICE Adoption

## Goal

Replace CROP's local low-level predicate parsing with SLICE where the behavior
is already a typed row-selector match, without moving CROP view policy or graph
selection into SLICE.

## Thesis

`frontmatter_query` is CROP's smallest useful SLICE migration because its
current grammar is already predicate-shaped: top-level field paths, `eq`, `ne`,
`has`, and `and`.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Frontmatter query adapter | done | Wired `frontmatter_query` through `slice-core` while preserving array-like tags and missing-field `ne`. |

## Success criteria

- Existing frontmatter-query view tests keep passing.
- Missing fields still satisfy `ne` and fail `eq`/`has`.
- Unsupported SLICE operators remain invalid for CROP frontmatter queries.
- CROP still owns view recipes, frontmatter extraction, graph selection, and
  rendering.
