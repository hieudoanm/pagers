# pagers-theme — Agent Guide

Back to the [workspace README](../../README.md) · [Theming docs](../../packages/landing/docs/theming.md).

## Conventions

- All assets are `include_str!`/`include_bytes!` constants — the binary is
  self-contained; never read theme files at runtime.
- `page.html` uses `{{PLACEHOLDER}}`-style vars injected by
  `template::render_page` — keep var names in sync between the two files.
- The template must emit the hooks `script.js` expects (`data-theme` on
  `<html>`, `md-dark` on the content element, theme toggle button) — the
  light/dark machinery lives in JS, not the template.
- `md.css` is generated — **don't hand-edit** `assets/md.css`; edit
  `packages/md.scss/src/**`, rebuild, copy `dist/md.css` over.
- Chrome styles live in `assets/styles.css` with light/dark custom
  properties mirroring `--md-*` tokens — keep the two palettes in sync
  when adding theme colours.

## Verification

```bash
cargo build -p pagers-theme
cargo run -p pagers-cli -- build docs --output /tmp/site   # eyeball output
```
