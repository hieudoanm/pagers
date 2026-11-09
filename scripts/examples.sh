#!/usr/bin/env bash
# Demo pipeline: optionally crawl Wikipedia articles into
# examples/wiki/<lang>/, then build them into a demo pagers site at
# examples/demo/ and optionally serve it.
#
#   ./scripts/examples.sh --crawl            # crawl + build
#   ./scripts/examples.sh --serve            # build existing files + serve :8080
#   ./scripts/examples.sh --crawl --serve    # crawl + build + serve
#   MAX_PAGES=10 DELAY=2 ./scripts/examples.sh --crawl
#
# Env: MAX_PAGES (default 100 new pages per crawl), DELAY (seconds between
#      requests, default 1), PAGERS (binary override), PORT (default 8080)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_DIR="$ROOT/examples/wiki"
SITE_DIR="$ROOT/examples/demo"
MAX_PAGES="${MAX_PAGES:-100}"
DELAY="${DELAY:-1}"
PORT="${PORT:-8080}"
CRAWL=0
SERVE=0

while [ $# -gt 0 ]; do
  case "$1" in
    --crawl|-c)    CRAWL=1; shift ;;
    --serve|-s)    SERVE=1; shift ;;
    --port)        PORT="$2"; shift 2 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

log() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

# Resolve the pagers binary: PATH → repo release build → cargo run.
if [ -n "${PAGERS:-}" ]; then
  PAGERS_BIN="$PAGERS"
elif command -v pagers >/dev/null 2>&1; then
  PAGERS_BIN="pagers"
elif [ -x "$ROOT/target/release/pagers" ]; then
  PAGERS_BIN="$ROOT/target/release/pagers"
else
  PAGERS_BIN="cargo run --quiet --manifest-path $ROOT/Cargo.toml -p pagers-cli --"
fi

# --- Crawl: incremental BFS over internal /wiki/ links. Already-downloaded
# articles (recorded in .seen as "url<TAB>path") are reused for link
# expansion; MAX_PAGES caps NEW downloads per run.
crawl() {
  local start_url="https://en.wikipedia.org/wiki/Rust_(programming_language)"
  local pending="$WIKI_DIR/.pending" seen="$WIKI_DIR/.seen"
  local claimed="$WIKI_DIR/.claimed" failed="$WIKI_DIR/.failed"
  mkdir -p "$WIKI_DIR"
  : > "$pending"; : > "$claimed"; : > "$failed"
  if [ -f "$seen" ]; then
    awk -F'\t' 'NF > 1' "$seen" > "$seen.tmp" && mv "$seen.tmp" "$seen" || : > "$seen"
  else
    : > "$seen"
  fi
  echo "$start_url" >> "$pending"

  # seen_path <url> → prints the recorded relative .md path, if any.
  seen_path() { awk -F'\t' -v u="$1" '$1 == u { p = $2 } END { print p }' "$seen"; }

  # harvest_links <file> → appends discovered wiki URLs to $pending.
  harvest_links() {
    { perl -ne 'while (/"(https:\/\/[a-z-]+\.wikipedia\.org\/wiki\/(?:\\.|[^"])*)"/g) { print "$1\n" }' "$1"
      perl -ne 'while (/\((https:\/\/[a-z-]+\.wikipedia\.org\/wiki\/(?:\\.|[^)"\s\\])+)/g) { print "$1\n" }' "$1"
    } | perl -pe 's/\\(.)/$1/g; s/&amp;/&/g; s/[?#].*$//' | sort -u >> "$pending" || true
  }

  local count=0 reused=0 nfailed=0
  log "Crawling up to $MAX_PAGES new articles → $WIKI_DIR"
  while [ -s "$pending" ] && [ "$count" -lt "$MAX_PAGES" ]; do
    local url="$(sed -n '1p' "$pending")"
    tail -n +2 "$pending" > "$pending.tmp" && mv "$pending.tmp" "$pending"

    local title="${url##*/wiki/}"
    case "$title" in
      File:*|Image:*|Category:*|Wikipedia:*|Template:*|Portal:*|Special:*|Help:*|Talk:*|Draft:*) continue ;;
    esac

    # Reuse: a file recorded in .seen still on disk → harvest links, no download.
    local rel="$(seen_path "$url")"
    if [ -n "$rel" ] && [ -f "$WIKI_DIR/$rel" ]; then
      harvest_links "$WIKI_DIR/$rel"
      reused=$((reused + 1))
      continue
    fi

    # Claimed already this run → skip duplicate queue entries.
    grep -Fxq "$url" "$claimed" && continue
    echo "$url" >> "$claimed"

    local host="${url#*://}" host="${host%%/*}"
    local lang="${host%%.*}"
    [ -n "$lang" ] || lang="wiki"
    local langdir="$WIKI_DIR/$lang"
    mkdir -p "$langdir"

    local stamp="$langdir/.stamp"; touch "$stamp"
    local err="$(mktemp)"
    if $PAGERS_BIN wiki convert "$url" -o "$langdir/" --quiet 2>"$err"; then
      rm -f "$err"
      count=$((count + 1))
      local new_md="$(find "$langdir" -maxdepth 1 -name '*.md' -newer "$stamp" | head -1)"
      rm -f "$stamp"
      if [ -n "$new_md" ]; then
        printf '%s\t%s/%s\n' "$url" "$lang" "$(basename "$new_md")" >> "$seen"
        harvest_links "$new_md"
      fi
    else
      rm -f "$stamp"
      nfailed=$((nfailed + 1))
      # Surface pagers' own error (rate limit, 404, timeout, …) instead of
      # a bare "failed". Rate limits get extra backoff before the next try.
      local reason="$(head -1 "$err" | sed 's/^pagers wiki: //')"
      rm -f "$err"
      echo "$url" >> "$failed"
      printf '\033[1;33mwarn:\033[0m %s — %s\n' "$url" "${reason:-unknown error}" >&2
      case "$reason" in
        *rate*limit*|*429*|*timed*out*|*timeout*) sleep 5 ;;
      esac
    fi
    [ "$count" -lt "$MAX_PAGES" ] && [ "$DELAY" != "0" ] && sleep "$DELAY"
  done
  rm -f "$pending" "$pending.tmp"
  [ "$count" -gt 0 ] || [ "$reused" -gt 0 ] || die "no articles available"
  log "Crawled $count new article(s), reused $reused ($nfailed failed)"
  [ "$nfailed" -eq 0 ] || log "failed URLs listed in $failed"
}

# --- Build the demo site. Internal .md links resolve to clean URLs, so
# the wiki set becomes a browsable site. No index.md → pagers generates a
# landing page from the nav.
build() {
  log "Building demo site → $SITE_DIR"
  "$PAGERS_BIN" build "$WIKI_DIR" --output "$SITE_DIR"
}

serve_site() {
  log "Serving on http://localhost:$PORT (Ctrl+C to stop)"
  exec "$PAGERS_BIN" serve "$WIKI_DIR" --output "$SITE_DIR" --port "$PORT"
}

[ "$CRAWL" -eq 1 ] && crawl
build
[ "$SERVE" -eq 1 ] && serve_site
log "Done. Serve with: $PAGERS_BIN serve $WIKI_DIR --output $SITE_DIR --port $PORT"
