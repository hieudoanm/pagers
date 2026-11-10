# md.css Module — Agent Guide

## Location

This module was migrated into the `pagers` repository (`packages/md.scss`). It
remains a **standalone pnpm project** — the Cargo workspace does not build
it. `pagers` ships a compiled copy embedded at
`crates/pagers-theme/assets/md.css`; after stylesheet changes, run
`pnpm run build` here and copy `dist/md.css` to that path, then rebuild
`pagers`.

## Commands

- `pnpm run build` — full build: `prebuild` = `bash scripts/build.sh clean` (wipes `dist`), then `scripts/build.sh` compiles the single entry `src/md.scss` into `dist/md.css` + `dist/md.css.map` and `dist/md.min.css` + `dist/md.min.css.map`, then `postbuild` = `ts-node scripts/post-build.ts` writes `dist/md.metadata.json` shaped like `metadata.schema.json` (filename → `{ size: bytes, kb: kibibytes }`).
- `pnpm run examples` — `ts-node scripts/examples.ts`: pandoc (GFM → HTML5) over `public/cheat-sheet.md`, a Node post-process pass, then one themed HTML page (`public/html/cheat-sheet.<theme>.html`) and full-page PNG (`public/images/cheat-sheet.<theme>.png`) per theme. Requires `pandoc`; rebuilds `dist` only when `dist/md.css` is missing; Playwright/Chromium optional (skips screenshots with a warning).

## Themes

- There is **one** output file for every theme. `src/md.scss` emits `.md` (light default palette + shared core rules via `@use 'md-core'` + `@include md-core.md-core;`), then a `.md.md-<theme> { @include <theme>.palette; }` block per theme inside the same file.
- Theme classes must be combined on the **same** element: `<div class="md md-dark">` (compound `.md.md-<theme>`), never a descendant `.md .md-dark` — custom properties only take effect on the element carrying both classes.
- Light is the default: plain `.md` needs no extra class; `.md.md-light` is an explicit alias.
- Palette files are `src/themes/<theme>.scss` (no `md.` prefix), each a single `@mixin palette { --md-*: … }` — **no** `@use '../md-core'`, **no** `@include md-core.md-core`. Adding a theme = new mixin file + one `@use` + one `.md.md-<theme>` block in `src/md.scss`.

## Layout

- `src/md.scss` — the only entry built. `@use` lines are kept alphabetical by module path.
- `src/_md-core.scss` — assembles the 12 fragments under `src/core/` into `@mixin md-core` (emits `.md { … }`). Core rules reference tokens via `var(--md-link, var(--md-primary))` style fallbacks: a palette may omit a token and inherit the fallback.
- `src/core/_*.scss` — one named mixin per fragment (headings, inline, lists, blockquote, code, tables, media, misc, footnotes, alerts, highlight, base). Files stay under 200 lines.
- All spacing/borders/radii flow from `--md-*` tokens; never hard-code a colour or radius in core rules.
- Any change to a token name or a core rule must be reflected in **every** theme palette (they should declare the same full set).

## Verification

- After changes: `pnpm run build`, then grep the expanded output, e.g. `grep -nE '\.md\.md-(dark|crimson|cerulean|emerald|saffron|lagoon|fuchsia)' dist/md.css` — each `.md.md-<theme>` block must be present once, and the shared `.md { color: var(--md-text); … }` rules must appear exactly once.
- Link token check: `--md-link` must resolve — light `#2563eb`, dark `#60a5fa`, color themes their signature blue.
- After theme changes, regenerate the examples (`pnpm run examples`) so `public/html/` and `public/images/` stay in sync (8 themes, no default variant).

## Conventions

- Match an existing fragment/theme file before writing a new one (same header comment, mixin shape, token order).
- Arrow functions only in `scripts/examples.ts` and `scripts/post-build.ts`.
- `scripts/examples.ts` runs via `ts-node`; Playwright is located by walking up from the module root for `node_modules/.pnpm/playwright@*/node_modules/playwright` — keep that discovery, do not add a hard dependency.
