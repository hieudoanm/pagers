---
title: Wikipedia
order: 7
---

# Wikipedia → Markdown

`pagers wiki` turns a Wikipedia article URL into clean, well-structured
Markdown — useful for archiving articles into a `pagers` site or feeding
them to an LLM. No API key required; it uses the public MediaWiki REST API.

```bash
pagers wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)"
pagers wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)" -o rust.md
pagers wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)" --format json
pagers wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)" --include-references
pagers wiki info "https://en.wikipedia.org/wiki/Rust_(programming_language)"
```

## How it works

The pipeline lives in `crates/pagers-wiki` as independent stages — the same
stages could serve other sites later:

1. **URL parsing** — accepts `https://<lang>.wikipedia.org/wiki/<title>`,
   percent-decodes titles, supports every Wikipedia language domain.
2. **Client** — MediaWiki REST (`/w/rest.php/v1/page/{title}/html` plus a
   metadata call), 30s timeout, descriptive User-Agent, retries on
   429/5xx/network errors. 404/403/429 produce specific error messages.
3. **Extraction** — `scraper` isolates `.mw-parser-output`, discarding
   navigation, sidebar, edit links and other site chrome.
4. **Conversion** — a mutable DOM cleanup pass (noise removal, section
   unwrapping, link absolutizing, `utm_*` stripping, MathML→LaTeX) then
   `htmd` HTML→Markdown.
5. **Normalization** — whitespace collapse, heading/list/table cleanup,
   GFM table re-padding, empty-element removal, optional paragraph dedupe.

## Output

Default output starts with the article title as `# Heading` and **omits
references**. `--include-references` keeps the citations section.

`--format json` returns (excerpt):

```json
{
  "title": "Rust (programming language)",
  "file_name": "rust_programming_language.md",
  "url": "https://en.wikipedia.org/wiki/Rust_(programming_language)",
  "language": "en",
  "revision_id": "…",
  "revision_timestamp": "…",
  "markdown": "# Rust (programming language)\n\n…",
  "metadata": { "word_count": 7502, "character_count": 51234 }
}
```

`revision_id`/`revision_timestamp` come from the API — they're `null` when
the endpoint doesn't provide them, never invented. `file_name` is the
suggested on-disk name, derived from the title (`Rust (programming
language)` → `rust_programming_language.md`).

`-o` accepts a file **or a directory** — a directory gets the suggested
filename appended:

```bash
pagers wiki convert "<url>" -o ./articles/   # → ./articles/rust_programming_language.md
```

## Preserved vs. removed

**Preserved:** headings, paragraphs, nested lists, tables/infoboxes (as
compact GFM tables), images with captions, internal links (rewritten to
local `<slug>.md` filenames with the source URL kept in the link title —
`[t](concurrent_computing.md "https://…")`; `--wiki-links` keeps absolute
`https://<lang>.wikipedia.org/wiki/…` URLs instead), external links, math
(`$…$` / `$$…$$`), code blocks.

**Removed:** scripts/styles, edit links, navigation, hidden UI, maintenance
messages, tracking parameters, reference markers (unless requested).

## Errors

Invalid URL, unsupported domain, missing article, rate limiting, timeouts
and empty content all exit non-zero with a descriptive `pagers wiki:` error.
Be polite: the [examples.sh crawler](../project/scripts.md#examplessh--wikipedia-crawler--demo-site)
spaces requests 1s apart by default.
