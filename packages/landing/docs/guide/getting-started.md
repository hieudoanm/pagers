---
title: Getting started
order: 1
---

# Getting started

## Install

Pick whichever fits your machine — all of them produce the same two binaries,
`pagers` and `pagers-mcp`.

### From a clone

```bash
git clone https://github.com/hieudoanm/pagers
cd pagers
./scripts/install-local.sh
```

Builds the release binaries and installs them into `~/.local/bin` (override with
`PREFIX=/path`). Requires a Rust toolchain.

### From anywhere, via curl

```bash
curl -fsSL https://raw.githubusercontent.com/hieudoanm/pagers/main/scripts/install.sh | sh
```

Downloads the source, builds it, and installs the binaries — same result as the
local script. See [Scripts](../project/scripts.md) for `PAGERS_REF`,
`PAGERS_REPO` and `PREFIX` options.

### With Cargo

```bash
cargo install --path crates/pagers-cli   # pagers
cargo install --path crates/pagers-mcp   # pagers-mcp
```

## Scaffold a site

```bash
pagers init my-site
```

Creates `my-site/docs/` with an index page and an example article.

## Build it

```bash
pagers build my-site/docs
# or explicitly:
pagers build my-site/docs --output my-site/dist
```

The directory layout _is_ the configuration:

```text
my-site/docs/
├── index.md           → /
├── guide/
│   ├── index.md       → /guide/
│   └── install.md     → /guide/install/
└── assets/logo.png    → copied through as an asset
```

Open `dist/index.html` in a browser, or run `pagers serve my-site/docs` to build
and serve it on `localhost:8080`.

## What's next

- [Commands](commands.md) — the full CLI surface.
- [Configuration](configuration.md) — titles, ordering, drafts, `basePath`.
- [Markdown](../content/markdown.md) — what the renderer supports.
