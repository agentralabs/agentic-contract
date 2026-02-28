---
status: stable
title: Integration Guide
description: Integrating AgenticContract with LLMs, frameworks, and the Agentra ecosystem
---

# Integration Guide

AgenticContract can be used through four interfaces: MCP server (for LLM agents), Rust library (for direct embedding), Python SDK (for scripting), and FFI (for C/C++ interop). This guide covers integration patterns for each.

## MCP Integration

The MCP server is the primary integration path for LLM agents. Any MCP-compatible client (Claude Desktop, Cursor, Windsurf, VS Code, Claude Code CLI) can use the contract engine.

### Setup

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

### Available Tools

The MCP server exposes 22 core tools organized by entity type:

| Category | Tools | Count |
|----------|-------|-------|
| Contract | create, sign, verify, list, get | 5 |
| Policy | add, check, list | 3 |
| Risk Limit | set, check, list | 3 |
| Approval | request, decide, list | 3 |
| Condition | add, evaluate | 2 |
| Obligation | add, check | 2 |
| Violation | report, list | 2 |
| Stats/Context | stats, context_log | 2 |

Plus 16 invention tools for advanced use cases (see [MCP Tools Reference](mcp-tools.md)).

### Common Workflow: Pre-Action Policy Check

The most common pattern is checking policies before taking an action:

```
1. Agent wants to perform an action (e.g., "deploy to production")
2. Call policy_check with the action description
3. Response:
   - "allow"    -> proceed with the action
   - "deny"     -> stop, optionally call violation_report
   - "require_approval" -> call approval_request, wait for approval_decide
   - "audit_only" -> proceed, the action is logged
```

### Common Workflow: Risk-Aware Operations

For actions with quantitative risk (API calls, spending, retries):

```
1. Call risk_limit_check with the label and proposed amount
2. If within limits: proceed, then call risk_limit_set to update usage
3. If would exceed: call violation_report with severity "critical"
```

### Common Workflow: Obligation Monitoring

For agents with recurring responsibilities:

```
1. At setup: call obligation_add with deadline and assignee
2. Periodically: call obligation_check to detect overdue items
3. On completion: call obligation_add to mark fulfilled
4. On overdue: call violation_report
```

### Server Authentication

In server profile, the MCP server requires an `AGENTIC_TOKEN` environment variable:

```bash
AGENTIC_TOKEN="your-secret-token" agentic-contract-mcp serve
```

The server checks this token in the `initialize` handshake. Without it, the server starts in local-only mode.

### Error Handling

MCP tool errors follow the MCP Quality Standard:

- **Tool execution errors**: Returned as `isError: true` in the result (NOT as JSON-RPC errors)
- **Unknown tool**: JSON-RPC error code `-32803` (TOOL_NOT_FOUND)
- **Protocol errors**: Standard JSON-RPC error codes (`-32601` for unknown method)

Example error response:

```json
{
  "content": [
    {
      "type": "text",
      "text": "Error: Policy not found: invalid-id"
    }
  ],
  "isError": true
}
```

## Rust API

Add `agentic-contract` to your `Cargo.toml`:

```toml
[dependencies]
agentic-contract = "0.1"
```

### Basic Usage

```rust
use agentic_contract::{
    ContractEngine, Policy, PolicyScope, PolicyAction,
    RiskLimit, LimitType, ViolationSeverity,
};

// Create or open engine
let mut engine = ContractEngine::new();

// Add a policy
let policy = Policy::new(
    "No production deploys without approval",
    PolicyScope::Global,
    PolicyAction::RequireApproval
);
engine.add_policy(policy);

// Check before acting
let result = engine.check_policy("deploy to production", None);
match result {
    PolicyAction::Deny => {
        engine.report_violation(
            "Attempted denied action",
            ViolationSeverity::Warning,
            "my-agent"
        );
    }
    PolicyAction::RequireApproval => {
        // Enter approval workflow
    }
    _ => {
        // Proceed with action
    }
}

// Save to disk
engine.save()?;
```

### Risk Limit Pattern

```rust
let limit = RiskLimit::new("API budget", LimitType::Budget, 100.0);
let limit_id = engine.add_risk_limit(limit);

// Before each API call
if engine.check_risk_limit("API budget", 0.05).is_some() {
    engine.report_violation(
        "API budget would be exceeded",
        ViolationSeverity::Critical,
        "my-agent"
    );
} else {
    engine.increment_risk_limit(limit_id, 0.05)?;
    // Make the API call
}
```

