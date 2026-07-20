# Contributing

Contributions should keep CROP product-neutral, deterministic, and explicit
about provenance and benchmark limits.

## Before opening a pull request

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo run -p crop-cli -- fixture --budget 170
```

Keep changes focused, update public contracts and examples when behavior
changes, and avoid committing local corpora, generated secrets, or
`.cargo\config.toml`.
