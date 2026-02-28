---
status: stable
title: Quickstart
description: Get started with AgenticContract in 5 minutes
---

# Quickstart

## 1. Install

```bash
curl -fsSL https://agentralabs.tech/install/contract | bash
```

## 2. Add a Policy

```bash
acon policy add "No production deploys on Friday" --scope global --action deny
```

## 3. Check a Policy

```bash
acon policy check "deploy" --scope global
# Output: Decision: Deny
```

## 4. Set a Risk Limit

```bash
acon limit set "API calls per hour" --max 1000 --type rate
```

## 5. Report a Violation

```bash
acon violation report "Rate limit exceeded" --severity warning --actor agent_1
```

## 6. View Stats

```bash
acon stats
```

## Using with MCP

The MCP server exposes 21 tools that AI agents can use directly:

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
