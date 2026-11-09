# pagers-build — Agent Guide

Back to the [workspace README](../../README.md) · [Architecture](../../docs/architecture.md).

## Conventions

- One public function: `build(content_dir, output_dir) -> Result<Stats, String>`.
  Don't grow the API surface — callers (CLI, MCP) only need build-or-fail.
- Errors are `String`s with `area: message` style prefixes, matching CLI
  output conventions; keep messages actionable.
- The pipeline must stay **deterministic and offline** — no network, no
  timestamps in output that would break reproducible builds.
- File writes go through a small `write_file` helper (create parent dirs);
  don't scatter `fs::write` calls.
- Depends only on `pagers-*` crates — no new external deps without need.

## Verification

```bash
cargo build -p pagers-build
cargo test --workspace   # integration coverage via pagers-cli tests
```
