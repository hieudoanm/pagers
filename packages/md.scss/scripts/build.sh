#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SASS="$ROOT/node_modules/.bin/sass"
DIST="$ROOT/dist"

clean() {
  rm -rf "$DIST"
}

build() {
  mkdir -p "$DIST"

  "$SASS" --source-map src/md.scss "$DIST/md.css"
  "$SASS" --source-map --style=compressed src/md.scss "$DIST/md.min.css"
}

cd "$ROOT"

if [[ "${1:-}" == "clean" ]]; then
  clean
else
  clean
  build
fi