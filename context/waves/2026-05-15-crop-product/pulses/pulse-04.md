# Pulse 04: Context pack manifest

## Status

Done.

## Goal

Emit replayable JSON manifests with source trails, deterministic hashes, and a
prefix-cache-aware layout for repeated agent calls over the same evidence graph.

## Delivered

- [x] Added `PrefixCacheProfile::Generic`.
- [x] Added `PrefixBlock`, `PrefixSourceEntry`, `TaskTail`, and
  `PrefixCachedPack`.
- [x] Added `prefix_cached_pack(...)` in `crop-core`.
- [x] Added `crop pack --prefix-cache generic`.
- [x] Kept bare `crop pack` output backward-compatible.
- [x] Added tests proving prefix hash/cache key remain stable when only the task
  changes and change when source content changes.

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- pack --root README.md --task "explain prefix cache" --budget 80 --prefix-cache generic
git diff --check
```
