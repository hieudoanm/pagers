# pagers-cli

The `pagers` command-line binary — clap-based entry point that wires the
library crates together: `build`, `init`, `serve`, `graph`, and the
`wiki convert`/`wiki info` commands.

Part of the [pagers](../../README.md) workspace; see the
[documentation](../../docs/) for the user-facing guide.

## Commands

| Module | Command |
| --- | --- |
| `main.rs` | clap definitions + dispatch |
| `build.rs` | thin wrapper over `pagers-build::build` |
| `init.rs` | `pagers init` site scaffolding |
| `serve.rs` | `pagers serve` static file server (basePath-aware, trailing-slash redirects) |
| `wiki.rs` | `pagers wiki convert`/`info` CLI surface over `pagers-wiki` |

## Usage

```bash
cargo run -p pagers-cli -- build ./docs --output ./dist
cargo run -p pagers-cli -- wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)"
```

The package ships `[[bin]] name = "pagers"` — the binary is `pagers`, not
`pagers-cli`.
