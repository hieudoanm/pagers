---
title: Landing pages
order: 4
---

# Landing pages

`pagers landing` generates a **single-page static site** from one `landify.yaml`
— no Node.js, no JavaScript runtime, no build step beyond the CLI itself. It is
a port of the Go `landify` tool: strict YAML in, self-contained HTML/CSS/SVG
out.

```bash
pagers landing new --type product   # scaffold an annotated landify.yaml
pagers landing validate             # strict schema check
pagers landing build -o public/     # render the site
pagers landing preview --dir public # serve it locally
pagers landing themes               # list the 64 built-in presets
```

## Page types

Twelve layouts cover the common landing-page shapes. `new -t <type>` scaffolds a
commented starter config for each:

| Type        | Use for                               |
| ----------- | ------------------------------------- |
| `product`   | SaaS/product marketing page (default) |
| `waitlist`  | Email capture before launch           |
| `event`     | Conference, meetup, webinar           |
| `download`  | App/binary download page              |
| `app`       | Mobile/desktop app showcase           |
| `pricing`   | Plans and tiers                       |
| `portfolio` | Personal portfolio                    |
| `docs`      | Documentation/product docs landing    |
| `faq`       | FAQ / knowledge base                  |
| `team`      | Team page                             |
| `status`    | Status page                           |
| `linktree`  | Link-in-bio page                      |

Every page is fully static — no JavaScript at all. Navigation, mobile menu, and
theme are pure CSS driven by custom properties.

## Theming

Two ways to set colors:

- **`theme: <name>`** — one of 64 built-in presets (`pagers landing themes`
  lists them all; lookup is case-insensitive). `--theme <name>` on `build`
  overrides the YAML value.
- **`theme:` block** — explicit `base`, `primary`, `secondary`, `neutral`,
  `info`, `warning`, `success`, `error`, and `radius` values. Empty fields merge
  with defaults.

From those eight colors the build derives a full token set — surface
tints/shades, readable foregrounds, borders, and semantic states — all computed
for WCAG contrast and emitted as plain hex `--tokens` in the page's inline
`<style>`. No `color-mix()`, no runtime theming.

## `landify.yaml` structure

```yaml
site:
  name: My Product
  description: One-line pitch
  url: https://example.com # used for sitemap/robots/OG
  mark: '🚀' # favicon + OG monogram
  og: # optional social card
    title: …
    description: …
    image: og.png
nav:
  - label: Docs
    url: https://docs.example.com
theme: ocean # or a theme: {…} block
hero: … # type-specific sections follow
```

Unknown fields are rejected — `validate` catches typos before `build`.

## Build output

`pagers landing build -o public/index.html` writes into the output directory:

| File               | Contents                                                                                         |
| ------------------ | ------------------------------------------------------------------------------------------------ |
| `index.html`       | The rendered page — inline `<style>` carries only the `:root` theme tokens derived from `theme:` |
| `styles.css`       | Shared stylesheet for all 12 page types (base + per-type rules)                                  |
| `og/og.svg`        | 1200×630 social card, only when `site.og` is configured                                          |
| `favicon.svg`      | Theme-colored monogram (site `mark` or initial)                                                  |
| `robots.txt`       | Allow-all + `Sitemap:` line when `site.og.url` is set                                            |
| `sitemap.xml`      | Single canonical URL with a trailing slash                                                       |
| `humans.txt`       | Site credits + generation year                                                                   |
| `site.webmanifest` | Web app manifest (name, colors, SVG icon)                                                        |
| `llms.txt`         | llmstxt.org-style index for LLM crawlers                                                         |

Every file except `index.html` is **write-if-absent**: a `robots.txt`,
`favicon.svg`, or `styles.css` you place in the output directory is never
overwritten — drop in a customized file and rebuild freely. Delete it to
regenerate the default on the next build. `index.html` itself always reflects
the current `landify.yaml`.

The generated `<head>` already links `favicon.svg`, `site.webmanifest`, and
`theme-color`, and carries Open Graph/Twitter tags when `site.og` is set. To
rasterize the card to PNG for social previews:

```bash
rsvg-convert -o public/og/og.png public/og/og.svg
```

The
[`examples/landing`](https://github.com/hieudoanm/pagers/tree/master/examples/landing)
directory in this repository is a working `docs`-type config —
`./scripts/examples.sh --landing` runs the full pipeline against it.

## Commands

The full flag reference lives in [Commands](commands.md#pagers-landing).
