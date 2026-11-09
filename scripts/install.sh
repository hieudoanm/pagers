#!/usr/bin/env bash
# Install pagers via curl: downloads the source and builds it locally.
#
#   curl -fsSL https://raw.githubusercontent.com/hieudoanm/pagers/main/scripts/install.sh | sh
#
# Options (environment variables):
#   PAGERS_REF    git ref to install (default: main; use a tag like v0.1.0)
#   PAGERS_REPO   repository URL (default: https://github.com/hieudoanm/pagers)
#   PREFIX        install prefix (default: ~/.local)
#
# Requires curl, tar and a Rust toolchain (https://rustup.rs).
set -euo pipefail

REPO="${PAGERS_REPO:-https://github.com/hieudoanm/pagers}"
REF="${PAGERS_REF:-main}"
PREFIX="${PREFIX:-$HOME/.local}"

log() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

for tool in curl tar cargo; do
  command -v "$tool" >/dev/null 2>&1 ||
    die "$tool not found. Install Rust first: https://rustup.rs"
done

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# GitHub archives work for both github.com and codeload-style mirrors.
BASE="${REPO%.git}"
case "$BASE" in
  https://github.com/*) ARCHIVE="$BASE/archive/refs/heads/$REF.tar.gz" ;;
  *) die "unsupported repository URL: $REPO (expected https://github.com/...)" ;;
esac
# Tags live under refs/tags, not refs/heads — probe both.
if ! curl -fsSL -o /dev/null "$ARCHIVE" 2>/dev/null; then
  ARCHIVE="${BASE}/archive/refs/tags/$REF.tar.gz"
fi

log "Downloading $REPO @ $REF"
curl -fsSL "$ARCHIVE" -o "$TMP/src.tar.gz" ||
  die "download failed: $ARCHIVE"
tar -xzf "$TMP/src.tar.gz" -C "$TMP"
SRC="$(find "$TMP" -mindepth 1 -maxdepth 1 -type d | head -1)"
[ -n "$SRC" ] || die "could not unpack archive"

log "Building and installing (this can take a few minutes)"
PREFIX="$PREFIX" bash "$SRC/scripts/install-local.sh"
