# pagers-mcp — Agent Guide

Back to the [workspace README](../../README.md) · [MCP docs](../../docs/mcp.md).

## Conventions

- `rmcp` server macros (`#[tool_router]`, `#[tool]`) — tool method names
  **must** keep the `pagers_` prefix.
- Parameters are `Option<String>` with `docs`/`dist` defaults matching the
  CLI; document defaults in the `#[tool(description = …)]` string.
- Keep tool results as JSON strings (`serde_json::to_string_pretty`) — MCP
  clients consume them uniformly.
- Don't add long-running or interactive operations (`serve`, `init`) —
  the request/response model doesn't fit them.
- This crate depends on `pagers-build`/`pagers-site`/`pagers-markdown`/
  `pagers-graph` — never duplicate pipeline logic here.

## Verification

```bash
cargo build -p pagers-mcp
cargo clippy -p pagers-mcp
# smoke test over stdio: send initialize → tools/list → tools/call
```
