# pagers-wiki

Wikipedia URL → clean Markdown. A staged, reusable pipeline:

```text
URL → validate → MediaWiki REST (HTML + metadata)
    → extract .mw-parser-output → clean DOM → htmd → normalize
    → Markdown / JSON
```

Part of the [pagers](../../README.md) workspace; see
[Wikipedia](../../docs/wiki.md). Powers `pagers wiki convert|info` and is
deliberately independent of the site crates — the stages can serve other
content sources later.

## Modules

| Module        | Role                                                              |
| ------------- | ----------------------------------------------------------------- |
| `url.rs`      | `WikiArticle::parse` — validates `*.wikipedia.org/wiki/<title>`, builds API endpoints |
| `client.rs`   | `WikiClient` — `fetch_html` (`page/{title}/html`) + `fetch_info` (`page/{title}`); timeout, UA, retries |
| `extractor.rs`| `extract_article_html` — scraper isolates `.mw-parser-output`      |
| `converter.rs`| `to_markdown` — mutable rcdom cleanup (noise, links → `.md` filenames, MathML→LaTeX) then `htmd` |
| `normalizer.rs`| `normalize` — whitespace, headings, lists, GFM tables, dedupe    |
| `metadata.rs` | `Article`, `ArticleMetadata` — JSON shape, word/char counts, `info` text |
| `error.rs`    | `WikiError` — thiserror variants for every failure mode           |

## API

```rust
pub struct ConvertOptions { pub include_references: bool, /* … */ }

pub fn convert(raw_url: &str, options: &ConvertOptions) -> Result<Article, WikiError>
pub fn info(raw_url: &str) -> Result<ArticleMetadata, WikiError>
pub fn convert_html(html: &str, /* … */) -> /* offline conversion path */
```

`Article::to_json()` produces the `--format json` payload including real
`revision_id`/`revision_timestamp` (null when unavailable — never invented).

## Tests

`tests/` uses a real Parsoid fixture (`tests/fixtures/rust.html`) plus
inline fixtures — **no live network in tests**. `examples/convert.rs`
demos offline conversion.
