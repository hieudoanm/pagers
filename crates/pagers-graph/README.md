# pagers-graph

Page-link graph extraction and rendering. Turns the edges collected during
Markdown rendering (or a standalone directory scan) into an interactive
HTML viewer, JSON data, or Graphviz DOT.

Part of the [pagers](../../README.md) workspace; see [Graph](../../packages/landing/docs/graph.md).

## API

```rust
pub struct Node { /* id (url), title, source */ }
pub struct GraphEdge { /* from, to */ }
pub struct Stats { /* pages, edges, orphaned */ }
pub struct Graph { /* nodes, edges, stats */ }

pub fn from_pages(pages: &[Page], edges: &[Edge]) -> Graph
pub fn scan(root: &str) -> Result<Graph, String>          // standalone: `pagers graph`
pub fn render_html(graph: &Graph, title: &str, theme: &str) -> String
pub fn to_json(graph: &Graph) -> serde_json::Value
pub fn to_dot(graph: &Graph) -> String
```

## Outputs

- `graph.html` — self-contained force-directed canvas viewer (vanilla JS,
  no dependencies): drag, click-for-details, "Open page", dark/light.
- `graph.json` — nodes/edges/stats for tooling.
- `graph.dot` — Graphviz DOT.

Embedded assets (`graph.html` template + `graph.css` + `graph.js`) live in
`assets/` and are `include_str!`'d — same pattern as `pagers-theme`.
