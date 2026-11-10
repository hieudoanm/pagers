# pagers-markdown

Markdown → HTML rendering, built on `pulldown-cmark`. One pass over the
event stream produces the page HTML **and** the metadata the rest of the
site needs: heading outline, resolved page links, and graph edges.

Part of the [pagers](../../README.md) workspace; see
[Markdown features](../../packages/landing/docs/markdown.md).

## API

```rust
pub struct Heading { pub level: u8, pub id: String, pub text: String }
pub struct Rendered { /* html, outline: Vec<Heading>, edges, title */ }

pub fn render(
    source: &str,                    // source path, for relative link resolution
    body: &str,                      // markdown body (frontmatter already split)
    resolver: Option<&Resolver>,     // internal link rewriting
) -> Rendered
```

## What the single pass does

- Automatic heading ids + `{#custom-id}` attributes.
- Rewrites internal `.md`/directory links through the `Resolver` to clean
  trailing-slash URLs; external and broken links untouched.
- GitHub alert blockquotes (`> [!NOTE]`) → `.markdown-alert` markup that
  `md.css` styles.
- Lifts a leading `#` heading into the page title.
- Collects `Vec<Heading>` (TOC) and `Vec<Edge>` (page→page links) for
  `pagers-graph`.
- GFM tables, task lists, strikethrough, footnotes via pulldown-cmark
  options; raw HTML passes through.
