# Pulse 01: Minimal view runner

## Status

Done.

## Goal

Add the smallest persisted named-view surface: a JSON view recipe that can be run
into a context pack with one command.

## Deliverables

- Add `CropView` and `CropViewRun` types.
- Add `run_view_json` and `run_view` APIs.
- Add `crop view --file`.
- Resolve relative roots from the view file location.
- Cover success, bad schema, and zero-budget cases with unit tests.

## Scout commands

```powershell
git -C C:\src\CROP --no-pager status --short --branch
rg "bench_manifest|pack|prefix_cached_pack" C:\src\CROP\crates -n
```

## Validation

```powershell
cargo fmt
cargo test --workspace
cargo run -p crop-cli -- view --file examples\views\docs-view.json
git diff --check
```

## Outcome

`crop.view.v1` recipes can now be persisted and run through `crop view --file`.
The output is `crop.view-run.v1` JSON containing the view metadata and produced
`CropPack`.
