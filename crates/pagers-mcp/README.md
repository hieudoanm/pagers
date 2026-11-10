# pagers-mcp

`pagers-mcp` — a stdio [Model Context Protocol](https://modelcontextprotocol.io)
server (built on `rmcp`) that exposes pagers' operations as tools so MCP
clients can build sites and inspect content programmatically.

Part of the [pagers](../../README.md) workspace; see
[MCP server](../../packages/landing/docs/mcp.md) for client configuration.

## Tools

| Tool | Purpose |
| --- | --- |
| `pagers_build` | Build a site (`content_dir`, `output_dir`) → page/asset counts |
| `pagers_list_pages` | Discover pages → title, URL, source, draft/order/label |
| `pagers_render_markdown` | Render a Markdown string → HTML, outline, title, edges |
| `pagers_site_graph` | Scan a directory → nodes/edges/stats JSON |
| `pagers_site_config` | Resolved site config (title, basePath, theme, footer) |

Tool names carry the `pagers_` prefix to avoid collisions with other MCP
servers. All results are pretty-printed JSON; failures return
`ErrorData::internal_error`.

## Usage

```bash
cargo run -p pagers-mcp           # stdio server — point an MCP client at it
```

```json
{ "mcpServers": { "pagers": { "command": "pagers-mcp" } } }
```
