# md.css

> Lives in the [`pagers`](../..) repository at `packages/md.css`. It is a
> standalone pnpm project — the Cargo workspace ignores it. `pagers` embeds
> a compiled copy at `crates/pagers-theme/assets/md.css`; after changing
> styles here, run `pnpm run build` and copy `dist/md.css` there to ship the
> update in the binary.

A single-entry Markdown stylesheet. The GitHub-flavoured typography rules are defined **once** under `.md` (`src/core/`), and every colour/radius resolves through `--md-*` custom properties, so a theme is just a palette class on the same element — no per-variant CSS files, no duplicated rules.

Usage:

```html
<link rel="stylesheet" href="dist/md.css" />

<!-- default light theme -->
<div class="md">
  <h1>Hello</h1>
</div>

<!-- any other theme by appending its class -->
<div class="md md-dark">
  <h1>Hello</h1>
</div>
```

## Table of Contents

- [md.css](#mdcss)
  - [Table of Contents](#table-of-contents)
  - [Usage](#usage)
    - [Themes](#themes)
    - [Design tokens](#design-tokens)
  - [Build](#build)
  - [Examples](#examples)
  - [Layout](#layout)

---

## Usage

One class switches themes: plain `.md` is the light default; appending `.md-<theme>` (both classes on the **same** element) selects another palette. The core rules are shared, so there is exactly one stylesheet (`dist/md.css`) for every theme.

### Themes

| Theme           | Class                 | Scheme |
| --------------- | --------------------- | ------ |
| light (default) | `.md`, `.md.md-light` | light  |
| dark            | `.md.md-dark`         | dark   |
| crimson         | `.md.md-crimson`      | dark   |
| cerulean        | `.md.md-cerulean`     | light  |
| emerald         | `.md.md-emerald`      | light  |
| saffron         | `.md.md-saffron`      | light  |
| lagoon          | `.md.md-lagoon`       | dark   |
| fuchsia         | `.md.md-fuchsia`      | dark   |

### Design tokens

Every palette sets the same `--md-*` tokens; override any of them on `.md` for a local tweak. Consumers of the core rules fall back to `var(--md-link, var(--md-primary))` etc., so a palette may omit a token to inherit the fallback.

- Text: `--md-text`, `--md-text-muted`
- Surfaces: `--md-border`, `--md-code-bg`, `--md-code-block-bg`, `--md-table-alt-bg`, `--md-quote-bg`, `--md-mark-bg`
- Brand: `--md-primary`, `--md-link`, `--md-secondary`, `--md-accent`
- Semantic: `--md-info`, `--md-success`, `--md-warning`, `--md-error`
- Radii: `--md-radius-box`

## Build

```sh
pnpm run build
```

`prebuild` wipes `dist` (`bash scripts/build.sh clean`), then `scripts/build.sh` compiles the single entry `src/md.scss` into `dist/md.css` (expanded) and `dist/md.min.css` (compressed), each with a source map. Finally `ts-node scripts/post-build.ts` writes `dist/md.metadata.json` shaped like `metadata.schema.json` (filename → `{ size: bytes, kb: kibibytes }`).

## Examples

```sh
pnpm run examples
```

`ts-node scripts/examples.ts` regenerates the themed cheat-sheet examples: pandoc (GFM → HTML5) over `public/cheat-sheet.md`, a Node post-process pass (task lists, footnotes, sub/sup, `<mark>`, definition lists, heading anchors, JSON syntax highlights), then one page per theme in `public/html/` and a full-page screenshot in `public/images/` via headless Chromium.

Requires `pandoc` (and `sass` only when `dist/md.css` is missing). Playwright is optional — when absent or without a cached Chromium install, the script regenerates HTML and skips the screenshots with a warning.

## Layout

- `src/md.scss` — entry: `.md` = light palette + core rules, then one `.md.md-<theme>` block per palette.
- `src/_md-core.scss` — aggregator: `@use`s the 12 core fragments and exposes `@mixin md-core` (emits the `.md { … }` ruleset).
- `src/core/_*.scss` — grouped rule fragments (headings, inline, code, tables, media, …), each a named mixin; assembled only by `_md-core`.
- `src/themes/_*.scss` — one pure `@mixin palette` per theme, defining only `--md-*` tokens.
- `scripts/build.sh` — Sass compile of `src/md.scss` into `dist/`.
- `scripts/post-build.ts` — writes `dist/md.metadata.json`.
- `scripts/examples.ts` — keeps `public/` in sync with `public/cheat-sheet.md`.
