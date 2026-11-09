---
title: Architecture
order: 10
---

# Architecture

`pagers` is a Cargo workspace — a virtual root manifest plus one crate per
concern. The dependency direction is acyclic:

<svg viewBox="0 0 980 430" width="100%" style="height:auto" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="pagers crate dependency graph">
  <defs>
    <marker id="ag-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M0,0 L10,5 L0,10 z" fill="#64748b"/>
    </marker>
  </defs>
  <style>
    .ag-box { fill: #f8fafc; stroke: #334155; stroke-width: 1.5; }
    .ag-bin { fill: #eff6ff; stroke: #2563eb; }
    .ag-core { fill: #f0fdf4; stroke: #16a34a; }
    .ag-ext  { fill: #fefce8; stroke: #ca8a04; stroke-dasharray: 6 4; }
    .ag-t { font: 600 14px system-ui, sans-serif; fill: #0f172a; text-anchor: middle; }
    .ag-s { font: 11px system-ui, sans-serif; fill: #64748b; text-anchor: middle; }
    .ag-e { stroke: #64748b; stroke-width: 1.5; fill: none; marker-end: url(#ag-arrow); }
  </style>
  <!-- edges -->
  <path class="ag-e" d="M170,60 L490,116"/>
  <path class="ag-e" d="M810,60 L490,116"/>
  <path class="ag-e" d="M490,164 L145,248"/>
  <path class="ag-e" d="M490,164 L415,248"/>
  <path class="ag-e" d="M490,164 L685,248"/>
  <path class="ag-e" d="M330,276 L234,276"/>
  <path class="ag-e" d="M145,300 L330,368"/>
  <path class="ag-e" d="M415,300 L450,368"/>
  <path class="ag-e" d="M685,300 L540,368"/>
  <path class="ag-e" d="M640,318 C520,352 300,352 215,304"/>
  <!-- binaries -->
  <rect class="ag-box ag-bin" x="80" y="16" width="180" height="44" rx="8"/>
  <text class="ag-t" x="170" y="43">pagers-cli</text>
  <rect class="ag-box ag-bin" x="720" y="16" width="180" height="44" rx="8"/>
  <text class="ag-t" x="810" y="43">pagers-mcp</text>
  <!-- pipeline -->
  <rect class="ag-box" x="400" y="120" width="180" height="44" rx="8"/>
  <text class="ag-t" x="490" y="147">pagers-build</text>
  <!-- libraries -->
  <rect class="ag-box" x="60" y="252" width="170" height="48" rx="8"/>
  <text class="ag-t" x="145" y="272">pagers-markdown</text>
  <text class="ag-s" x="145" y="290">pulldown-cmark</text>
  <rect class="ag-box" x="330" y="252" width="170" height="48" rx="8"/>
  <text class="ag-t" x="415" y="272">pagers-theme</text>
  <text class="ag-s" x="415" y="290">embedded assets</text>
  <rect class="ag-box" x="600" y="252" width="170" height="48" rx="8"/>
  <text class="ag-t" x="685" y="272">pagers-graph</text>
  <text class="ag-s" x="685" y="290">html / json / dot</text>
  <!-- foundation -->
  <rect class="ag-box ag-core" x="305" y="372" width="370" height="44" rx="8"/>
  <text class="ag-t" x="490" y="399">pagers-site — content model, zero deps</text>
  <!-- standalone -->
  <rect class="ag-box ag-ext" x="790" y="120" width="170" height="48" rx="8"/>
  <text class="ag-t" x="875" y="140">pagers-wiki</text>
  <text class="ag-s" x="875" y="158">reqwest · scraper · htmd</text>
</svg>

## Crates

| Crate             | Role                                                                      | External deps             |
| ----------------- | ------------------------------------------------------------------------- | ------------------------- |
| `pagers-cli`      | `pagers` binary: clap CLI, build/serve/init/wiki commands                 | clap, pagers-*            |
| `pagers-mcp`      | `pagers-mcp` binary: stdio MCP server (rmcp)                              | rmcp, serde, tokio        |
| `pagers-build`    | Build pipeline: discover → render → write → copy → graph                  | pagers-*                  |
| `pagers-site`     | Content model: discovery, frontmatter, config, URLs, nav, link resolution | **none**                  |
| `pagers-markdown` | Markdown → HTML (pulldown-cmark), outlines, alerts, link rewriting        | pulldown-cmark            |
| `pagers-theme`    | Embedded `md.css`, site chrome, `page.html` template                      | none                      |
| `pagers-graph`    | Page-link graph extraction + HTML/JSON/DOT rendering                      | pagers-markdown           |
| `pagers-wiki`     | Wikipedia: URL parse → REST fetch → extract → convert → normalize         | reqwest, scraper, htmd, … |

`pagers-build` exists so both the CLI and the MCP server share one pipeline;
`pagers-wiki` is deliberately independent of the site crates — its stages
(url → client → extractor → converter → normalizer → metadata) are reusable
for other content sources.

## Build pipeline

<svg viewBox="0 0 1000 120" width="100%" style="height:auto" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="pagers build pipeline">
  <defs>
    <marker id="pp-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto">
      <path d="M0,0 L10,5 L0,10 z" fill="#64748b"></path>
    </marker>
  </defs>
  <style>
    .pp-step { fill: #f8fafc; stroke: #334155; stroke-width: 1.5; }
    .pp-t { font: 600 13px system-ui, sans-serif; fill: #0f172a; text-anchor: middle; }
    .pp-s { font: 11px system-ui, sans-serif; fill: #64748b; text-anchor: middle; }
    .pp-a { stroke: #64748b; stroke-width: 1.5; fill: none; marker-end: url(#pp-arrow); }
  </style>
  <rect class="pp-step" x="10"  y="24" width="170" height="64" rx="8"/>
  <text class="pp-t" x="95"  y="52">discover</text>
  <text class="pp-s" x="95"  y="72">pages + assets + config</text>
  <path class="pp-a" d="M184,56 L206,56"></path>
  <rect class="pp-step" x="210" y="24" width="170" height="64" rx="8"/>
  <text class="pp-t" x="295" y="52">render</text>
  <text class="pp-s" x="295" y="72">markdown → html + edges</text>
  <path class="pp-a" d="M384,56 L406,56"></path>
  <rect class="pp-step" x="410" y="24" width="170" height="64" rx="8"/>
  <text class="pp-t" x="495" y="52">write pages</text>
  <text class="pp-s" x="495" y="72">trailing-slash html</text>
  <path class="pp-a" d="M584,56 L606,56"></path>
  <rect class="pp-step" x="610" y="24" width="170" height="64" rx="8"/>
  <text class="pp-t" x="695" y="52">copy assets</text>
  <text class="pp-s" x="695" y="72">non-markdown files</text>
  <path class="pp-a" d="M784,56 L806,56"></path>
  <rect class="pp-step" x="810" y="24" width="180" height="64" rx="8"/>
  <text class="pp-t" x="900" y="52">emit graph</text>
  <text class="pp-s" x="900" y="72">graph.html · meta files</text>
</svg>

Everything is deterministic and offline — no network access during a build.

## Conventions

- Edition 2021/2024 idioms, `thiserror` for library errors, `clap` derive
  for the CLI.
- No `unsafe`, no `unwrap()`/`expect()` on fallible paths in library code.
- Small focused modules: `pagers-wiki` splits url/client/extractor/
  converter/normalizer/metadata/error into separate files.
- Tests live beside the code (`#[cfg(test)]`) plus `tests/` integration
  suites with local fixtures — **no live network in tests**.

## Developing

```bash
cargo build                  # debug
cargo build --release        # release → target/release/pagers
cargo test --workspace       # all crate tests
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

CI (`.github/workflows/ci.yaml`) runs fmt/clippy/tests on every push and PR,
and on `master` additionally builds the site you are reading — the converted
`README.md` plus `docs/` — and deploys it to GitHub Pages.
