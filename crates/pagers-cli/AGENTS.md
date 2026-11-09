# pagers-cli — Agent Guide

Back to the [workspace README](../../README.md) · [docs](../../docs/).

## Conventions

- This crate is a **binary** — keep it thin. Business logic belongs in the
  library crates (`pagers-build`, `pagers-wiki`, `pagers-site`, …); the CLI
  only parses args, calls the library, formats output, sets exit codes.
- clap derive for all arguments; match the existing command/flag naming
  (`--output`, `--format`, `--quiet`).
- Errors print as `pagers <area>: <message>` on stderr and exit non-zero.
- `serve.rs` must stay `basePath`-aware: strip the configured prefix before
  resolving files, and redirect `/dir` → `/dir/` for clean URLs.

## Verification

```bash
cargo build -p pagers-cli        # produces target/{debug,release}/pagers
cargo clippy -p pagers-cli
cargo run -p pagers-cli -- --help
```
