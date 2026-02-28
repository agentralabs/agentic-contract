---
status: stable
---

# Runtime Install Sync

## Install Profiles

AgenticContract supports three install profiles:

| Profile | Auth Required | MCP Config | Use Case |
|---------|--------------|------------|----------|
| `desktop` | No | Claude Desktop, Cursor, Windsurf | Local development |
| `terminal` | No | Claude Code CLI | Terminal-based workflows |
| `server` | Yes (`AGENTIC_TOKEN`) | Manual | Remote deployment |

## Installation Methods

### One-liner (recommended)

```bash
# Default (desktop profile)
curl -fsSL https://agentralabs.tech/install/contract | bash

# Specific profile
curl -fsSL https://agentralabs.tech/install/contract | bash -s -- --profile=server
```

### Package managers

```bash
pip install agentic-contract
cargo install agentic-contract-cli
npm install @agenticamem/contract
```

### From source

```bash
git clone https://github.com/agentralabs/agentic-contract.git
cd agentic-contract
cargo install --path crates/agentic-contract-cli
cargo install --path crates/agentic-contract-mcp
```

## Runtime Sync

The contract file (`.acon`) is the single source of truth. All access surfaces (MCP, CLI, FFI, Python) read from and write to the same file.

### File Location Resolution

1. Explicit path via `--path` flag or `ACON_PATH` env var
2. Project-local: `.agentra/<project>.acontract`
3. Default: `~/.agentic/contract.acon`

### Concurrent Access

The `.acon` file uses a sidecar lock file (`.acon.lock`) for concurrent access safety. Multiple MCP servers or CLI instances can safely operate on the same contract file.