### Persistence

```rust
// Open existing or create new
let mut engine = ContractEngine::open("~/.agentic/contract.acon")
    .unwrap_or_else(|_| ContractEngine::new());

// ... modify engine ...

engine.save()?;
```

## Python SDK

```bash
pip install agentic-contract
```

### Basic Usage

```python
from agentic_contract import ContractEngine

engine = ContractEngine()

# Policy enforcement
engine.add_policy("No deploys on Friday", scope="global", action="deny")
result = engine.check_policy("deploy to production")
if result == "deny":
    print("Action blocked by policy")

# Risk limits
engine.set_risk_limit("API calls", max=1000, type="rate")
limit = engine.check_risk_limit("API calls", amount=1)
print(f"Remaining: {limit['remaining']}")

# Obligations
engine.add_obligation(
    "Weekly report",
    description="Submit compliance summary",
    assignee="my-agent",
    deadline="2026-03-07T17:00:00Z"
)

# Statistics
stats = engine.stats()
print(f"Policies: {stats['policies']}, Violations: {stats['violations']}")
```

### With Claude (Anthropic)

```python
import anthropic
from agentic_contract import ContractEngine

client = anthropic.Anthropic()
engine = ContractEngine()

# Add governance constraints
engine.add_policy("No data deletion", scope="global", action="deny")
engine.set_risk_limit("API spend", max=50.0, type="budget")

# Before each tool use, check policy
def guarded_tool_call(tool_name, args):
    result = engine.check_policy(f"{tool_name}: {args}")
    if result == "deny":
        return {"error": "Action denied by policy"}
    return execute_tool(tool_name, args)
```

### With OpenAI GPT

```python
import openai
from agentic_contract import ContractEngine

client = openai.OpenAI()
engine = ContractEngine()

engine.add_policy("Read-only mode", scope="session", action="deny")
engine.set_risk_limit("Token usage", max=100000, type="count")
```

### With LangChain

```python
from agentic_contract import ContractEngine

engine = ContractEngine()

class ContractGuard:
    """LangChain callback that checks policies before tool execution."""

    def __init__(self, engine):
        self.engine = engine

    def on_tool_start(self, tool_name, tool_input):
        result = self.engine.check_policy(f"{tool_name}: {tool_input}")
        if result == "deny":
            raise ValueError(f"Blocked by policy: {tool_name}")
```

## FFI Integration

The FFI crate exposes a C-compatible API for embedding the contract engine in any language.

### C Header

```c
#include "agentic_contract.h"

AconHandle* handle = acon_open("/path/to/contract.acon");
if (!handle) {
    int err = acon_last_error();
    // Handle error
}

int result = acon_policy_add(handle, "No deploys", "global", "deny");
acon_save(handle);
acon_close(handle);
```

See the [FFI Reference](ffi-reference.md) for the complete C API.

## Cross-Sister Integration

AgenticContract integrates with the Agentra ecosystem through shared SDK traits and cross-references.

### AgenticIdentity

Approval decisions link to identity trust grants via receipts. When an approval is granted, the identity system records who approved it and under what authority.

```
approval_decide -> receipt signed by identity -> trust grant verification
```

### AgenticMemory

Contract decisions are stored as memory nodes for long-term recall. This enables agents to reason about past governance decisions.

```
violation_report -> memory_add(type="decision", content="Budget exceeded")
```

### AgenticTime

All contract events are timestamped with the time sister's temporal tracking. This enables temporal queries like "show all violations in the last 24 hours."

### AgenticVision

Visual observations can trigger policy evaluations. For example, a screenshot showing sensitive data can trigger a "data exposure" policy check.

### AgenticCodebase

Code analysis results can feed into condition evaluations. For example, a condition might require code review approval before deployment.

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ACON_PATH` | Contract file location | `~/.agentic/contract.acon` |
| `AGENTIC_TOKEN` | Auth token (server mode) | (none) |
| `RUST_LOG` | Log level (trace/debug/info) | `info` |

### File Location

The default contract file is `~/.agentic/contract.acon`. Override with:

```bash
# CLI
acon --path /custom/path.acon stats

# Environment variable
ACON_PATH=/custom/path.acon agentic-contract-mcp serve

# Python
engine = ContractEngine(path="/custom/path.acon")
```
