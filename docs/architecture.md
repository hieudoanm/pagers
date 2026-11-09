---
title: Architecture
order: 10
---

# Architecture

`pagers` is a Cargo workspace — a virtual root manifest plus one crate per
concern. The dependency direction is acyclic:

```text
pagers-cli ──┐
             ├─→ pagers-build ─→ pagers-markdown ─→ pagers-site
pagers-mcp ──┘       │                ▲
                     ├─→ pagers-theme ─┘
                     └─→ pagers-graph ─→ pagers-markdown

pagers-wiki (standalone: reqwest + scraper + htmd)
```

## Crates

| Crate           | Role                                                        | External deps          |
| --------------- | ----------------------------------------------------------- | ---------------------- |
| `pagers-cli`    | `pagers` binary: clap CLI, build/serve/init/wiki commands | clap, pagers-*         |
| `pagers-mcp`    | `pagers-mcp` binary: stdio MCP server (rmcp)                | rmcp, serde, tokio     |
| `pagers-build`  | Build pipeline: discover → render → write → copy → graph   | pagers-*               |
| `pagers-site`   | Content model: discovery, frontmatter, config, URLs, nav, link resolution | **none**    |
| `pagers-markdown` | Markdown → HTML (pulldown-cmark), outlines, alerts, link rewriting | pulldown-cmark |
| `pagers-theme`  | Embedded `md.css`, site chrome, `page.html` template        | none                   |
| `pagers-graph`  | Page-link graph extraction + HTML/JSON/DOT rendering        | pagers-markdown        |
| `pagers-wiki`   | Wikipedia: URL parse → REST fetch → extract → convert → normalize | reqwest, scraper, htmd, … |

`pagers-build` exists so both the CLI and the MCP server share one pipeline;
`pagers-wiki` is deliberately independent of the site crates — its stages
(url → client → extractor → converter → normalizer → metadata) are reusable
for other content sources.

## Build pipeline

```text
discover(content_dir)
  → Vec<Page> + assets + frontmatter/config
render each page (pagers-markdown → HTML + outline + link edges)
  → write output_path_for_source() HTML files
copy non-Markdown assets to output root
emit graph.html/.json/.dot (pagers-graph)
```

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
and on `main` additionally builds the site you are reading — the converted
`README.md` plus `docs/` — and deploys it to GitHub Pages.
