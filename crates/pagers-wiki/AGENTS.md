# pagers-wiki — Agent Guide

Back to the [workspace README](../../README.md) · [Wikipedia docs](../../docs/wiki.md).

## Conventions

- **Stages stay independent** — url/client/extractor/converter/normalizer/
  metadata/error are separate modules on purpose; don't collapse them.
- HTTP goes through `WikiClient` only: it owns the User-Agent, timeout,
  retry policy (429/5xx/network) and status→error mapping. Never `reqwest`
  directly elsewhere in the crate.
- MediaWiki endpoints: `…/w/rest.php/v1/page/{title}/html` (content) and
  `…/page/{title}` (metadata). `api_title()` percent-encodes the title.
- DOM cleanup mutates htmd's `markup5ever_rcdom` tree **before**
  `tree_to_markdown` — `scraper` is read-only and only used for extraction.
- Internal `/wiki/` links become `<slug>.md` filenames by default
  (`ConvertOptions.local_links`); the absolute URL is kept in the link
  `title` so nothing is lost. `--wiki-links` on the CLI disables this.
  Namespace links (`File:`, `Category:`, …) always stay absolute.
- Cleanup must be conservative: remove chrome (scripts, edit links, nav,
  hidden UI, tracking params) but never meaningful encyclopedic content —
  don't filter on class names alone.
- References are stripped by default; `--include-references` keeps them.
- Fidelity over simplicity: preserve math (`annotation[x-tex]` → `$…$`),
  code, tables, images/captions, nested lists.
- `normalize()` is pure string→string — keep it that way (no I/O).
- **No live network in tests** — use `tests/fixtures/*.html`.

## Verification

```bash
cargo test -p pagers-wiki
cargo clippy -p pagers-wiki
cargo run -p pagers-cli -- wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)" | head
```
