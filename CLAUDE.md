# CLAUDE.md

## Pre-push checklist

Always run lint and tests before pushing anything:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Project structure

- `src/` — Rust CLI source code
- `providers/` — JSON recipe files for each API provider
- `npm/` — npm package structure (shim + 5 platform packages)
- `docs/` — Documentation site source (GitHub Pages)
- `homebrew/` — Local copy of Homebrew formula (canonical copy lives in `m2de/homebrew-tap`)

## Release process

1. Update `version` in `Cargo.toml` and all 6 `npm/*/package.json` files
2. Update `optionalDependencies` versions in `npm/getapi-cli/package.json`
3. Commit and tag: `git tag vX.Y.Z && git push origin master --tags`
4. The release workflow handles building, GitHub Release, crates.io, npm, and Homebrew tap
