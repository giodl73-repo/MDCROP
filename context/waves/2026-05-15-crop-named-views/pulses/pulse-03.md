# Pulse 03: View prefix cache

## Status

Done.

## Goal

Allow persisted named views to emit prefix-cache-aware packs so repeated view
runs can reuse stable corpus prefix bytes while varying task tails.

## Deliverables

- Add `MdcropViewPrefixCachedRun`.
- Add `run_view_prefix_cached_json` and `run_view_prefix_cached`.
- Add `mdcrop view --file ... --prefix-cache generic`.
- Prove prefix hash stability across task-only changes for the same view root.
- Document the view prefix-cache command.

## Scout commands

```powershell
git -C C:\src\MDCROP --no-pager status --short --branch
rg "prefix_cached_pack|run_view|PrefixCache" C:\src\MDCROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p mdcrop-cli -- view --file examples\views\docs-view.json --prefix-cache generic
git diff --check
```

## Outcome

Named views can now emit `mdcrop.view-prefix-cache.v1` JSON containing the view
name and a `PrefixCachedPack`. The prefix block is stable for the same view root
when only the task changes, while `task_tail` carries the volatile task, seed,
and budget.
