# pagers-build

The site build pipeline, extracted from the CLI so both `pagers` and
`pagers-mcp` share one implementation:

```text
discover → render pages → write HTML → copy assets → emit graph
```

Part of the [pagers](../../README.md) workspace; see
[Architecture](../../packages/landing/docs/architecture.md).

## API

```rust
pub struct Stats { /* pages, assets, output dir, elapsed */ }

pub fn build(content_dir: &str, output_dir: &str) -> Result<Stats, String>
```

That's the whole surface — `build` is synchronous and offline; all errors
are `String`s formatted for CLI display.

## Pipeline detail

1. `pagers_site::site::discover` — pages, assets, config, frontmatter.
2. `pagers_markdown::render` per page — HTML, heading outline, link edges.
3. Write `output_path_for_source()` files via `pagers_theme::template`.
4. Copy non-Markdown assets to the output root.
5. `pagers_graph` — `graph.html`, `graph.json`, `graph.dot` at site root.
