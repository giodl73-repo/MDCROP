# Pulse 20: Strict corpus status

## Goal

Let any generated corpus status page also act as a CI gate without losing the
rendered Markdown artifact.

## Changes

- Added `mdcrop status --strict`.
- Strict mode renders or writes the Markdown page first, then exits non-zero if
  broken links, orphan pages, or duplicate heading anchors are present.
- Added CLI coverage for both failing and clean strict-mode corpora.
- Documented strict mode in README, specs, wave, and product plan.

## Validation

- `cargo fmt`
- `cargo test --workspace`
- CLI smoke for strict failure and clean strict success
- `git diff --check`

## Status

Done.
