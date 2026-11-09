---
title: Documentation
order: 0
---

# pagers documentation

`pagers` turns a folder of Markdown files into a complete static website —
clean URLs, generated navigation, light/dark themes, and an interactive
graph of how pages link together. No build pipeline, no runtime, no
configuration file required: **one Rust binary**.

```bash
pagers build ./docs
```

## Quick start

```bash
pagers init my-site          # scaffold docs/index.md + an example page
pagers serve my-site/docs    # build + serve on http://localhost:8080
pagers build my-site/docs --output my-site/dist
```

See [Getting started](getting-started.md) for installation and your first
site.

## Features

- **[Zero configuration](features.md#zero-configuration)** — the directory
  layout is the config; frontmatter covers ordering, labels and drafts.
- **[Clean URLs](features.md#clean-urls-zero-rewrite-rules)** — every page
  publishes at `/section/page/`, so `dist/` works on any static host.
- **[File-tree navigation](features.md#file-tree-navigation)** — sidebar
  generated from folders and frontmatter.
- **[GitHub-flavoured Markdown](features.md#github-flavoured-markdown-extended)**
  — tables, alerts, footnotes, heading ids and rewritten internal links.
- **[Eight themes](features.md#themes-out-of-the-box)** — palettes embedded
  in the binary, plus a persisted light/dark toggle.
- **[Graph page](features.md#the-graph-page)** — interactive map of
  page-to-page links on every build.
- **[Wikipedia import](features.md#wikipedia-markdown)** — article URL →
  clean Markdown, with a BFS crawler for whole sections.
- **[MCP server](features.md#mcp-server)** — `pagers-mcp` exposes build and
  inspection tools to AI agents over stdio.

## Guides

- **[Getting started](getting-started.md)** — install, scaffold, build.
- **[Commands](commands.md)** — every CLI command and flag.
- **[Configuration](configuration.md)** — frontmatter, `pagers.yaml`,
  `basePath`, URL rules.
- **[Markdown](markdown.md)** — GFM syntax, heading ids, alerts, link
  rewriting.
- **[Theming](theming.md)** — palettes, tokens, the dark-mode toggle.
- **[Graph](graph.md)** — the page-link graph and `pagers graph` outputs.
- **[Wikipedia](wiki.md)** — `pagers wiki convert` / `pagers wiki info`.
- **[MCP server](mcp.md)** — client setup and the five `pagers_*` tools.
- **[Scripts](scripts.md)** — installers and the `examples.sh` crawler.
- **[Deploy](deploy.md)** — GitHub Pages workflow and other static hosts.

## Internals

- **[Architecture](architecture.md)** — the Cargo workspace, the build
  pipeline, and how to develop `pagers` itself.
