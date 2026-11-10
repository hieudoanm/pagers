# pagers-site — Agent Guide

Back to the [workspace README](../../README.md) · [Architecture](../../packages/landing/docs/architecture.md).

## Hard rules

- **Zero external dependencies** — this crate is the workspace's foundation;
  keep it that way (pure `std`).
- Frontmatter is a **flat `key: value` subset**, not YAML — `parse_block`
  skips comments/blank lines, unquotes values, lowercases keys. Don't add
  nested structures.
- URL mapping is load-bearing: `url_for_source` produces trailing-slash
  URLs that `serve`, the resolver and the graph all assume. Changing it
  changes the whole site's public URLs.
- `resolve.rs` must stay conservative: only rewrite destinations that map
  to a known page; leave everything else untouched so broken links stay
  visible.
- `escape_html` lives here because theme and graph both need it — don't
  duplicate escapers elsewhere.

## Conventions

- `*_pub` re-exports (`dir_of_pub`, `title_from_source_pub`, …) exist for
  `pagers-graph`'s standalone scanner — prefer them over making internals
  `pub`.
- Tests are inline `#[cfg(test)]` modules; keep them dependency-free.

## Verification

```bash
cargo test -p pagers-site
cargo clippy -p pagers-site
```
