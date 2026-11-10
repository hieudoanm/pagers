---
title: Features
order: 1
---

# Features

Everything `pagers` does, in one place. Each feature links to the page that
documents it in detail.

## Zero configuration

Point `pagers build` at a folder of Markdown and get a site. The directory
layout **is** the configuration — no `config.js`, no plugin list, no build
pipeline to maintain. See [Configuration](configuration.md).

```text
docs/
├── index.md           → /
├── guide/
│   ├── index.md       → /guide/
│   └── install.md     → /guide/install/
└── assets/logo.png    → copied through
```

## Clean URLs, zero rewrite rules

Every page publishes at a trailing-slash URL (`/guide/install/` →
`guide/install/index.html`), so the output works on **any** static host —
GitHub Pages, Cloudflare Pages, Netlify, S3, a plain `python3 -m http.server`
— with no `_redirects` file or server config. See
[Commands](commands.md#pagers-build-content).

## File-tree navigation

The sidebar is generated from the directory structure: folders become
groups, `index.md`/`README.md` become landing pages, frontmatter `order`
sorts siblings, `label` overrides the nav text, and `draft: true` hides a
page without deleting it. See [Configuration](configuration.md#frontmatter).

## GitHub-flavoured Markdown, extended

Tables, task lists, footnotes, strikethrough, code blocks, raw HTML — plus
heading `{#custom-ids}` and GitHub-style alerts (`> [!NOTE]`). Internal
`.md` links are rewritten to clean URLs at build time; broken links stay
visible. See [Markdown](../content/markdown.md).

## Themes out of the box

Eight palettes ship inside the binary — light, dark, crimson, cerulean,
emerald, saffron, lagoon, fuchsia — plus a light/dark toggle that persists
in `localStorage` and respects `prefers-color-scheme`. No CDN, no runtime
fetches: built sites are fully self-contained. See [Theming](../content/theming.md).

## The graph page

Every build writes an interactive `graph.html`: a force-directed map of how
every page links to every other, with node details and theme support. It's
linked from every page's header, and `pagers graph` emits the same data as
standalone HTML, JSON or Graphviz DOT. See [Graph](../content/graph.md).

## `basePath` for sub-path hosting

One field (`basePath: /pagers`) prefixes every generated URL — links,
assets, the graph page — so the same `dist/` deploys correctly under a
project path or a custom domain path. See
[Configuration](configuration.md#pagersyaml).

## Wikipedia → Markdown

`pagers wiki convert` turns any `*.wikipedia.org/wiki/…` article into clean
Markdown through the MediaWiki REST API — metadata, references, MathML,
internal links rewritten to local `.md` filenames. `pagers wiki info`
returns title, description, revision id and timestamp as JSON.
`scripts/examples.sh` crawls a whole link graph from a seed article. See
[Wikipedia](../content/wiki.md) and [Scripts](../project/scripts.md).

## MCP server

`pagers-mcp` speaks the Model Context Protocol over stdio, exposing
`pagers_build`, `pagers_list_pages`, `pagers_render_markdown`,
`pagers_site_graph` and `pagers_site_config` — so AI agents can build and
inspect sites directly. See [MCP server](../project/mcp.md).

## One binary, no runtime

Rust, std-heavy dependencies, fully static output. `cargo install`, a curl
installer, or `cargo build --release` — then it runs anywhere a static
server does. See [Architecture](../project/architecture.md).
