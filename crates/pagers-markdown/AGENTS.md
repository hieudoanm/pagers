# pagers-markdown — Agent Guide

Back to the [workspace README](../../README.md) · [Markdown docs](../../packages/landing/docs/markdown.md).

## Conventions

- **One pass, three outputs** — `render()` returns HTML + outline + edges
  from a single event-stream walk. Don't add a second parse for metadata;
  extend the existing pass.
- Link rewriting goes through `pagers_site::resolve::Resolver` — never
  string-replace URLs; resolution must match `url_for_source` semantics.
- Alert markup must stay `.markdown-alert`-compatible with `md.css`
  (`packages/md.scss`); check `src/core/_alerts.scss` before changing the
  emitted classes.
- Heading ids: auto-slug + `{#custom-id}` — keep ids stable, they're public
  anchor links.
- Only dependency is `pulldown-cmark` — resist adding markdown libraries.

## Verification

```bash
cargo test -p pagers-markdown
cargo clippy -p pagers-markdown
```
