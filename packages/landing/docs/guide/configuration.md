---
title: Configuration
order: 3
---

# Configuration

There is no config file format to learn — the directory layout is the
configuration, and a small amount of frontmatter or a flat `pagers.yaml` covers
the rest.

## Frontmatter

Every page may start with a `---` fenced block. All fields are optional; the
title falls back to the first `#` heading, then to a humanized file name.

```yaml
---
title: Getting Started
description: Learn the basics in five minutes.
order: 1
label: Start here
draft: false
---
```

| Field         | Effect                                                 |
| ------------- | ------------------------------------------------------ |
| `title`       | Page `<title>` and nav label                           |
| `description` | Reserved for metadata; not yet rendered                |
| `order`       | Sort position among siblings (lower first)             |
| `label`       | Navigation label, when it should differ from the title |
| `draft`       | `true` excludes the page from the build                |

## Site fields

The **root** `index.md`/`README.md` additionally accepts site-wide fields:

```yaml
---
title: My Docs
language: en
basePath: /docs
theme: dark
footer: MIT License
url: https://example.com/docs
contact: mailto:hi@example.com
keybase: alice
pgp: 'ABCD 1234 EF56 7890'
---
```

- `language` — `<html lang>`.
- `contact` — security contact for `.well-known/security.txt` (falls back to
  `url`).
- `keybase` / `pgp` — fill the `keybase.txt` / `pgpkey.txt` stubs (or ship real
  files in the content dir to override).
- `url` — absolute site root (e.g. `https://example.com/docs`), including the
  `basePath`. Used for sitemap and robots.txt URLs.
- `basePath` — prefix prepended to every generated URL, for hosting under a
  sub-path (e.g. GitHub Pages project sites → `/pagers`).
- `theme` — initial theme, `light` (default) or `dark`.
- `footer` — text rendered in the page footer.

## `pagers.yaml`

The same flat `key: value` shape works as a standalone file in the content root
— used when the index page has no frontmatter (or no site fields):

```yaml
title: My Docs
basePath: /docs
theme: dark
footer: MIT License
url: https://example.com/docs
contact: mailto:hi@example.com
```

Index-page frontmatter wins over `pagers.yaml`, which wins over defaults (title
derived from the directory name).

## URL rules

| Source                           | URL               | Output                     |
| -------------------------------- | ----------------- | -------------------------- |
| `index.md`, `README.md` (root)   | `/`               | `index.html`               |
| `guide/index.md`                 | `/guide/`         | `guide/index.html`         |
| `guide/install.md`               | `/guide/install/` | `guide/install/index.html` |
| `assets/logo.png` (non-Markdown) | unchanged         | copied to output root      |

Trailing-slash URLs mean the site works on any static host with zero rewrite
rules.
