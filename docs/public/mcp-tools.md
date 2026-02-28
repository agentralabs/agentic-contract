---
status: stable
title: MCP Tools
description: Complete reference for AgenticContract MCP tools
---

# MCP Tools Reference

AgenticContract exposes 22 core tools and 16 invention tools via the Model Context Protocol.

## Contract Tools

### `contract_create`

Create a new contract between agents or agent and user.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Contract name |
| `parties` | string[] | Yes | List of party identifiers |
| `description` | string | No | Contract description |

Returns: Contract ID and creation timestamp.

### `contract_sign`

Sign a contract to indicate acceptance.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contract_id` | string | Yes | Contract to sign |
| `signer` | string | Yes | Identity of the signer |

Returns: Signature confirmation.

### `contract_verify`

Verify a contract's validity and signature chain.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contract_id` | string | Yes | Contract to verify |

Returns: Validity status and signature details.

### `contract_list`

List contracts with optional status filter.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter: active, expired, all |

Returns: Array of contract summaries.

### `contract_get`

Get a specific contract by ID.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contract_id` | string | Yes | Contract ID |

Returns: Full contract details.

## Policy Tools

### `policy_add`

Add a policy rule governing agent behavior.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | string | Yes | Human-readable policy name |
| `scope` | string | Yes | `global`, `session`, or `agent` |
| `action` | string | Yes | `allow`, `deny`, `require_approval`, or `audit_only` |
| `description` | string | No | Detailed policy description |
| `tags` | string[] | No | Categorization tags |
| `conditions` | string[] | No | Prerequisite condition labels |

Returns: Policy ID.

### `policy_check`

Check if an action is allowed under current policies.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | Yes | Description of the intended action |
| `scope` | string | No | Filter to specific scope |

Returns: Policy action (`allow`, `deny`, `require_approval`, `audit_only`) and matching policy details.

### `policy_list`

List active policies with optional scope filter.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `scope` | string | No | Filter: `global`, `session`, `agent` |

Returns: Array of policy objects with ID, label, scope, action, and tags.

## Risk Limit Tools

### `risk_limit_set`

Set a risk limit threshold for a resource or action.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | string | Yes | Limit name |
| `max_value` | number | Yes | Maximum allowed value |
| `limit_type` | string | Yes | `rate`, `threshold`, `budget`, or `count` |
| `window_secs` | number | No | Time window in seconds (for rate limits) |

Returns: Risk limit ID.

### `risk_limit_check`

Check if an action would exceed risk limits.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | string | Yes | Limit to check against |
| `amount` | number | Yes | Proposed usage amount |

Returns: Whether the limit would be exceeded, current usage, remaining capacity.

### `risk_limit_list`

List all risk limits with current values.

Returns: Array of risk limits with ID, label, type, current value, max value, and usage ratio.

## Approval Tools

### `approval_request`

Request approval for a controlled action.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `rule_id` | string | Yes | Approval rule that triggered this |
| `action_description` | string | Yes | What the agent wants to do |
| `requestor` | string | Yes | Who is requesting |

Returns: Approval request ID with pending status.

### `approval_decide`

Approve or deny a pending approval request.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `request_id` | string | Yes | Request to decide |
| `decision` | string | Yes | `approve` or `deny` |
| `decider` | string | Yes | Who is deciding |
| `reason` | string | Yes | Justification |

Returns: Decision ID and updated request status.

### `approval_list`

List approval requests with optional status filter.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter: `pending`, `approved`, `denied`, `expired` |

Returns: Array of approval requests.

## Condition Tools

### `condition_add`

Add a conditional execution rule.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | string | Yes | Condition name |
| `condition_type` | string | Yes | `threshold`, `time_based`, `dependency`, or `custom` |
| `expression` | string | Yes | Condition expression |

Returns: Condition ID.

### `condition_evaluate`

Evaluate whether conditions are met for an action.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `condition_id` | string | Yes | Condition to evaluate |

Returns: Whether the condition is met, with status and last evaluation result.

## Obligation Tools

### `obligation_add`

Add an obligation that an agent must fulfill.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | string | Yes | Obligation name |
| `description` | string | Yes | Detailed description |
| `assignee` | string | Yes | Responsible agent/user |
| `deadline` | string | No | ISO 8601 deadline |

Returns: Obligation ID.

### `obligation_check`

Check the status of obligations.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `assignee` | string | No | Filter by assignee |
| `status` | string | No | Filter: `pending`, `fulfilled`, `overdue`, `waived` |

Returns: Array of obligations, with overdue items flagged.

## Violation Tools

### `violation_report`

Report a contract or policy violation.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `description` | string | Yes | What happened |
| `severity` | string | Yes | `info`, `warning`, `critical`, or `fatal` |
| `actor` | string | Yes | Who committed the violation |
| `policy_id` | string | No | Related policy ID |
| `context` | object | No | Additional JSON context |

Returns: Violation ID with timestamp.

### `violation_list`

List recorded violations with optional severity filter.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `severity` | string | No | Minimum severity filter |
| `actor` | string | No | Filter by actor |

Returns: Array of violation records.

## Context and Stats Tools

### `contract_context_log`

Log the intent and context behind a contract action.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `intent` | string | Yes | Why the action is being taken |
| `topic` | string | No | Category (e.g., "compliance", "risk") |
| `decision` | string | No | What was concluded |

Returns: Confirmation of logged context.

### `contract_stats`

Get summary statistics for the contract store.

Returns: Counts for all entity types, file size, last modified timestamp.

## Error Handling

All tool errors are returned as MCP content with `isError: true`:

```json
{
  "content": [{"type": "text", "text": "Error: Policy not found"}],
  "isError": true
}
```

Unknown tools return JSON-RPC error code `-32803` (TOOL_NOT_FOUND).
