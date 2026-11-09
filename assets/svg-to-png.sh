#!/usr/bin/env bash
# Convert every SVG in assets/branding/svg/ to a sibling PNG in
# assets/branding/png/ using librsvg (rsvg-convert).
#
#   ./assets/svg-to-png.sh              # convert branding/svg → branding/png
#   ./assets/svg-to-png.sh <base_dir>   # convert <base_dir>/svg → <base_dir>/png
#
# Requires: brew install librsvg
set -euo pipefail

BASE_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/branding" && pwd)}"
SVG_DIR="$BASE_DIR/svg"
PNG_DIR="$BASE_DIR/png"

if ! command -v rsvg-convert >/dev/null 2>&1; then
  echo "rsvg-convert not found. Install librsvg: brew install librsvg" >&2
  exit 1
fi

[ -d "$SVG_DIR" ] || { echo "no svg dir: $SVG_DIR" >&2; exit 1; }
mkdir -p "$PNG_DIR"

shopt -s nullglob
for svg in "$SVG_DIR"/*.svg; do
  png="$PNG_DIR/$(basename "${svg%.svg}").png"
  echo "Converting $svg -> $png"
  rsvg-convert -o "$png" "$svg"
done

echo "Done."
