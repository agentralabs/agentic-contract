---
status: stable
title: Installation
description: How to install AgenticContract
---

# Installation

## Quick Install (Recommended)

```bash
curl -fsSL https://agentralabs.tech/install/contract | bash
```

This auto-installer:
1. Detects your platform (macOS/Linux, x86_64/aarch64)
2. Downloads the `agentic-contract-mcp` binary from GitHub releases
3. Scans for MCP-compatible clients (Claude Desktop, Cursor, Windsurf, VS Code, Claude Code CLI)
4. Writes the MCP server configuration to detected clients
5. Prints a summary of what was installed and where

### Install Profiles

The installer supports three profiles:

```bash
# Desktop (default): Configures GUI-based MCP clients
curl -fsSL https://agentralabs.tech/install/contract | bash -s -- --profile desktop

# Terminal: Configures Claude Code CLI
curl -fsSL https://agentralabs.tech/install/contract | bash -s -- --profile terminal

# Server: Configures for remote deployment with AGENTIC_TOKEN auth
curl -fsSL https://agentralabs.tech/install/contract | bash -s -- --profile server
```

| Profile | Clients Configured | Auth Required |
|---------|-------------------|---------------|
| desktop | Claude Desktop, Cursor, Windsurf, VS Code | No |
| terminal | Claude Code CLI (`~/.claude.json`) | No |
| server | Manual MCP JSON output | Yes (`AGENTIC_TOKEN`) |

## Python SDK

```bash
pip install agentic-contract
```

The Python SDK wraps the `acon` CLI binary via subprocess. It requires the CLI to be installed separately.

```python
from agentic_contract import ContractEngine

engine = ContractEngine()
engine.add_policy("No deploys on Friday", scope="global", action="deny")
result = engine.check_policy("deploy to production")
print(result)  # "deny"
```

**Requirements**: Python 3.8+

## Rust CLI

```bash
cargo install agentic-contract-cli
```

This installs the `acon` binary:

```bash
acon stats
acon policy add "No unauthorized access" --scope global --action deny
acon policy list
acon limit set "API calls" --max 1000 --type rate --window 3600
acon serve  # Start MCP server
```

**Requirements**: Rust 1.75+

## MCP Server

Install the MCP binary directly:

```bash
cargo install agentic-contract-mcp
```

### Claude Desktop Configuration

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

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

### Cursor Configuration

Add to `.cursor/mcp.json` in your project:

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

### VS Code Configuration

Add to `.vscode/mcp.json`:

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

### Claude Code CLI

Add to `~/.claude.json`:

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

### Verifying the Connection

After configuration, restart your MCP client. The server should appear in the tools list. Run any tool to verify:

```
> contract_stats
Policies: 0, Risk Limits: 0, Approvals: 0, ...
```

## Build from Source

```bash
git clone https://github.com/agentralabs/agentic-contract.git
cd agentic-contract
cargo build --release
```

Binaries are placed in `target/release/`:
- `agentic-contract-mcp` — MCP server
- `acon` — CLI tool

Run tests:

```bash
cargo test --workspace
```

Run benchmarks:

```bash
cd crates/agentic-contract
cargo bench
```

## Package Registry Links

| Package | Registry | Install Command |
|---------|----------|----------------|
| `agentic-contract` | PyPI | `pip install agentic-contract` |
| `agentic-contract-cli` | crates.io | `cargo install agentic-contract-cli` |
| `agentic-contract-mcp` | crates.io | `cargo install agentic-contract-mcp` |
| `@agenticamem/contract` | npm | `npm install @agenticamem/contract` |

## Requirements

| Component | Minimum Version |
|-----------|----------------|
| macOS | 12.0+ (Monterey) |
| Linux | glibc 2.31+ |
| Python | 3.8+ |
| Rust | 1.75+ |
| Node.js | 18+ (for npm package) |

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ACON_PATH` | Path to the `.acon` contract file | `~/.agentic/contract.acon` |
| `AGENTIC_TOKEN` | Auth token for server profile | (none) |

## Troubleshooting

**Binary not found after install**

Ensure `~/.local/bin` is in your `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Add this to your `~/.zshrc` or `~/.bashrc` for persistence.

**Permission denied during install**

The installer does not require root. If you encounter permission issues, check that `~/.local/bin` exists and is writable:

```bash
mkdir -p ~/.local/bin
```

**MCP server not appearing in Claude Desktop**

1. Verify the config JSON is valid: `python3 -m json.tool < ~/Library/Application\ Support/Claude/claude_desktop_config.json`
2. Ensure the binary path is correct: `which agentic-contract-mcp`
3. Restart Claude Desktop completely (quit and reopen)

**Contract file locked**

If you see "file locked" errors, another process may be using the file. Check for running `acon` or `agentic-contract-mcp` processes:

```bash
ps aux | grep agentic-contract
```

**Checksum mismatch on load**

The `.acon` file may be corrupted. The BLAKE3 checksum protects against partial writes. If you see this error, restore from a backup or create a new contract file.
