---
title: Graph
order: 6
---

# The graph page

Every `pagers build` writes **`graph.html`** at the site root — an interactive,
dependency-free force-directed map of how pages link to each other. It's linked
from the site header on every page.

## What it shows

- One node per content page, sized/labelled by title.
- Directed edges for internal Markdown links (`[x](other.md)` and resolved
  clean-URL links).
- Drag nodes to rearrange; click a node for its title, URL and outgoing links;
  "Open page" jumps to the page.
- Dark/light theming follows the site toggle.

The renderer (`crates/pagers-graph`) extracts edges during the build from the
same link-resolution pass that rewrites URLs, so the graph always matches what
readers can actually click.

## Standalone graph

`pagers graph` scans a directory and writes the same data without building the
site — useful for auditing link structure or embedding elsewhere:

```bash
pagers graph ./docs
# → graph.html, graph.json, graph.dot
```

- `graph.html` — self-contained viewer (open directly in a browser).
- `graph.json` — `{ nodes: [...], edges: [...], stats: {...} }` for tooling.
- `graph.dot` — Graphviz DOT if you prefer `dot -Tsvg`.

## Reading the JSON

```json
{
  "nodes": [
    {
      "id": "/guide/install/",
      "title": "Installation",
      "source": "guide/install.md"
    }
  ],
  "edges": [{ "from": "/", "to": "/guide/install/" }],
  "stats": { "pages": 12, "edges": 30, "orphaned": 1 }
}
```

Orphaned pages (no inbound links) and dangling references are the usual
actionable findings.
