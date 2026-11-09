---
title: Deploy
order: 10
---

# Deploy

`pagers build` emits a plain directory of HTML, CSS, JS and assets — there
is nothing to run at request time. Deploy `dist/` anywhere static files
are served.

## GitHub Pages (recommended)

This repo ships a ready-made workflow at
[`.github/workflows/ci.yaml`](https://github.com/hieudoanm/pagers/blob/main/.github/workflows/ci.yaml):
it runs fmt/clippy/tests, builds `pagers` from source, builds your docs
with that binary, and deploys the result on every push to `main`.

To use it in your own repo:

1. Copy `.github/workflows/ci.yaml` into your repo.
2. Enable **Settings → Pages → Source: GitHub Actions**.
3. Push — the workflow stages `README.md` + `docs/` into `.page/`, runs
   `pagers build .page --output _site`, and deploys `_site/`.

For a project site (`https://<user>.github.io/<repo>/`), set `basePath` to
the repo name so every URL is prefixed correctly:

```yaml
---
title: My Docs
basePath: /my-repo
---
```

For a user/organisation site (`https://<user>.github.io/`), `basePath`
defaults to `/` — no configuration needed.

## Other static hosts

| Host            | Notes                                                        |
| --------------- | ------------------------------------------------------------ |
| Cloudflare Pages| Connect the repo; build command `pagers build docs`, output `dist` |
| Netlify         | Same: build `pagers build docs`, publish `dist`              |
| Vercel          | Framework preset "Other"; output directory `dist`            |
| S3 / CDN        | `aws s3 sync dist/ s3://bucket --delete`                     |
| Any web server  | `pagers build docs && cp -r dist/* /var/www/`                |

## Generated metadata files

Every build emits a standard set of site metadata files at the output
root, and copies PNG icons (`favicon-32.png`, `favicon-64.png`,
`apple-touch-icon.png`, `logo.png`) under `assets/`. **A same-named file
in your content directory always wins** — drop `robots.txt`,
`keybase.txt` or any other file into `docs/` and the generator skips its
own version, so you can ship custom or real-key versions.

### Search engines

- **`robots.txt`** — `User-agent: *` / `Allow: /`, plus a `Sitemap:` line
  pointing at `sitemap.xml` when `url` is configured. Override it to block
  paths or specific crawlers.
- **`sitemap.xml`** — a `<urlset>` listing every published page URL with
  `<lastmod>` taken from each source file's mtime (and `/` when the
  landing page is generated). With `url` set, `<loc>` entries are
  absolute (`https://example.com/docs/guide/`); without it they are
  base-path-relative. Drafts are excluded automatically.

### Browser & platform metadata

- **`favicon.ico`** — a real ICO container wrapping the embedded 32px
  PNG mark, for browsers that still prefer `.ico`. Every page `<head>`
  links the full icon set: `favicon.svg`, `favicon.ico`,
  `favicon-32.png`, `apple-touch-icon.png`, plus `site.webmanifest` and
  a `theme-color` meta tag.
- **`site.webmanifest`** — PWA/install metadata: `name`, `lang`,
  `start_url`/`scope` (the site root), display mode, theme/background
  colors, and the four PNG icons. Makes the site installable and gives
  Android/desktop browsers proper icons.
- **`.well-known/security.txt`** — [RFC 9116](https://www.rfc-editor.org/rfc/rfc9116)
  security contact file. `Contact:` comes from the `contact` site field
  (a `mailto:` or URL), falling back to `url`; `Expires:` is one year
  from build time; `Canonical:` and `Preferred-Languages:` are filled in
  when `url`/`language` are set.
- **`browserconfig.xml`** — legacy IE/Edge tile metadata pointing at the
  PNG icons. Harmless on modern browsers; kept for completeness.
- **`opensearch.xml`** — OpenSearch descriptor so browsers can offer
  "search this site". The `{searchTerms}` template currently points at
  the site root — `pagers` has no built-in search yet, so this is a stub
  until one exists.

### Identity & verification

- **`assetlinks.json`** — Android App Links / Digital Asset Links
  statement, generated as an empty `[]`. To activate, ship a real file
  (e.g. `docs/.well-known/assetlinks.json` is **not** the right path —
  it must be at the domain root, so place `assetlinks.json` in the
  content root) with your app's `package_name` and signing-cert
  fingerprints. (Android also accepts it under `/.well-known/` — to use
  that path, put the file in a `.well-known/` folder in your content
  dir.)
- **`apple-app-site-association`** — iOS universal-links/webcredentials
  statement with empty app lists. iOS requires this file at
  `/.well-known/apple-app-site-association`, so place a real one in a
  `.well-known/` folder in your content dir — a root-level file is
  ignored by iOS.
- **`keybase.txt` / `pgpkey.txt`** — stubs for Keybase/PGP identity
  proofs. Set `keybase:` and `pgp:` (fingerprint) in site config to fill
  them in, or replace the files with your real signed proof / armored
  public key.

### AI & LLM discoverability

- **`llms.txt`** — an [llms.txt](https://llmstxt.org)-style index for
  LLM agents: site title, URL, and every page grouped by top-level
  section with absolute links when `url` is set.
- **`ai.txt`** — a short plain-language policy note pointing agents at
  `robots.txt`, `sitemap.xml` and `llms.txt`. Not a formal standard, but
  a stable declaration point.

The `url` site field drives the absolute URLs in `sitemap.xml`,
`site.webmanifest`, `security.txt`, `llms.txt` and the icon links — the
CI workflow sets it automatically from the repository name.

Because pages publish at trailing-slash URLs (`/guide/` →
`guide/index.html`), **no rewrite rules are needed** on any of them.

## `basePath` quick reference

| Deployment URL                    | `basePath`    |
| --------------------------------- | ------------- |
| `https://<user>.github.io/`       | `/` (default) |
| `https://<user>.github.io/docs/`  | `/docs`       |
| `https://example.com/`            | `/`           |
| `https://example.com/manual/`     | `/manual`     |

`basePath` is applied to every generated URL — internal links, asset URLs,
the graph page — both in the HTML and by the `pagers serve` preview server.

## Checking before you deploy

```bash
pagers serve ./docs --port 8080   # build + local server, basePath-aware
```

The server strips `basePath` before resolving files and redirects
`/guide` → `/guide/`, so what you see locally matches production.
