---
status: stable
---

# Configuration

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ACON_PATH` | `~/.agentic/contract.acon` | Path to the contract file |
| `AGENTIC_TOKEN` | — | Authentication token for server mode |
| `AGENTIC_TOKEN_FILE` | — | Path to file containing auth token |
| `AGENTRA_AUTH_TOKEN_FILE` | — | Alternative token file path |
| `AGENTRA_INSTALL_PROFILE` | `desktop` | Install profile: `desktop`, `terminal`, `server` |

## Runtime Modes

### Desktop Mode (default)

MCP server runs locally, configured for Claude Desktop, Cursor, Windsurf, and other MCP clients. No authentication required.

### Terminal Mode

MCP server runs for CLI-based MCP clients like Claude Code. No authentication required.

### Server Mode

Remote deployment with mandatory token-based authentication. Set `AGENTIC_TOKEN` or provide a token file via `AGENTIC_TOKEN_FILE`.

## MCP Configuration

Add to your MCP client configuration:

```json
{
  "mcpServers": {
    "agentic-contract": {
      "command": "agentic-contract-mcp",
      "args": ["serve"]
    }
  }
}
```

## File Location

The default contract file location is `~/.agentic/contract.acon`. Override with:

- CLI: `acon --path /custom/path.acon`
- Environment: `export ACON_PATH=/custom/path.acon`
- MCP: Set `ACON_PATH` before starting the server
