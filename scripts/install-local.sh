#!/usr/bin/env bash
# Install pagers from a local clone: builds the release binaries and
# installs them into ~/.local/bin.
#
#   git clone https://github.com/hieudoanm/pagers
#   cd pagers && ./scripts/install-local.sh
#
# Requires a Rust toolchain (https://rustup.rs). Override the install
# prefix with PREFIX=/usr/local ./scripts/install-local.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREFIX="${PREFIX:-$HOME/.local}"
BIN_DIR="$PREFIX/bin"

log() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null 2>&1 ||
  die "cargo not found. Install Rust first: https://rustup.rs"

mkdir -p "$BIN_DIR"

log "Building pagers (release)"
cargo build --release --manifest-path "$ROOT/Cargo.toml" -p pagers-cli -p pagers-mcp

for bin in pagers pagers-mcp; do
  src="$ROOT/target/release/$bin"
  [ -x "$src" ] || die "expected binary $src not found"
  install -m 0755 "$src" "$BIN_DIR/$bin"
  log "installed $BIN_DIR/$bin"
done

case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) log "note: $BIN_DIR is not on your PATH — add: export PATH=\"$BIN_DIR:\$PATH\"" ;;
esac

log "done: $("$BIN_DIR/pagers" --version 2>/dev/null || echo pagers)"
