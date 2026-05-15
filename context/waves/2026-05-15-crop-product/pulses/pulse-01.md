# Pulse 01: Repository foundation

## Status

Done.

## Pre-implementation scout

```powershell
Test-Path C:\src\CROP
git -C C:\src\TRACKER --no-pager status --short --branch
git -C C:\src\apportionment --no-pager status --short --branch
```

## Deliverables

- [x] Create CROP Rust workspace.
- [x] Add GitHub dependencies on BISECT shared kernels.
- [x] Add local peer override example for `C:\src\apportionment`.
- [x] Add minimal crop-core fixture and tests.
- [x] Add product plan and README.
- [x] Add wave and pulse structure.
- [x] Add project skills.

## Validation

```powershell
cargo fmt
cargo test --workspace
git diff --check
```

