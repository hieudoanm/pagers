---
title: Commands
order: 2
---

# Commands

`pagers` has seven top-level commands. Run `pagers <command> --help` for the
authoritative flag list.

## `pagers build [content]`

Build a site. `[content]` defaults to `docs/`, output defaults to `dist/`.

```bash
pagers build ./docs
pagers build ./docs --output ./public
```

Every `.md`/`.markdown` file becomes a trailing-slash HTML page; every other
file is copied through as an asset. The build also writes `graph.html`
(page-link graph), `graph.json` and `graph.dot` data at the site root.

## `pagers init [dir]`

Scaffold a new site: creates `dir/docs/index.md` and an example page. Defaults
to `./pagers-site` when `dir` is omitted.

```bash
pagers init my-site
```

## `pagers serve [content]`

Build once, then serve the output on localhost — handy for checking clean URLs,
`basePath` handling and the theme toggle.

```bash
pagers serve ./docs --port 8080
```

The server understands `basePath` (it strips the prefix before resolving files)
and redirects `/guide` → `/guide/`.

## `pagers graph [content]`

Scan a content directory and write the link graph **without** building the site.
Emits three files next to each other:

- `graph.html` — the interactive viewer (same one embedded in builds)
- `graph.json` — nodes/edges as data
- `graph.dot` — Graphviz DOT

```bash
pagers graph ./docs
```

## `pagers wiki convert <url>`

Fetch a Wikipedia article through the MediaWiki REST API and print clean
Markdown. See [Wikipedia](../content/wiki.md) for the full pipeline.

```bash
pagers wiki convert "https://en.wikipedia.org/wiki/Rust_(programming_language)"
pagers wiki convert "<url>" -o rust.md
pagers wiki convert "<url>" --format json
pagers wiki convert "<url>" --include-references
pagers wiki convert "<url>" --quiet        # scripting: no progress output
```

Flags: `-o/--output`, `--format markdown|json`, `--include-references`,
`-q/--quiet`.

## `pagers wiki info <url>`

Show article metadata (title, canonical URL, language, revision id and
timestamp) without converting. `--format json` for machine-readable output.

```bash
pagers wiki info "https://en.wikipedia.org/wiki/Rust_(programming_language)"
```

## `pagers landing`

Generate a single-page static site from one `landify.yaml` — no build step, no
JavaScript, theme derived from eight colors. Subcommands:

| Command                   | Does                                                     |
| ------------------------- | -------------------------------------------------------- |
| `pagers landing new`      | Scaffold an annotated `landify.yaml` (`-t` picks a type) |
| `pagers landing validate` | Strictly validate the YAML schema                        |
| `pagers landing build`    | Render the site files described below                    |
| `pagers landing themes`   | List the 64 built-in theme presets                       |
| `pagers landing preview`  | Serve the output directory over HTTP                     |

```bash
pagers landing new --type product
pagers landing validate -f landify.yaml
pagers landing build -f landify.yaml -o public/index.html --theme ocean
pagers landing preview --dir public --port 8080
```

Twelve page types: `product` (default), `waitlist`, `event`, `download`, `app`,
`pricing`, `portfolio`, `docs`, `faq`, `team`, `status`, `linktree`. `--theme`
overrides the YAML `theme:` section with a built-in preset; see
`pagers landing themes` for all 64 names.

Page types, theming, the `landify.yaml` schema, and the full build output
(`index.html`, `styles.css`, `og/og.svg`, and the site metadata files) are
covered in [Landing pages](landing.md).

## Exit codes

All commands exit non-zero on failure with a `pagers <area>:`-prefixed error on
stderr — safe to use in scripts and CI.
