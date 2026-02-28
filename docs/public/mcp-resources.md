---
status: stable
title: MCP Resources
description: MCP resource endpoints for AgenticContract
---

# MCP Resources

AgenticContract exposes 12 resources via the MCP protocol using the `acon://` URI scheme. Resources provide read-only access to contract data.

## Resource List

| URI | Name | MIME Type | Description |
|-----|------|-----------|-------------|
| `acon://policies` | All Policies | application/json | Complete list of policies |
| `acon://policies/global` | Global Policies | application/json | Policies with global scope |
| `acon://policies/session` | Session Policies | application/json | Session-scoped policies |
| `acon://risk-limits` | Risk Limits | application/json | All limit definitions and values |
| `acon://risk-limits/exceeded` | Exceeded Limits | application/json | Limits currently over threshold |
| `acon://approvals` | Approval Requests | application/json | All approval requests |
| `acon://approvals/pending` | Pending Approvals | application/json | Requests awaiting decision |
| `acon://obligations` | Obligations | application/json | All tracked obligations |
| `acon://obligations/overdue` | Overdue Obligations | application/json | Obligations past deadline |
| `acon://violations` | Violations | application/json | All recorded violations |
| `acon://violations/critical` | Critical Violations | application/json | Critical/fatal violations |
| `acon://stats` | Contract Statistics | application/json | Summary statistics |

## URI Scheme

All resources use the `acon://` prefix. This follows the Agentra convention where each sister has its own URI scheme (`amem://` for memory, `avis://` for vision, `atmp://` for time, `acb://` for codebase, `aid://` for identity).

## Concrete Resources

### `acon://policies`

Returns all policies regardless of scope or status.

```json
{
  "policies": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "label": "No production deploys without approval",
      "scope": "global",
      "action": "require_approval",
      "status": "active",
      "tags": ["deployment", "production"],
      "created_at": "2026-02-27T10:30:00Z"
    }
  ],
  "count": 1
}
```

### `acon://policies/global`

Returns only policies with `scope: global`.

### `acon://policies/session`

Returns only policies with `scope: session`.

### `acon://risk-limits`

Returns all risk limits with current usage values.

```json
{
  "limits": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "label": "API budget",
      "limit_type": "budget",
      "current_value": 42.50,
      "max_value": 100.0,
      "usage_ratio": 0.425,
      "remaining": 57.50,
      "window_secs": null
    }
  ],
  "count": 1
}
```

### `acon://risk-limits/exceeded`

Returns only limits where `current_value >= max_value`.

### `acon://approvals`

Returns all approval requests with their current status.

```json
{
  "requests": [
    {
      "id": "770e8400-e29b-41d4-a716-446655440002",
      "rule_id": "880e8400-e29b-41d4-a716-446655440003",
      "action_description": "deploy to production",
      "requestor": "deploy-agent",
      "status": "pending",
      "created_at": "2026-02-27T14:00:00Z"
    }
  ],
  "count": 1
}
```

### `acon://approvals/pending`

Returns only requests with `status: pending`.

### `acon://obligations`

Returns all obligations with deadline and status.

```json
{
  "obligations": [
    {
      "id": "990e8400-e29b-41d4-a716-446655440004",
      "label": "Weekly compliance report",
      "assignee": "compliance-agent",
      "deadline": "2026-03-07T17:00:00Z",
      "status": "pending"
    }
  ],
  "count": 1
}
```

### `acon://obligations/overdue`

Returns only obligations where the deadline has passed and status is still `pending`.

### `acon://violations`

Returns all recorded violations.

```json
{
  "violations": [
    {
      "id": "aa0e8400-e29b-41d4-a716-446655440005",
      "description": "API budget exceeded by $12.50",
      "severity": "critical",
      "actor": "data-agent-3",
      "detected_at": "2026-02-27T15:30:00Z"
    }
  ],
  "count": 1
}
```

### `acon://violations/critical`

Returns only violations with severity `critical` or `fatal`.

### `acon://stats`

Returns summary statistics for the entire contract store.

```json
{
  "policies": 12,
  "risk_limits": 5,
  "approval_rules": 3,
  "approval_requests": 8,
  "conditions": 4,
  "obligations": 6,
  "violations": 9,
  "total_entities": 47,
  "file_path": "/Users/user/.agentic/contract.acon",
  "file_size_bytes": 24576,
  "last_modified": "2026-02-27T14:15:00Z"
}
```

## Cross-Sister Resources

AgenticContract resources can be referenced by other Agentra sisters:

- **AgenticIdentity**: Links approval decisions to identity trust grants via `aid://` resources
- **AgenticMemory**: Stores contract decisions as memory nodes accessible via `amem://` resources
- **AgenticTime**: Contract events carry temporal metadata compatible with `atmp://` resources
- **AgenticVision**: Visual observations can reference contract violations via `avis://` resources
- **AgenticCodebase**: Code analysis can reference contract conditions via `acb://` resources

## Reading Resources Programmatically

### MCP Client

Resources are read through the standard MCP `resources/read` method:

```json
{
  "method": "resources/read",
  "params": {
    "uri": "acon://stats"
  }
}
```

### CLI

```bash
acon stats          # Equivalent to acon://stats
acon policy list    # Equivalent to acon://policies
acon violation list # Equivalent to acon://violations
```
