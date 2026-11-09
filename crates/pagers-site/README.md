# pagers-site

The content model — **zero external dependencies**. Everything the rest of
the workspace needs to know about a site lives here: file discovery,
frontmatter, `pagers.yaml` config, URL/output mapping, navigation trees and
internal link resolution.

Part of the [pagers](../../README.md) workspace; see
[Configuration](../../docs/configuration.md) and
[Markdown → links](../../docs/markdown.md).

## Modules

| Module       | Contents                                                        |
| ------------ | --------------------------------------------------------------- |
| `site.rs`    | `Page`, `Content`, `discover()`, `url_for_source()`, `output_path_for_source()` |
| `frontmatter.rs` | `Frontmatter`, `split()`, `parse_block()` — flat `key: value` YAML subset |
| `config.rs`  | `Config` (title/language/basePath/theme/footer), `pagers.yaml` loading |
| `nav.rs`     | `Nav`/`Group` tree built from the page list                      |
| `resolve.rs` | `Resolver`/`Resolution` — maps `[x](other.md)` destinations to site URLs |
| `util.rs`    | `escape_html` (shared by theme + graph crates)                   |

## Key API

```rust
let content = pagers_site::site::discover("docs")?;
// content.pages: Vec<Page>, content.assets, content.config, content.nav

pagers_site::site::url_for_source("guide/install.md");   // "/guide/install/"
pagers_site::site::output_path_for_source("index.md");   // "index.html"
```

URL rules: `index.md`/`README.md` → directory landing pages with trailing
slashes; non-Markdown files pass through as assets.
