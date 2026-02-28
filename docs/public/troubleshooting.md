---
status: stable
---

# Troubleshooting

## Installation Issues

### Binary not found after install

Ensure `~/.local/bin` is in your `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Add to your shell profile (`~/.bashrc`, `~/.zshrc`) for persistence.

### Permission denied during install

The installer needs write access to `~/.local/bin`. Create it manually if needed:

```bash
mkdir -p ~/.local/bin
```

### Cargo install fails

Ensure you have Rust 1.75+ installed:

```bash
rustup update stable
cargo install agentic-contract-cli
```

## MCP Server Issues

### Server not appearing in MCP client

1. Verify the binary is installed: `which agentic-contract-mcp`
2. Check your MCP configuration file for correct entry
3. Restart your MCP client (Claude Desktop, Cursor, etc.)

### Server authentication error

In server mode, `AGENTIC_TOKEN` must be set:

```bash
export AGENTIC_TOKEN="$(openssl rand -hex 32)"
agentic-contract-mcp serve
```

### Connection timeout

The MCP server uses stdio transport with Content-Length framing. Ensure no other process is consuming stdin/stdout.

## Common Errors

### `ContractError::NotFound`

The specified entity (policy, risk limit, approval, etc.) does not exist. Check the ID is correct.

### `ContractError::PolicyViolation`

An action was denied by an active policy. Use `policy_list` to see active policies and `policy_check` to test specific actions.

### `ContractError::RiskLimitExceeded`

A risk limit threshold would be exceeded. Use `risk_limit_list` to check current values.

### `ContractError::ApprovalRequired`

An action requires approval before proceeding. Use `approval_request` to submit a request.

### File format errors

If the `.acon` file is corrupted, delete it and start fresh:

```bash
rm ~/.agentic/contract.acon
acon stats  # Creates a new empty contract
```

## Performance Tips

- The `.acon` binary format is optimized for fast reads
- Policy evaluation is sub-millisecond for typical workloads
- For large policy sets (10K+), consider scoping policies to reduce evaluation time
- Use `contract_stats` to monitor entity counts
