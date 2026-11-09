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

Every build emits the standard site metadata files at the root:
`robots.txt`, `sitemap.xml`, `humans.txt`, `favicon.ico`,
`site.webmanifest`, `.well-known/security.txt`, `browserconfig.xml`,
`opensearch.xml`, `assetlinks.json`, `apple-app-site-association`,
`keybase.txt`, `pgpkey.txt`, `llms.txt` and `ai.txt` — plus PNG icons
(`favicon-32/64.png`, `apple-touch-icon.png`, `logo.png`) under
`assets/`. A same-named file in your content directory overrides the
generated one, so `keybase.txt`/`pgpkey.txt` stubs can be replaced with
real keys. Set the `url` site field — the CI workflow does this
automatically — and the sitemap/manifest/security files get absolute
URLs.

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
