#!/usr/bin/env bash
# Reproduce .github/workflows/ci.yaml locally.
#
#   ./scripts/ci.sh              # PR flow: fmt → clippy → tests → release build
#   ./scripts/ci.sh --deploy     # push-to-master flow: checks + stage .page + build _site
#
# The GitHub-only steps (configure-pages, upload-pages-artifact, deploy-pages)
# have no local equivalent; --deploy stops after producing _site/.
set -euo pipefail
cd "$(dirname "$0")/.."

DEPLOY=0
for arg in "$@"; do
  case "$arg" in
    --deploy|-d) DEPLOY=1 ;;
    *) echo "unknown flag: $arg" >&2; exit 2 ;;
  esac
done

echo "==> cargo fmt --check"
cargo fmt --check

echo "==> cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> cargo test --workspace"
cargo test --workspace

echo "==> cargo build --release -p pagers-cli"
cargo build --release -p pagers-cli

if [[ "$DEPLOY" -eq 1 ]]; then
  echo "==> build landing site (packages/landing → out/)"
  rm -rf _site
  pnpm -C packages/landing install --frozen-lockfile
  pnpm -C packages/landing build
  cp -R packages/landing/out _site
  echo "==> done — _site/ is ready for Pages upload (configure-pages/deploy-pages are CI-only)"
else
  echo "==> all checks passed"
fi
