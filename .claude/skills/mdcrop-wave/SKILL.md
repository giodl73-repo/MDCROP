---
name: mdcrop-wave
description: Execute MDCROP product waves with scout, implementation, docs, validation, and commits.
user-invocable: true
---

# mdcrop-wave

Use for MDCROP development waves.

## Procedure

1. Read `context/waves/PHASES.md`.
2. Read the active wave `WAVE.md`.
3. Select the first pulse with `Status` or `status` marked `todo`.
4. Run the pulse scout commands.
5. Implement only that pulse's deliverables.
6. Update docs, the pulse checklist, and the wave table.
7. Run:

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

8. Commit with the required co-author trailer.

## Rules

- New graph behavior needs positive and negative coverage.
- Keep GitHub dependency wiring and local peer override working.
- Do not add LLM calls before deterministic fixtures prove graph-native value.

