---
status: stable
---

# CLI Reference

AgenticContract ships a CLI binary `acon` for managing policies, risk limits, approvals, obligations, and violations from the terminal.

## Global Options

| Option | Env Var | Default | Description |
|--------|---------|---------|-------------|
| `--path` | `ACON_PATH` | `~/.agentic/contract.acon` | Path to the `.acon` contract file |

## Subcommands

### `acon policy add`

Add a policy rule governing agent behavior.

```bash
acon policy add "No production deploys on Friday" --scope global --action deny
```

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--scope` | No | `global` | Policy scope: `global`, `session`, `agent` |
| `--action` | No | `deny` | Policy action: `allow`, `deny`, `require_approval`, `audit_only` |

### `acon policy check`

Check if an action is allowed under current policies.

```bash
acon policy check "deploy to production"
```

### `acon policy list`

List active policies.

```bash
acon policy list
```

### `acon limit set`

Set a risk limit threshold.

```bash
acon limit set "API calls per hour" --max 1000 --type rate
```

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--max` | Yes | — | Maximum allowed value |
| `--type` | No | `threshold` | Limit type: `rate`, `threshold`, `budget`, `count` |

### `acon limit check`

Check current risk limit usage.

```bash
acon limit check "API calls per hour" --usage 500
```

### `acon limit list`

List all risk limits with current values.

### `acon approval rule`

Add an approval rule.

```bash
acon approval rule "Production deploy" --pattern "deploy*prod*"
```

### `acon approval request`

Request approval for a controlled action.

### `acon approval decide`

Approve or deny a pending request.

```bash
acon approval decide <request-id> --approved
```

### `acon approval list`

List approval requests with optional status filter.

### `acon obligation add`

Add an obligation with a deadline.

```bash
acon obligation add "Submit compliance report" --deadline "2026-03-01T00:00:00Z"
```

### `acon obligation check`

Check obligation status.

### `acon obligation fulfill`

Mark an obligation as fulfilled.

### `acon obligation list`

List all obligations.

### `acon violation report`

Report a contract or policy violation.

```bash
acon violation report "Exceeded API rate limit" --severity warning
```

### `acon violation list`

List recorded violations with optional severity filter.

```bash
acon violation list --severity critical
```

### `acon stats`

Display contract statistics.

### `acon info`

Display build and version information.

### `acon install`

Configure MCP client integration.

### `acon serve`

Start the MCP server (delegates to `agentic-contract-mcp`).
