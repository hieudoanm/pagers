---
title: Scripts
order: 9
---

# Scripts

`scripts/` holds the installer and crawler used by this project. All are
POSIX-ish bash with `set -euo pipefail`.

## `install-local.sh`

Run after cloning. Builds `pagers-cli` + `pagers-mcp` in release mode and
installs both binaries into `$PREFIX/bin` (default `~/.local/bin`):

```bash
git clone https://github.com/hieudoanm/pagers && cd pagers
./scripts/install-local.sh
PREFIX=/usr/local ./scripts/install-local.sh   # system-wide
```

Requires `cargo`. Prints a PATH hint when the prefix isn't on `PATH`.

## `install.sh`

The curl-pipe installer — downloads the source tarball and delegates to
`install-local.sh`:

```bash
curl -fsSL https://raw.githubusercontent.com/hieudoanm/pagers/main/scripts/install.sh | sh
```

| Variable      | Default                               | Meaning             |
| ------------- | ------------------------------------- | ------------------- |
| `PAGERS_REPO` | `https://github.com/hieudoanm/pagers` | repository to fetch |
| `PAGERS_REF`  | `main`                                | branch or tag       |
| `PREFIX`      | `~/.local`                            | install prefix      |

Requires `curl`, `tar` and `cargo`.

## `examples.sh` — Wikipedia crawler + demo site

Crawls Wikipedia articles into `examples/wiki/<lang>/` and builds them into a
demo pagers site at `examples/demo/`. Two independent flags:

```bash
./scripts/examples.sh --crawl            # crawl + build demo site
./scripts/examples.sh --serve            # build existing files + serve :8080
./scripts/examples.sh --crawl --serve    # crawl + build + serve
MAX_PAGES=10 DELAY=2 ./scripts/examples.sh --crawl
```

| Flag       | Effect                                            |
| ---------- | ------------------------------------------------- |
| `--crawl`  | BFS crawl from `Rust_(programming_language)` seed |
| `--serve`  | serve the built site (implies a build)            |
| `--port N` | serve port (default `8080`)                       |

| Variable    | Default                                                    |
| ----------- | ---------------------------------------------------------- |
| `MAX_PAGES` | `100` **new** downloads per `--crawl` run                  |
| `DELAY`     | `1` (seconds between requests — keep >=1 for Wikipedia)    |
| `PORT`      | `8080`                                                     |
| `PAGERS`    | `pagers` on PATH -> `target/release/pagers` -> `cargo run` |

Details worth knowing:

- The crawl is **incremental**: `.seen` persists `url -> file` pairs, so
  already-downloaded articles are reused for link expansion and `MAX_PAGES` only
  caps new downloads. Failed URLs land in `.failed` and are retried on the next
  `--crawl`.
- `--serve` alone never touches the network — it builds whatever is already in
  `examples/wiki/`.
- Articles are grouped into per-language subdirectories — `en.wikipedia.org` ->
  `examples/wiki/en/`.
- Filenames come from the article title via `pagers wiki -o <dir>/`
  (`Rust (programming language)` -> `rust_programming_language.md`).
- Internal links are saved as local `.md` filenames with the source URL in the
  link title, so the built demo links between pages cleanly.
- Non-article namespaces (`File:`, `Category:`, `Template:`, `Special:`,
  `Talk:`, ...) are skipped; conversion errors print pagers' own message (rate
  limit, 404, ...) with extra backoff on rate limits.
