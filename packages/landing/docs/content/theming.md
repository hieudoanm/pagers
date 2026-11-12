---
title: Theming
order: 5
---

# Theming

Markdown typography and palettes come from **md.css** — a single-entry Sass
module migrated into this repo at `packages/md.scss`. The compiled stylesheet is
embedded in the `pagers` binary, so built sites are fully self-contained (no
CDN, no runtime fetches).

## Palettes

One class switches themes: plain `.md` is the light default; appending
`.md-<theme>` selects another palette. All eight ship in the binary:

| Theme    | Class             | Scheme |
| -------- | ----------------- | ------ |
| light    | `.md` (default)   | light  |
| dark     | `.md.md-dark`     | dark   |
| crimson  | `.md.md-crimson`  | dark   |
| cerulean | `.md.md-cerulean` | light  |
| emerald  | `.md.md-emerald`  | light  |
| saffron  | `.md.md-saffron`  | light  |
| lagoon   | `.md.md-lagoon`   | dark   |
| fuchsia  | `.md.md-fuchsia`  | dark   |

Every colour and radius resolves through `--md-*` custom properties — see
`packages/md.scss/README.md` for the full token list and `AGENTS.md` for how to
add a theme.

## Light / dark toggle

The site chrome has its own light/dark tokens alongside the Markdown ones. The
header toggle:

- flips `data-theme` on `<html>` and `md-dark` on the content element,
- persists the choice in `localStorage`,
- respects `prefers-color-scheme` on first visit.

Set the initial theme with the `theme` site field (`light` or `dark`) — see
[Configuration](../guide/configuration.md).

## Site chrome

`styles.css` (header, sidebar nav, footer, graph viewer chrome) uses its own CSS
custom properties mirroring the same light/dark split, so chrome and content
always agree.

## Rebuilding md.css

`packages/md.scss` is a standalone pnpm project:

```bash
cd packages/md.scss
pnpm install
pnpm run build      # → dist/md.css + dist/md.min.css (+ source maps)
pnpm run examples   # regenerate themed cheat-sheet HTML/PNG examples
```

The binary embeds `crates/pagers-theme/assets/md.css`; copy a fresh
`dist/md.css` there and rebuild `pagers` to ship an updated stylesheet.
